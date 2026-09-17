import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import LeaderboardCard from '$lib/components/dashboard/LeaderboardCard.svelte';
import type { LeaderboardEntry } from '$lib/types';

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

describe('LeaderboardCard', () => {
	const mockEntry: LeaderboardEntry = {
		id: '123e4567-e89b-12d3-a456-426614174000',
		name: 'Golden Cross Momentum',
		owner_name: 'Alfin',
		strategy_name: 'SMA Cross',
		year: 2024,
		initial_cash: 100_000_000,
		backtest_duration_months: 12,
		buy_fee_percentage: 0.15,
		sell_fee_percentage: 0.25,
		status: 'DONE',
		net_pnl: 25_000_000,
		net_pnl_percentage: 25.0,
		win_rate: 65.5,
		profit_factor: 2.34,
		trades_processed: 48,
		star_count: 7,
		is_starred_by_me: false,
		portfolio_history: [
			{ date: '2024-01-01', net_value: 100_000_000 },
			{ date: '2024-12-31', net_value: 125_000_000 }
		],
		created_at: new Date().toISOString()
	};

	it('renders leaderboard card details and fires star callback on click', async () => {
		const user = userEvent.setup();
		const onstar = vi.fn();

		render(LeaderboardCard, {
			props: {
				entry: mockEntry,
				rank: 1,
				onstar
			}
		});

		expect(screen.getByText('#1')).toBeInTheDocument();
		expect(screen.getByText('Golden Cross Momentum')).toBeInTheDocument();
		expect(screen.getByText('by Alfin')).toBeInTheDocument();
		expect(screen.getByText('+Rp25.000.000 (+25.00%)')).toBeInTheDocument();
		expect(screen.getByText('65.5%')).toBeInTheDocument();
		expect(screen.getByText('2.34')).toBeInTheDocument();
		expect(screen.getByText('48')).toBeInTheDocument();
		expect(screen.getByText('7')).toBeInTheDocument();

		const starBtn = screen.getByRole('button', { name: /star backtest/i });
		await user.click(starBtn);
		expect(onstar).toHaveBeenCalledWith(mockEntry.id, true);
	});
});
