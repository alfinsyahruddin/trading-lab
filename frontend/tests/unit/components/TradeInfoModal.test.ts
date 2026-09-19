import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import TradeInfoModal from '$lib/components/backtest/TradeInfoModal.svelte';
import type { TradeHistoryEntry, TradingStrategy } from '$lib/types';

describe('TradeInfoModal', () => {
	const mockTrade: TradeHistoryEntry = {
		code: 'BBCA.JK',
		company_name: 'PT Bank Central Asia Tbk.',
		pnl: 500000,
		pnl_percentage: 5.5,
		exit_reason: 'TAKE_PROFIT',
		lot: 10,
		buy_price: 9000,
		buy_value: 9000000,
		sell_price: 9495,
		sell_value: 9495000,
		buy_fee: 13500,
		sell_fee: 23737.5,
		buy_date: '2024-01-15',
		sell_date: '2024-02-01',
		query_values: {
			sub_sector: 'Banks',
			market_cap: 753611199412500,
			'pe[2024]': 18.5,
			unrelated_metric: 1234
		}
	};

	const mockStrategy: TradingStrategy = {
		id: 'strat-1',
		user_id: 'user-1',
		name: 'Big Cap Banking Strategy',
		description: 'Screen banks with large market cap',
		tp_percentage: 10,
		sl_percentage: 5,
		risk_reward_ratio: 2,
		max_holding_period_days: 30,
		rules: [
			{
				id: 'group-1',
				connector_to_next: 'AND',
				conditions: [
					{
						id: 'cond-1',
						variable: 'market_cap',
						operator: '>',
						value: '1000000000000',
						connector_to_next: 'AND'
					},
					{
						id: 'cond-2',
						variable: 'sub_sector',
						operator: '=',
						value: 'Banks',
						connector_to_next: null
					}
				]
			},
			{
				id: 'group-2',
				connector_to_next: null,
				conditions: [
					{
						id: 'cond-3',
						variable: 'pe',
						operator: '<',
						value: '25',
						connector_to_next: null
					}
				]
			}
		],
		created_at: '2024-01-01T00:00:00Z',
		updated_at: '2024-01-01T00:00:00Z'
	};

	it('does not render dialog when open is false', () => {
		render(TradeInfoModal, {
			props: {
				open: false,
				trade: mockTrade,
				strategy: mockStrategy,
				year: 2024
			}
		});

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('renders dialog with stock code, company name, entry details, and matched conditions when open is true', () => {
		render(TradeInfoModal, {
			props: {
				open: true,
				trade: mockTrade,
				strategy: mockStrategy,
				year: 2024
			}
		});

		expect(screen.getByRole('dialog')).toBeInTheDocument();
		expect(screen.getByText('BBCA')).toBeInTheDocument();
		expect(screen.getByText('PT Bank Central Asia Tbk.')).toBeInTheDocument();
		expect(screen.getByText('Mon, 15 Jan 2024')).toBeInTheDocument();
		expect(screen.getByText('Thu, 1 Feb 2024')).toBeInTheDocument();
		expect(screen.getByText('Take profit')).toBeInTheDocument();
		expect(screen.getByText('10 Lot')).toBeInTheDocument();
		expect(screen.getByText('17 Days')).toBeInTheDocument();
		expect(screen.getByText('Rp 9.000')).toBeInTheDocument();
		expect(screen.getByText('Rp 9.495')).toBeInTheDocument();
		expect(screen.getByText('Rp 9.000.000')).toBeInTheDocument();
		expect(screen.getByText('Rp 9.495.000')).toBeInTheDocument();
		expect(screen.getByText('+5.50%')).toBeInTheDocument();

		// Check formatted actual market cap
		expect(screen.getByText('Rp 753.61 T')).toBeInTheDocument();
		// Check formatted P/E ratio without trailing zero (18.50 -> 18.5)
		expect(screen.getByText('18.5x')).toBeInTheDocument();
		// Check sub_sector strings (both target rule and actual screened value)
		expect(screen.getAllByText('Banks').length).toBe(2);

		// Check Matched badges
		const matchedBadges = screen.getAllByText('Matched');
		expect(matchedBadges.length).toBeGreaterThanOrEqual(2);

		// Check unmapped metric
		expect(screen.getByText('unrelated_metric')).toBeInTheDocument();
	});

	it('renders raw JSON in collapsible accordion and allows closing modal', async () => {
		const user = userEvent.setup();
		render(TradeInfoModal, {
			props: {
				open: true,
				trade: mockTrade,
				strategy: mockStrategy,
				year: 2024
			}
		});

		expect(screen.getByText(/Raw Screener Data \(JSON\)/)).toBeInTheDocument();
		expect(screen.getByText(/"sub_sector": "Banks"/)).toBeInTheDocument();

		const closeBtn = screen.getByRole('button', { name: 'Close modal' });
		await user.click(closeBtn);

		// When close is clicked, modal is closed and removed from DOM
		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('formats Entry Date as "Mon, 8 Jun 2026" format', () => {
		const trade2026: TradeHistoryEntry = {
			...mockTrade,
			buy_date: '2026-06-08'
		};

		render(TradeInfoModal, {
			props: {
				open: true,
				trade: trade2026,
				strategy: mockStrategy,
				year: 2026
			}
		});

		expect(screen.getByText('Mon, 8 Jun 2026')).toBeInTheDocument();
	});

	it('removes trailing zeros in decimal rules and actual values (e.g., 1.00 -> 1, 0.24 -> 0.24)', () => {
		const tradeWithDecimals: TradeHistoryEntry = {
			...mockTrade,
			query_values: {
				pb: 1.0,
				roe: 0.24,
				custom_score: 5.0
			}
		};

		const strategyWithDecimals: TradingStrategy = {
			...mockStrategy,
			rules: [
				{
					id: 'g-1',
					connector_to_next: null,
					conditions: [
						{
							id: 'c-1',
							variable: 'pb',
							operator: '<=',
							value: '1.00',
							connector_to_next: 'AND'
						},
						{
							id: 'c-2',
							variable: 'roe',
							operator: '>=',
							value: '0.24',
							connector_to_next: null
						}
					]
				}
			]
		};

		render(TradeInfoModal, {
			props: {
				open: true,
				trade: tradeWithDecimals,
				strategy: strategyWithDecimals,
				year: 2024
			}
		});

		// Check 1.00 -> 1 in both target rule and actual PB
		expect(screen.getAllByText('1x').length).toBe(2);
		// Check 0.24 -> 24% in both target rule and actual ROE
		expect(screen.getAllByText('24%').length).toBe(2);
		// Check unmapped metric with trailing zero (5.00 -> 5)
		expect(screen.getByText('5')).toBeInTheDocument();
	});
});
