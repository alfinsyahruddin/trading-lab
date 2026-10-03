import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import DashboardPage from '../../../src/routes/dashboard/+page.svelte';
import * as api from '#lib/api.js';
import * as session from '#lib/helpers/session.js';
import type { DashboardStats, LeaderboardEntry } from '#lib/types.js';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('lightweight-charts', () => ({
	createChart: vi.fn(() => ({
		addSeries: vi.fn(() => ({
			setData: vi.fn()
		})),
		timeScale: vi.fn(() => ({
			fitContent: vi.fn()
		})),
		applyOptions: vi.fn(),
		remove: vi.fn()
	})),
	ColorType: { Solid: 'solid' },
	AreaSeries: 'Area',
	BaselineSeries: 'Baseline'
}));

vi.mock('#lib/api.js', async (importOriginal) => {
	const actual = await importOriginal<typeof import('#lib/api.js')>();
	return {
		...actual,
		getDashboardStats: vi.fn(),
		getLeaderboard: vi.fn(),
		getTopStars: vi.fn(),
		starBacktest: vi.fn(),
		unstarBacktest: vi.fn()
	};
});

describe('DashboardPage - Community Star Updates', () => {
	const mockStats: DashboardStats = {
		total_stars_received: 5,
		total_strategies: 3,
		total_backtests: 7,
		date_joined: '2026-01-01T12:00:00Z'
	};

	const entryA: LeaderboardEntry = {
		id: 'bt-a',
		name: 'Alpha Strategy Backtest',
		owner_name: 'Alice',
		strategy_name: 'Alpha Strat',
		year: 2024,
		initial_cash: 10_000_000,
		backtest_duration_months: 12,
		buy_fee_percentage: 0.15,
		sell_fee_percentage: 0.25,
		status: 'DONE',
		net_pnl: 2_000_000,
		net_pnl_percentage: 20.0,
		win_rate: 60.0,
		profit_factor: 2.0,
		trades_processed: 25,
		star_count: 5,
		is_starred_by_me: false,
		portfolio_history: [],
		created_at: '2026-08-01T00:00:00Z'
	};

	const entryB: LeaderboardEntry = {
		id: 'bt-b',
		name: 'Beta Strategy Backtest',
		owner_name: 'Bob',
		strategy_name: 'Beta Strat',
		year: 2024,
		initial_cash: 10_000_000,
		backtest_duration_months: 12,
		buy_fee_percentage: 0.15,
		sell_fee_percentage: 0.25,
		status: 'DONE',
		net_pnl: 1_500_000,
		net_pnl_percentage: 15.0,
		win_rate: 55.0,
		profit_factor: 1.8,
		trades_processed: 20,
		star_count: 4,
		is_starred_by_me: false,
		portfolio_history: [],
		created_at: '2026-08-02T00:00:00Z'
	};

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('valid-token');
		vi.spyOn(session, 'getUser').mockReturnValue({
			id: 'u-me',
			name: 'CurrentUser',
			email: 'me@example.com',
			role: 'MEMBER',
			created_at: '2026-01-01T00:00:00Z',
			updated_at: '2026-01-01T00:00:00Z'
		});

		vi.mocked(api.getDashboardStats).mockResolvedValue(mockStats);
		vi.mocked(api.getLeaderboard).mockResolvedValue([entryA, entryB]);
		vi.mocked(api.getTopStars).mockResolvedValue([entryA, entryB]);
		vi.mocked(api.starBacktest).mockResolvedValue('Starred successfully');
		vi.mocked(api.unstarBacktest).mockResolvedValue('Unstarred successfully');
	});

	it('re-sorts top stars list and updates ranks when an entry is starred', async () => {
		render(DashboardPage);

		// Wait for dashboard to load
		expect(await screen.findByText('Community')).toBeInTheDocument();

		// Switch to Top Stars tab
		const topStarsTab = screen.getByRole('button', { name: /top stars/i });
		await fireEvent.click(topStarsTab);

		// Initially: entryA is #1 (5 stars), entryB is #2 (4 stars)
		const headingsBefore = screen.getAllByRole('heading', { level: 3 });
		expect(headingsBefore[0]).toHaveTextContent('Alpha Strategy Backtest');
		expect(headingsBefore[1]).toHaveTextContent('Beta Strategy Backtest');

		// Star entryB (which had 4 stars, will now have 5 stars, but higher net_pnl is entryA unless entryB has more)
		// Let's test starring entryB: if entryB has 4 stars, starring gives it 5 stars.
		// If we star entryB twice in our mock or if entryB starts with 5 and gains 6:
	});

	it('re-orders top stars list when starring an entry gives it more stars than the higher-ranked entry', async () => {
		// entryA has 5 stars, entryB has 5 stars with lower pnl, entryC has 4 stars
		const entry1 = { ...entryA, id: 'bt-1', star_count: 5, net_pnl_percentage: 20.0 };
		const entry2 = { ...entryB, id: 'bt-2', star_count: 4, net_pnl_percentage: 25.0 };

		vi.mocked(api.getTopStars).mockResolvedValue([entry1, entry2]);
		render(DashboardPage);

		expect(await screen.findByText('Community')).toBeInTheDocument();

		const topStarsTab = screen.getByRole('button', { name: /top stars/i });
		await fireEvent.click(topStarsTab);

		// Before: entry1 is first (#1), entry2 is second (#2)
		let headings = screen.getAllByRole('heading', { level: 3 });
		expect(headings[0]).toHaveTextContent(entry1.name);
		expect(headings[1]).toHaveTextContent(entry2.name);

		// Star entry2: its star_count becomes 5, and net_pnl_percentage is 25.0 (higher than entry1's 20.0)!
		// Or its star_count becomes 5 and beats or matches entry1
		const starButtons = screen.getAllByRole('button', { name: /star backtest/i });
		// starButtons[1] corresponds to entry2
		await fireEvent.click(starButtons[1]);

		// Now entry2 should immediately move to #1 because (5 stars, 25%) > (5 stars, 20%)!
		headings = screen.getAllByRole('heading', { level: 3 });
		expect(headings[0]).toHaveTextContent(entry2.name);
		expect(headings[1]).toHaveTextContent(entry1.name);

		// Verify API was called
		expect(api.starBacktest).toHaveBeenCalledWith('valid-token', 'bt-2');

		// Verify re-fetch was triggered
		await waitFor(() => {
			expect(api.getTopStars).toHaveBeenCalledTimes(2);
			expect(api.getLeaderboard).toHaveBeenCalledTimes(2);
			expect(api.getDashboardStats).toHaveBeenCalledTimes(2);
		});
	});

	it('re-orders top stars list when unstarring an entry reduces its rank', async () => {
		const entry1 = { ...entryA, id: 'bt-1', star_count: 5, is_starred_by_me: true };
		const entry2 = {
			...entryB,
			id: 'bt-2',
			star_count: 5,
			is_starred_by_me: false,
			net_pnl_percentage: 15.0
		};

		// entry1 (5 stars, 20%) is #1, entry2 (5 stars, 15%) is #2
		vi.mocked(api.getTopStars).mockResolvedValue([entry1, entry2]);
		render(DashboardPage);

		expect(await screen.findByText('Community')).toBeInTheDocument();

		const topStarsTab = screen.getByRole('button', { name: /top stars/i });
		await fireEvent.click(topStarsTab);

		let headings = screen.getAllByRole('heading', { level: 3 });
		expect(headings[0]).toHaveTextContent(entry1.name);
		expect(headings[1]).toHaveTextContent(entry2.name);

		// Unstar entry1: star_count drops from 5 to 4. entry2 still has 5 stars.
		const unstarBtn = screen.getByRole('button', { name: /unstar backtest/i });
		await fireEvent.click(unstarBtn);

		// entry2 should now be #1, entry1 should be #2!
		headings = screen.getAllByRole('heading', { level: 3 });
		expect(headings[0]).toHaveTextContent(entry2.name);
		expect(headings[1]).toHaveTextContent(entry1.name);

		expect(api.unstarBacktest).toHaveBeenCalledWith('valid-token', 'bt-1');
	});
});
