import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import { readable } from 'svelte/store';
import StrategyDetailPage from '../../../src/routes/dashboard/strategies/[id]/+page.svelte';
import * as api from '$lib/api';
import * as session from '$lib/helpers/session';
import { toast } from '$lib/helpers/toast.svelte';
import type { TradingStrategy, BacktestJob } from '$lib/types';

const mockGoto = vi.fn();
vi.mock('$app/navigation', () => ({
	goto: (...args: unknown[]) => mockGoto(...args)
}));

vi.mock('$app/stores', () => ({
	page: readable({
		params: { id: 'strat-123' },
		url: new URL('http://localhost:3000/dashboard/strategies/strat-123')
	})
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		getTradingStrategy: vi.fn(),
		deleteTradingStrategy: vi.fn(),
		duplicateTradingStrategy: vi.fn(),
		listBacktests: vi.fn()
	};
});

describe('StrategyDetailPage', () => {
	const mockStrategy: TradingStrategy = {
		id: 'strat-123',
		user_id: 'u-owner',
		name: 'Momentum Breakout IDX',
		description: 'High momentum strategy with volume confirmation',
		tp_percentage: 15,
		sl_percentage: 5,
		risk_reward_ratio: 3,
		max_holding_period_days: 20,
		rules: [
			{
				id: 'group-1',
				connector_to_next: 'AND',
				conditions: [
					{
						id: 'cond-1',
						variable: 'close_to_high',
						operator: '>=',
						value: '0.95',
						connector_to_next: 'AND'
					},
					{
						id: 'cond-2',
						variable: 'volume_spike',
						operator: '>',
						value: '2.0',
						connector_to_next: null
					}
				]
			}
		],
		created_at: '2026-08-01T00:00:00Z',
		updated_at: '2026-08-05T00:00:00Z'
	};

	const mockBacktests: BacktestJob[] = [
		{
			id: 'bt-1',
			user_id: 'u-owner',
			strategy_id: 'strat-123',
			strategy_name: 'Momentum Breakout IDX',
			name: 'Momentum 2024 Test',
			year: 2024,
			initial_cash: 50_000_000,
			max_holding_stocks: 5,
			max_stocks: 10,
			backtest_duration_months: 12,
			buy_fee_percentage: 0.15,
			sell_fee_percentage: 0.25,
			is_public: false,
			status: 'DONE',
			error_message: null,
			result: {
				available_cash: 60_000_000,
				trades_processed: 30,
				net_pnl: 10_000_000,
				net_pnl_percentage: 20,
				gross_pnl: 11_000_000,
				gross_pnl_percentage: 22,
				win_rate: 65,
				profit_factor: 2.1,
				wins: 20,
				losses: 10,
				sharpe_ratio: 1.8,
				max_profit: 2_000_000,
				max_profit_percentage: 20,
				max_loss: -500_000,
				max_loss_percentage: -5,
				avg_profit: 1_000_000,
				avg_profit_percentage: 10,
				avg_loss: -250_000,
				avg_loss_percentage: -2.5,
				avg_hold_time_days: 10,
				total_fees: 1_000_000,
				avg_win_hold_days: 12,
				avg_loss_hold_days: 6,
				portfolio_volatility: 12,
				ai_summary: null
			},
			portfolio_history: null,
			most_traded: null,
			top_gainers: null,
			top_losers: null,
			trade_history: null,
			created_at: '2026-08-10T00:00:00Z',
			updated_at: '2026-08-10T00:00:00Z'
		}
	];

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('mock-token');
		vi.spyOn(session, 'getUser').mockReturnValue({
			id: 'u-owner',
			name: 'Strategy Owner',
			email: 'owner@example.com',
			role: 'MEMBER',
			created_at: '2026-01-01T00:00:00Z',
			updated_at: '2026-01-01T00:00:00Z'
		});

		vi.mocked(api.getTradingStrategy).mockResolvedValue(mockStrategy);
		vi.mocked(api.listBacktests).mockResolvedValue(mockBacktests);
	});

	it('loads and renders strategy details, parameters, rules, and related backtests', async () => {
		render(StrategyDetailPage);

		expect(await screen.findByText('Momentum Breakout IDX')).toBeInTheDocument();
		expect(screen.getByText('High momentum strategy with volume confirmation')).toBeInTheDocument();

		// Parameters
		expect(screen.getByText('+15%')).toBeInTheDocument();
		expect(screen.getByText('-5%')).toBeInTheDocument();
		expect(screen.getByText('20 Days')).toBeInTheDocument();

		// Rules
		expect(screen.getByText('Condition Group #1')).toBeInTheDocument();
		expect(screen.getByText('close_to_high')).toBeInTheDocument();
		expect(screen.getByText('volume_spike')).toBeInTheDocument();

		// Related backtests
		expect(screen.getByText('Momentum 2024 Test')).toBeInTheDocument();
		expect(screen.getByText('DONE')).toBeInTheDocument();
	});

	it('shows owner action buttons (Run Backtest, Edit, Delete) when user is the owner', async () => {
		render(StrategyDetailPage);

		expect(await screen.findByText('Momentum Breakout IDX')).toBeInTheDocument();

		expect(screen.getByText('Run Backtest')).toBeInTheDocument();
		expect(screen.getByText('Edit')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /delete strategy/i })).toBeInTheDocument();
	});

	it('hides owner action buttons when user is not the owner', async () => {
		vi.spyOn(session, 'getUser').mockReturnValue({
			id: 'u-different',
			name: 'Other User',
			email: 'other@example.com',
			role: 'MEMBER',
			created_at: '2026-01-01T00:00:00Z',
			updated_at: '2026-01-01T00:00:00Z'
		});

		render(StrategyDetailPage);

		expect(await screen.findByText('Momentum Breakout IDX')).toBeInTheDocument();

		expect(screen.queryByText('Run Backtest')).not.toBeInTheDocument();
		expect(screen.queryByText('Edit')).not.toBeInTheDocument();
		expect(screen.queryByRole('button', { name: /delete strategy/i })).not.toBeInTheDocument();
		// Duplicate should still be visible for all users
		expect(screen.getByRole('button', { name: /duplicate/i })).toBeInTheDocument();
	});

	it('duplicates strategy when duplicate modal is submitted', async () => {
		const clonedStrategy: TradingStrategy = {
			...mockStrategy,
			id: 'strat-cloned-456',
			name: 'Momentum Breakout IDX (Copy)'
		};
		vi.mocked(api.duplicateTradingStrategy).mockResolvedValue(clonedStrategy);
		const toastSuccessSpy = vi.spyOn(toast, 'success');

		render(StrategyDetailPage);

		expect(await screen.findByText('Momentum Breakout IDX')).toBeInTheDocument();

		const duplicateBtn = screen.getByRole('button', { name: /duplicate/i });
		await fireEvent.click(duplicateBtn);

		expect(screen.getByText('Duplicate Trading Strategy')).toBeInTheDocument();

		const submitBtn = screen.getByRole('button', { name: /duplicate strategy/i });
		await fireEvent.click(submitBtn);

		await waitFor(() => {
			expect(api.duplicateTradingStrategy).toHaveBeenCalledWith(
				'mock-token',
				'strat-123',
				'Momentum Breakout IDX (Copy)'
			);
			expect(toastSuccessSpy).toHaveBeenCalledWith(
				'Strategy "Momentum Breakout IDX (Copy)" duplicated successfully.'
			);
			expect(mockGoto).toHaveBeenCalledWith('/dashboard/strategies/strat-cloned-456');
		});
	});

	it('deletes strategy when delete confirmation modal is confirmed', async () => {
		vi.mocked(api.deleteTradingStrategy).mockResolvedValue('Deleted successfully');
		const toastSuccessSpy = vi.spyOn(toast, 'success');

		render(StrategyDetailPage);

		expect(await screen.findByText('Momentum Breakout IDX')).toBeInTheDocument();

		const deleteBtn = screen.getByRole('button', { name: /delete strategy/i });
		await fireEvent.click(deleteBtn);

		expect(screen.getByText('Delete Trading Strategy')).toBeInTheDocument();

		const confirmDeleteBtn = screen.getByRole('button', { name: 'Delete Strategy' });
		await fireEvent.click(confirmDeleteBtn);

		await waitFor(() => {
			expect(api.deleteTradingStrategy).toHaveBeenCalledWith('mock-token', 'strat-123');
			expect(toastSuccessSpy).toHaveBeenCalledWith('Strategy "Momentum Breakout IDX" deleted.');
			expect(mockGoto).toHaveBeenCalledWith('/dashboard/strategies');
		});
	});
});
