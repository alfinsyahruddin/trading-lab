import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import NewBacktestPage from '../../../src/routes/dashboard/backtests/new/+page.svelte';
import * as api from '$lib/api';
import * as session from '$lib/helpers/session';
import { calculateBacktestDateRange } from '$lib/helpers/date';
import type { TradingStrategy } from '$lib/types';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		listTradingStrategies: vi.fn(),
		createBacktest: vi.fn()
	};
});

describe('NewBacktestPage', () => {
	const mockStrategies: TradingStrategy[] = [
		{
			id: 'strat-1',
			user_id: 'u-1',
			name: 'Momentum Breakout',
			description: 'Trend following strategy',
			tp_percentage: 10,
			sl_percentage: 5,
			risk_reward_ratio: 2,
			max_holding_period_days: 30,
			rules: [],
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		}
	];

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('valid-token');
		vi.mocked(api.listTradingStrategies).mockResolvedValue(mockStrategies);
	});

	it('renders Backtest Date label and computes date range based on default year and duration', async () => {
		render(NewBacktestPage);

		// Wait for strategies to load and form to render
		expect(await screen.findByText('Backtest Date')).toBeInTheDocument();

		// Default year is 2025 -> test year 2026, default duration is 12 months (1 Year)
		const expectedDateRange = calculateBacktestDateRange(2025, 12).formatted;
		expect(screen.getByText(expectedDateRange)).toBeInTheDocument();
	});

	it('updates Backtest Date when duration or year changes', async () => {
		render(NewBacktestPage);

		expect(await screen.findByText('Backtest Date')).toBeInTheDocument();

		// Change duration to 1 Month
		const durationSelect = screen.getByLabelText(/^Backtest Duration/i);
		await fireEvent.change(durationSelect, { target: { value: '1' } });

		expect(screen.getByText('1 Jan 2026 - 31 Jan 2026')).toBeInTheDocument();

		// Change year to 2023 (test year 2024, leap year)
		const year2023Btn = screen.getByRole('button', { name: '2023' });
		await fireEvent.click(year2023Btn);

		expect(screen.getByText('1 Jan 2024 - 31 Jan 2024')).toBeInTheDocument();
	});
});
