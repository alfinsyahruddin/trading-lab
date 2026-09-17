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

		// Default year is 2025 -> test year 2026, default duration is 3 months (3 Months)
		const expectedDateRange = calculateBacktestDateRange(2025, 3).formatted;
		expect(screen.getByText(expectedDateRange)).toBeInTheDocument();
		expect(screen.getByText('(90 Days)')).toBeInTheDocument();
	});

	it('updates Backtest Date when duration or year changes', async () => {
		render(NewBacktestPage);

		expect(await screen.findByText('Backtest Date')).toBeInTheDocument();

		// Change duration to 1 Month
		const durationTrigger = screen.getByLabelText(/^Backtest Duration/i);
		await fireEvent.click(durationTrigger);
		await fireEvent.click(screen.getByRole('option', { name: /1 Month/i }));

		expect(screen.getByText('1 Jan 2026 - 31 Jan 2026')).toBeInTheDocument();
		expect(screen.getByText('(31 Days)')).toBeInTheDocument();

		// Change year to 2023 (test year 2024, leap year)
		const year2023Btn = screen.getByRole('button', { name: '2023' });
		await fireEvent.click(year2023Btn);

		expect(screen.getByText('1 Jan 2024 - 31 Jan 2024')).toBeInTheDocument();
		expect(screen.getByText('(31 Days)')).toBeInTheDocument();
	});

	it('renders Max Stocks input with default value 12 and submits max_stocks in payload', async () => {
		render(NewBacktestPage);

		expect(await screen.findByText('Backtest Date')).toBeInTheDocument();

		const maxStocksInput = screen.getByLabelText(/^Max Stocks/i) as HTMLInputElement;
		expect(maxStocksInput).toBeInTheDocument();
		expect(maxStocksInput.value).toBe('12');

		await fireEvent.input(maxStocksInput, { target: { value: '20' } });
		expect(maxStocksInput.value).toBe('20');

		await fireEvent.click(screen.getByRole('button', { name: /run backtest/i }));

		expect(api.createBacktest).toHaveBeenCalledWith(
			'valid-token',
			expect.objectContaining({
				max_stocks: 20
			})
		);
	});

	it('sets default backtest name to "Backtest <strategy name> - <year>"', async () => {
		render(NewBacktestPage);

		expect(await screen.findByText('Backtest Date')).toBeInTheDocument();

		const nameInput = screen.getByLabelText(/^Backtest Name/i) as HTMLInputElement;
		expect(nameInput).toBeInTheDocument();
		expect(nameInput.value).toBe('Backtest Momentum Breakout - 2025');

		// Changing year should update the default name if pristine
		const year2024Btn = screen.getByRole('button', { name: '2024' });
		await fireEvent.click(year2024Btn);
		expect(nameInput.value).toBe('Backtest Momentum Breakout - 2024');
	});

	it('displays dynamic stock filtering year informative note under Backtest Data Year', async () => {
		render(NewBacktestPage);

		expect(await screen.findByText('Backtest Date')).toBeInTheDocument();

		// Default year 2025
		expect(
			screen.getByText(/the year selected will be used for filtering stocks/i)
		).toBeInTheDocument();
		expect(screen.getByText('pb[2025]')).toBeInTheDocument();
		expect(screen.getByText('ebitda[2025]')).toBeInTheDocument();

		// Change year to 2022
		const year2022Btn = screen.getByRole('button', { name: '2022' });
		await fireEvent.click(year2022Btn);

		expect(screen.getByText('pb[2022]')).toBeInTheDocument();
		expect(screen.getByText('ebitda[2022]')).toBeInTheDocument();
	});
});
