import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import StatsCard from '$lib/components/dashboard/StatsCard.svelte';

describe('StatsCard', () => {
	it('renders label and value correctly', () => {
		render(StatsCard, {
			props: {
				icon: 'lucide:star',
				label: 'Total Stars',
				value: '42',
				orbColor: 'rgba(245,158,11,0.22)',
				iconFg: 'text-amber-400',
				iconBg: 'bg-amber-500/15'
			}
		});

		expect(screen.getByText('Total Stars')).toBeInTheDocument();
		expect(screen.getByText('42')).toBeInTheDocument();
	});

	it('renders as a link when href is provided', () => {
		render(StatsCard, {
			props: {
				icon: 'lucide:candlestick-chart',
				label: 'Trading Strategies',
				value: '5',
				href: '/dashboard/strategies',
				orbColor: 'rgba(48,180,201,0.22)',
				iconFg: 'text-cyan-400',
				iconBg: 'bg-cyan-500/15'
			}
		});

		const link = screen.getByRole('link');
		expect(link).toHaveAttribute('href', '/dashboard/strategies');
		expect(screen.getByText('Trading Strategies')).toBeInTheDocument();
		expect(screen.getByText('5')).toBeInTheDocument();
	});

	it('renders subtitle correctly when provided', () => {
		render(StatsCard, {
			props: {
				icon: 'lucide:calendar-check',
				label: 'Date Joined',
				value: 'Sun, 30 Aug 2026',
				subtitle: '13.00',
				orbColor: 'rgba(34,197,94,0.22)',
				iconFg: 'text-emerald-400',
				iconBg: 'bg-emerald-500/15'
			}
		});

		expect(screen.getByText('Sun, 30 Aug 2026')).toBeInTheDocument();
		expect(screen.getByText('13.00')).toBeInTheDocument();
		expect(screen.getByText('Date Joined')).toBeInTheDocument();
	});
});
