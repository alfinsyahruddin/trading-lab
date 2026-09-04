import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import HeroChartWidget from '../../../../src/lib/components/landing/HeroChartWidget.svelte';

describe('HeroChartWidget', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('renders SVG chart structure and terminal telemetry header', () => {
		const { container } = render(HeroChartWidget);

		expect(screen.getByText(/IDX • TRADING LAB/i)).toBeInTheDocument();
		expect(screen.getByText(/REAL MARKET DATA/i)).toBeInTheDocument();
		expect(container.querySelector('.chart-line')).toBeInTheDocument();
		expect(container.querySelector('.chart-area')).toBeInTheDocument();
		expect(container.querySelector('.chart-pulse-dot')).toBeInTheDocument();
	});

	it('animates numbers from 0 to targets for return, sharpe, and winrate', async () => {
		render(HeroChartWidget, {
			targetReturn: 24.8,
			targetSharpe: 1.82,
			targetWinRate: 75,
			duration: 1000
		});

		// Initially at or close to 0
		expect(screen.getByText(/RETURN/i)).toBeInTheDocument();
		expect(screen.getByText(/SHARPE RATIO/i)).toBeInTheDocument();
		expect(screen.getByText(/WIN RATE/i)).toBeInTheDocument();

		// Advance animation frames past duration
		await vi.advanceTimersByTimeAsync(1200);

		expect(screen.getByText('+24.8%')).toBeInTheDocument();
		expect(screen.getByText('1.82')).toBeInTheDocument();
		expect(screen.getByText('75%')).toBeInTheDocument();
	});
});
