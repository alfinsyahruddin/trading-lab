import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import BacktestResultPreview from '$lib/components/backtest/BacktestResultPreview.svelte';
import type { PortfolioHistoryEntry } from '$lib/types';

const mockSetData = vi.fn();
const mockFitContent = vi.fn();
const mockAddSeries = vi.fn(() => ({
	setData: mockSetData
}));
const mockChartInstance = {
	addSeries: mockAddSeries,
	timeScale: vi.fn(() => ({
		fitContent: mockFitContent
	}))
};
const mockCreateChart = vi.fn((_container?: HTMLElement, _options?: unknown) => mockChartInstance);

vi.mock('lightweight-charts', () => ({
	createChart: (container: HTMLElement, options?: unknown) => mockCreateChart(container, options),
	ColorType: { Solid: 'solid' },
	AreaSeries: 'Area'
}));

describe('BacktestResultPreview', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('initializes chart with profit green color scheme when net value increases', () => {
		const mockData: PortfolioHistoryEntry[] = [
			{ date: '2024-01-01', net_value: 10_000_000, gross_value: 10_000_000 },
			{ date: '2024-01-02', net_value: 12_000_000, gross_value: 12_000_000 }
		];

		render(BacktestResultPreview, { props: { data: mockData, initialCash: 10_000_000 } });

		expect(mockCreateChart).toHaveBeenCalled();
		expect(mockAddSeries).toHaveBeenCalledWith(
			'Area',
			expect.objectContaining({
				lineColor: '#10b981',
				topColor: 'rgba(16, 185, 129, 0.4)'
			})
		);
		expect(mockSetData).toHaveBeenCalledWith([
			{ time: '2024-01-01', value: 10_000_000 },
			{ time: '2024-01-02', value: 12_000_000 }
		]);
		expect(mockFitContent).toHaveBeenCalled();
	});

	it('initializes chart with loss red color scheme when net value decreases', () => {
		const mockData: PortfolioHistoryEntry[] = [
			{ date: '2024-01-01', net_value: 10_000_000, gross_value: 10_000_000 },
			{ date: '2024-01-02', net_value: 8_000_000, gross_value: 8_000_000 }
		];

		render(BacktestResultPreview, { props: { data: mockData, initialCash: 10_000_000 } });

		expect(mockCreateChart).toHaveBeenCalled();
		expect(mockAddSeries).toHaveBeenCalledWith(
			'Area',
			expect.objectContaining({
				lineColor: '#ef4444',
				topColor: 'rgba(239, 68, 68, 0.4)'
			})
		);
	});

	it('does not initialize chart when portfolio history data is empty', () => {
		render(BacktestResultPreview, { props: { data: [], initialCash: 10_000_000 } });

		expect(mockCreateChart).not.toHaveBeenCalled();
	});
});
