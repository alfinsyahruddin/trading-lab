import { describe, it, expect, vi } from 'vitest';
import { tick } from 'svelte';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import PortfolioChart from '$lib/components/backtest/PortfolioChart.svelte';
import type { PortfolioHistoryEntry } from '$lib/types';

const mockSetData = vi.fn();
const mockFitContent = vi.fn();
const mockApplyOptions = vi.fn();
const mockRemove = vi.fn();
let crosshairCallback: ((param: unknown) => void) | null = null;
const mockSubscribeCrosshairMove = vi.fn((cb) => {
	crosshairCallback = cb;
});
const mockUnsubscribeCrosshairMove = vi.fn(() => {
	crosshairCallback = null;
});

const mockSeriesInstance = {
	setData: mockSetData
};

vi.mock('lightweight-charts', () => {
	return {
		createChart: vi.fn(() => ({
			addSeries: vi.fn(() => mockSeriesInstance),
			timeScale: vi.fn(() => ({
				fitContent: mockFitContent
			})),
			applyOptions: mockApplyOptions,
			subscribeCrosshairMove: mockSubscribeCrosshairMove,
			unsubscribeCrosshairMove: mockUnsubscribeCrosshairMove,
			remove: mockRemove
		})),
		ColorType: { Solid: 'solid' },
		BaselineSeries: 'Baseline'
	};
});

describe('PortfolioChart', () => {
	const mockData: PortfolioHistoryEntry[] = [
		{ date: '2024-01-02', net_value: 10_500_000, gross_value: 11_000_000 },
		{ date: '2024-01-03', net_value: 11_200_000, gross_value: 12_000_000 }
	];

	it('renders Realized P/L and switches between Net and Gross', async () => {
		const user = userEvent.setup();
		render(PortfolioChart, {
			props: {
				data: mockData,
				initialCash: 10_000_000
			}
		});

		expect(screen.getByText('Realized P/L')).toBeInTheDocument();
		expect(screen.getByText('Net')).toBeInTheDocument();
		expect(screen.getByText('Gross')).toBeInTheDocument();

		// Net P/L should be shown initially: (11.2m - 10m) = +Rp1.200.000 (+12.00%)
		expect(screen.getByText(/Rp1\.200\.000/)).toBeInTheDocument();

		// Switch to Gross P/L
		const grossBtn = screen.getByRole('button', { name: /^gross$/i });
		await user.click(grossBtn);

		// Gross P/L should now be shown: (12m - 10m) = +Rp2.000.000 (+20.00%)
		expect(screen.getByText(/Rp2\.000\.000/)).toBeInTheDocument();

		// mockSetData should have been called with gross values
		expect(mockSetData).toHaveBeenCalledWith([
			{ time: '2024-01-02', value: 11_000_000 },
			{ time: '2024-01-03', value: 12_000_000 }
		]);
	});

	it('updates Realized P/L when cursor/crosshair is dragged and restores on leave', async () => {
		render(PortfolioChart, {
			props: {
				data: mockData,
				initialCash: 10_000_000
			}
		});

		expect(crosshairCallback).not.toBeNull();

		// Simulate moving cursor over day 1 (net value 10_500_000 -> +Rp500.000 (+5.00%))
		const seriesDataMap = new Map();
		seriesDataMap.set(mockSeriesInstance, { time: '2024-01-02', value: 10_500_000 });

		crosshairCallback!({
			point: { x: 50, y: 100 },
			time: '2024-01-02',
			seriesData: seriesDataMap
		});
		await tick();

		expect(screen.getByText(/Rp500\.000/)).toBeInTheDocument();
		expect(screen.getByText(/\(\+5\.00%\)/)).toBeInTheDocument();

		// Simulate cursor leaving chart
		crosshairCallback!({
			point: undefined,
			time: undefined,
			seriesData: new Map()
		});
		await tick();

		// Restores to final value: +Rp1.200.000 (+12.00%)
		expect(screen.getByText(/Rp1\.200\.000/)).toBeInTheDocument();
		expect(screen.getByText(/\(\+12\.00%\)/)).toBeInTheDocument();
	});
});
