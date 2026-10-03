import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import HalfDoughnutChart from '#lib/components/backtest/HalfDoughnutChart.svelte';

describe('HalfDoughnutChart', () => {
	it('renders trade totals and win/loss breakdown correctly', () => {
		const { container } = render(HalfDoughnutChart, { props: { wins: 15, losses: 5 } });

		expect(screen.getByText('20')).toBeInTheDocument();
		expect(screen.getByText('Trades')).toBeInTheDocument();
		expect(screen.getByText('15x')).toBeInTheDocument();
		expect(screen.getByText('5x')).toBeInTheDocument();

		// Check SVG paths for win/loss strokes
		const paths = container.querySelectorAll('svg path');
		// 1 track + 1 losses + 1 wins = 3 paths
		expect(paths.length).toBe(3);
	});

	it('renders correctly when there are zero trades', () => {
		const { container } = render(HalfDoughnutChart, { props: { wins: 0, losses: 0 } });

		expect(screen.getByText('0')).toBeInTheDocument();
		const zeroCounts = screen.getAllByText('0x');
		expect(zeroCounts.length).toBe(2);

		// Only background track should be rendered
		const paths = container.querySelectorAll('svg path');
		expect(paths.length).toBe(1);
	});

	it('renders only win arc when there are 100% wins', () => {
		const { container } = render(HalfDoughnutChart, { props: { wins: 10, losses: 0 } });

		expect(screen.getByText('10')).toBeInTheDocument();
		expect(screen.getByText('10x')).toBeInTheDocument();
		expect(screen.getByText('0x')).toBeInTheDocument();

		// 1 track + 1 wins arc = 2 paths (no losses path rendered because lossesArcLen is 0)
		const paths = container.querySelectorAll('svg path');
		expect(paths.length).toBe(2);
	});
});
