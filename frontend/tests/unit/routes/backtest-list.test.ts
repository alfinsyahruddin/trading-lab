import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import BacktestListPage from '../../../src/routes/dashboard/backtests/+page.svelte';
import * as api from '$lib/api';
import * as session from '$lib/helpers/session';
import { toast } from '$lib/helpers/toast.svelte';
import type { BacktestJob } from '$lib/types';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		listBacktests: vi.fn(),
		rerunBacktest: vi.fn()
	};
});

describe('BacktestListPage - Run Again', () => {
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

	const mockDoneJob: BacktestJob = {
		id: 'job-done-1',
		user_id: 'u-1',
		strategy_id: 'strat-1',
		strategy_name: 'Breakout Strategy',
		name: 'Successful Test 2025',
		year: 2025,
		initial_cash: 10000000,
		max_holding_stocks: 5,
		max_stocks: 12,
		backtest_duration_months: 12,
		buy_fee_percentage: 0.15,
		sell_fee_percentage: 0.25,
		is_public: false,
		status: 'DONE',
		error_message: null,
		result: {
			available_cash: 11000000,
			trades_processed: 10,
			net_pnl: 1000000,
			net_pnl_percentage: 10,
			gross_pnl: 1200000,
			gross_pnl_percentage: 12,
			win_rate: 60,
			profit_factor: 1.8,
			wins: 6,
			losses: 4,
			sharpe_ratio: 1.5,
			max_profit: 300000,
			max_profit_percentage: 3,
			max_loss: -100000,
			max_loss_percentage: -1,
			avg_profit: 200000,
			avg_profit_percentage: 2,
			avg_loss: -75000,
			avg_loss_percentage: -0.75,
			avg_hold_time_days: 14,
			total_fees: 50000,
			avg_win_hold_days: 12,
			avg_loss_hold_days: 16,
			portfolio_volatility: 0.12
		},
		portfolio_history: null,
		most_traded: null,
		top_gainers: null,
		top_losers: null,
		trade_history: null,
		created_at: '2026-08-30T00:00:00Z',
		updated_at: '2026-08-30T00:00:00Z'
	};

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('valid-token');
	});

	it('renders Run Again button for failed backtest but not for done backtest', async () => {
		vi.mocked(api.listBacktests).mockResolvedValue([mockFailedJob, mockDoneJob]);
		render(BacktestListPage);

		expect(await screen.findByText('Failed Test 2025')).toBeInTheDocument();
		expect(screen.getByText('Successful Test 2025')).toBeInTheDocument();

		const runAgainButtons = screen.getAllByText('Run Again');
		expect(runAgainButtons).toHaveLength(1);
	});

	it('clicking Run Again invokes rerunBacktest and notifies user', async () => {
		vi.mocked(api.listBacktests).mockResolvedValue([mockFailedJob]);
		vi.mocked(api.rerunBacktest).mockResolvedValue({
			...mockFailedJob,
			status: 'PENDING',
			error_message: null
		});
		const toastSpy = vi.spyOn(toast, 'success');

		render(BacktestListPage);

		const runAgainBtn = (await screen.findByText('Run Again')).closest('button')!;
		expect(runAgainBtn).not.toBeNull();
		await fireEvent.click(runAgainBtn);

		expect(api.rerunBacktest).toHaveBeenCalledWith('valid-token', 'job-failed-1');
		expect(toastSpy).toHaveBeenCalledWith('Backtest "Failed Test 2025" restarted.');
	});
});
