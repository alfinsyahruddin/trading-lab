import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import { readable } from 'svelte/store';
import BacktestDetailPage from '../../../src/routes/dashboard/backtests/[id]/+page.svelte';
import * as api from '$lib/api';
import * as session from '$lib/helpers/session';
import { toast } from '$lib/helpers/toast.svelte';
import type { BacktestJob, TradingStrategy } from '$lib/types';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$app/stores', () => ({
	page: readable({
		params: { id: 'job-failed-1' },
		url: new URL('http://localhost:3000/dashboard/backtests/job-failed-1')
	})
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		getBacktest: vi.fn(),
		getTradingStrategy: vi.fn(),
		rerunBacktest: vi.fn()
	};
});

vi.mock('lightweight-charts', () => ({
	createChart: vi.fn(() => ({
		addSeries: vi.fn(() => ({ setData: vi.fn() })),
		timeScale: vi.fn(() => ({ fitContent: vi.fn() })),
		applyOptions: vi.fn(),
		subscribeCrosshairMove: vi.fn(),
		unsubscribeCrosshairMove: vi.fn(),
		remove: vi.fn()
	})),
	ColorType: { Solid: 'solid' },
	BaselineSeries: 'Baseline'
}));

Object.defineProperty(window, 'matchMedia', {
	writable: true,
	value: vi.fn().mockImplementation((query) => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: vi.fn(),
		removeListener: vi.fn(),
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn()
	}))
});

describe('BacktestDetailPage - Run Again', () => {
	const mockFailedJob: BacktestJob = {
		id: 'job-failed-1',
		user_id: 'u-1',
		strategy_id: 'strat-1',
		strategy_name: 'Breakout Strategy',
		name: 'Failed Test 2025',
		year: 2025,
		initial_cash: 10000000,
		max_holding_stocks: 5,
		max_stocks: 12,
		backtest_duration_months: 12,
		buy_fee_percentage: 0.15,
		sell_fee_percentage: 0.25,
		is_public: false,
		status: 'FAILED',
		error_message: 'Sectors Screener API error (400): Invalid syntax',
		result: null,
		portfolio_history: null,
		most_traded: null,
		top_gainers: null,
		top_losers: null,
		trade_history: null,
		created_at: '2026-08-30T00:00:00Z',
		updated_at: '2026-08-30T00:00:00Z'
	};

	const mockStrategy: TradingStrategy = {
		id: 'strat-1',
		user_id: 'u-1',
		name: 'Breakout Strategy',
		description: 'Desc',
		tp_percentage: 10,
		sl_percentage: 5,
		risk_reward_ratio: 2,
		max_holding_period_days: 30,
		rules: [],
		created_at: '2026-08-30T00:00:00Z',
		updated_at: '2026-08-30T00:00:00Z'
	};

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('valid-token');
		vi.spyOn(session, 'getUser').mockReturnValue({
			id: 'u-1',
			name: 'Test User',
			email: 'test@example.com',
			role: 'MEMBER',
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		});
		vi.mocked(api.getTradingStrategy).mockResolvedValue(mockStrategy);
	});

	it('renders Run Again buttons when backtest has failed and user is owner', async () => {
		vi.mocked(api.getBacktest).mockResolvedValue(mockFailedJob);
		render(BacktestDetailPage);

		expect(await screen.findByText('Backtest Failed')).toBeInTheDocument();
		expect(
			screen.getByText('Sectors Screener API error (400): Invalid syntax')
		).toBeInTheDocument();

		const runAgainButtons = screen.getAllByText('Run Again');
		expect(runAgainButtons.length).toBeGreaterThanOrEqual(1);
	});

	it('clicking Run Again invokes rerunBacktest and notifies user', async () => {
		vi.mocked(api.getBacktest).mockResolvedValue(mockFailedJob);
		vi.mocked(api.rerunBacktest).mockResolvedValue({
			...mockFailedJob,
			status: 'PENDING',
			error_message: null
		});
		const toastSpy = vi.spyOn(toast, 'success');

		render(BacktestDetailPage);

		const runAgainBtn = (await screen.findAllByText('Run Again'))[0].closest('button')!;
		expect(runAgainBtn).not.toBeNull();
		await fireEvent.click(runAgainBtn);

		expect(api.rerunBacktest).toHaveBeenCalledWith('valid-token', 'job-failed-1');
		expect(toastSpy).toHaveBeenCalledWith('Backtest "Failed Test 2025" restarted.');
	});

	it('renders All-Time High and All-Time Low in winning and losing trades cards', async () => {
		const mockDoneJob: BacktestJob = {
			id: 'job-done-1',
			user_id: 'u-1',
			strategy_id: 'strat-1',
			strategy_name: 'Breakout Strategy',
			name: 'Successful Test 2025',
			year: 2025,
			initial_cash: 10_000_000,
			max_holding_stocks: 5,
			max_stocks: 12,
			backtest_duration_months: 12,
			buy_fee_percentage: 0.15,
			sell_fee_percentage: 0.25,
			is_public: false,
			status: 'DONE',
			error_message: null,
			result: {
				available_cash: 11_000_000,
				trades_processed: 10,
				net_pnl: 1_500_000,
				net_pnl_percentage: 15,
				gross_pnl: 1_600_000,
				gross_pnl_percentage: 16,
				win_rate: 60,
				profit_factor: 2.1,
				wins: 6,
				losses: 4,
				sharpe_ratio: 1.5,
				max_profit: 500_000,
				max_profit_percentage: 12.5,
				max_loss: -250_000,
				max_loss_percentage: -6.2,
				avg_profit: 300_000,
				avg_profit_percentage: 8.5,
				avg_loss: -150_000,
				avg_loss_percentage: -3.8,
				avg_hold_time_days: 14,
				total_fees: 100_000,
				avg_win_hold_days: 12,
				avg_loss_hold_days: 16,
				portfolio_volatility: 10.2,
				ai_summary: null
			},
			portfolio_history: [
				{ date: '2025-01-02', net_value: 9_600_000, gross_value: 9_650_000 },
				{ date: '2025-02-15', net_value: 12_500_000, gross_value: 12_550_000 },
				{ date: '2025-03-30', net_value: 11_500_000, gross_value: 11_550_000 }
			],
			most_traded: [],
			top_gainers: [],
			top_losers: [],
			trade_history: [],
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		};

		vi.mocked(api.getBacktest).mockResolvedValue(mockDoneJob);
		render(BacktestDetailPage);

		expect(await screen.findByText('All-Time High')).toBeInTheDocument();
		expect(screen.getByText('Rp2.500.000 (+25.00%)')).toBeInTheDocument();

		expect(screen.getByText('All-Time Low')).toBeInTheDocument();
		expect(screen.getByText('Rp400.000 (-4.00%)')).toBeInTheDocument();
	});

	it('renders compact trade history cards with trade metrics and handles click to open modal', async () => {
		const mockJobWithTrades: BacktestJob = {
			id: 'job-trades-1',
			user_id: 'u-1',
			strategy_id: 'strat-1',
			strategy_name: 'Breakout Strategy',
			name: 'Test with Trades 2025',
			year: 2025,
			initial_cash: 10_000_000,
			max_holding_stocks: 5,
			max_stocks: 12,
			backtest_duration_months: 12,
			buy_fee_percentage: 0.15,
			sell_fee_percentage: 0.25,
			is_public: false,
			status: 'DONE',
			error_message: null,
			result: {
				available_cash: 11_000_000,
				trades_processed: 1,
				net_pnl: 500_000,
				net_pnl_percentage: 5.5,
				gross_pnl: 520_000,
				gross_pnl_percentage: 5.7,
				win_rate: 100,
				profit_factor: 2.1,
				wins: 1,
				losses: 0,
				sharpe_ratio: 1.5,
				max_profit: 500_000,
				max_profit_percentage: 5.5,
				max_loss: 0,
				max_loss_percentage: 0,
				avg_profit: 500_000,
				avg_profit_percentage: 5.5,
				avg_loss: 0,
				avg_loss_percentage: 0,
				avg_hold_time_days: 17,
				total_fees: 20_000,
				avg_win_hold_days: 17,
				avg_loss_hold_days: 0,
				portfolio_volatility: 10.2,
				ai_summary: null
			},
			portfolio_history: [
				{ date: '2025-01-02', net_value: 10_000_000, gross_value: 10_000_000 },
				{ date: '2025-02-15', net_value: 10_500_000, gross_value: 10_520_000 }
			],
			most_traded: [],
			top_gainers: [],
			top_losers: [],
			trade_history: [
				{
					code: 'BBCA.JK',
					buy_date: '2025-01-15',
					sell_date: '2025-02-01',
					buy_price: 9000,
					sell_price: 9495,
					buy_value: 9000000,
					sell_value: 9495000,
					buy_fee: 13500,
					sell_fee: 23737.5,
					lot: 10,
					pnl: 495000,
					pnl_percentage: 5.5,
					exit_reason: 'TAKE_PROFIT'
				}
			],
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		};

		vi.mocked(api.getBacktest).mockResolvedValue(mockJobWithTrades);
		render(BacktestDetailPage);

		expect(await screen.findByText('Trade history')).toBeInTheDocument();
		expect(screen.getByText('BBCA')).toBeInTheDocument();
		expect(screen.getByText('10 Lot')).toBeInTheDocument();
		expect(screen.getByText('17 Days')).toBeInTheDocument();
		expect(screen.getByText('Take profit')).toBeInTheDocument();

		const tradeCard = screen.getByRole('button', {
			name: /View screening entry details for BBCA/i
		});
		expect(tradeCard).toBeInTheDocument();
		await fireEvent.click(tradeCard);

		// TradeInfoModal should open showing trade details
		expect(
			screen.getByText('Screening criteria & actual metrics evaluated for entry')
		).toBeInTheDocument();
	});
});
