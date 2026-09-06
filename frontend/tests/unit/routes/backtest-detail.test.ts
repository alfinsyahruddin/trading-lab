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
});
