<script lang="ts">
	import {
		createChart,
		ColorType,
		BaselineSeries,
		type IChartApi,
		type ISeriesApi,
		type BaselineData,
		type Time
	} from 'lightweight-charts';
	import type { PortfolioHistoryEntry } from '$lib/types';
	import SegmentedControl from '$lib/components/SegmentedControl.svelte';

	let { data, initialCash }: { data: PortfolioHistoryEntry[]; initialCash: number } = $props();

	let container = $state<HTMLElement | null>(null);
	let mode = $state<'NET' | 'GROSS'>('NET');

	let chart: IChartApi | null = null;
	let baselineSeries: ISeriesApi<'Baseline'> | null = null;

	const modeOptions = [
		{ value: 'NET', label: 'Net' },
		{ value: 'GROSS', label: 'Gross' }
	];

	// Extract the realized (final) value for the header display
	const finalValue = $derived(
		data.length > 0
			? mode === 'NET'
				? data[data.length - 1].net_value
				: data[data.length - 1].gross_value
			: initialCash
	);

	const pnl = $derived(finalValue - initialCash);
	const pnlPercentage = $derived((pnl / initialCash) * 100);
	const isProfit = $derived(pnl >= 0);

	function formatRupiah(value: number): string {
		const abs = Math.abs(value);
		const formatted = new Intl.NumberFormat('id-ID').format(Math.round(abs));
		const sign = value >= 0 ? '' : '-';
		return `${sign}Rp${formatted}`;
	}

	function formatCompactRupiah(price: number): string {
		const abs = Math.abs(price);
		const sign = price < 0 ? '-' : '';
		if (abs >= 1_000_000_000) {
			const formatted = parseFloat((abs / 1_000_000_000).toFixed(2));
			return `${sign}${formatted}M`;
		}
		if (abs >= 1_000_000) {
			const formatted = parseFloat((abs / 1_000_000).toFixed(2));
			return `${sign}${formatted}jt`;
		}
		if (abs >= 1_000) {
			const formatted = parseFloat((abs / 1_000).toFixed(1));
			return `${sign}${formatted}rb`;
		}
		return `${sign}${abs.toFixed(0)}`;
	}

	function formatPnlDisplay(value: number, percentage: number): string {
		const sign = value >= 0 ? '+' : '';
		return `${formatRupiah(value)} (${sign}${percentage.toFixed(2)}%)`;
	}

	// Colors matching the design system
	const colors = {
		upColor: '#10b981', // success
		downColor: '#ef4444', // danger
		bg: 'transparent',
		textColor: '#94a3b8', // muted
		gridColor: 'rgba(148, 163, 184, 0.1)'
	};

	$effect(() => {
		if (!container) return;

		const isDark =
			(typeof document !== 'undefined' && document.documentElement.classList.contains('dark')) ||
			(typeof window !== 'undefined' &&
				window.matchMedia?.('(prefers-color-scheme: dark)')?.matches);

		const currentTextColor = isDark ? '#94a3b8' : '#64748b';
		const currentGridColor = isDark ? 'rgba(255, 255, 255, 0.05)' : 'rgba(0, 0, 0, 0.05)';

		const localChart = createChart(container, {
			localization: {
				priceFormatter: formatCompactRupiah
			},
			layout: {
				background: { type: ColorType.Solid, color: colors.bg },
				textColor: currentTextColor
			},
			grid: {
				vertLines: { color: currentGridColor },
				horzLines: { color: currentGridColor }
			},
			rightPriceScale: {
				borderVisible: false
			},
			timeScale: {
				borderVisible: false,
				fixLeftEdge: true,
				fixRightEdge: true
			},
			crosshair: {
				mode: 1 // normal
			},
			height: 300
		});

		const localSeries = localChart.addSeries(BaselineSeries, {
			baseValue: { type: 'price', price: initialCash },
			priceFormat: {
				type: 'custom',
				formatter: formatCompactRupiah
			},
			topLineColor: colors.upColor,
			topFillColor1: 'rgba(16, 185, 129, 0.28)',
			topFillColor2: 'rgba(16, 185, 129, 0.05)',
			bottomLineColor: colors.downColor,
			bottomFillColor1: 'rgba(239, 68, 68, 0.05)',
			bottomFillColor2: 'rgba(239, 68, 68, 0.28)',
			lineWidth: 2
		});

		chart = localChart;
		baselineSeries = localSeries;

		// Initial data load if available
		if (data && data.length > 0) {
			const chartData: BaselineData[] = data.map((entry) => ({
				time: entry.date as Time,
				value: mode === 'NET' ? entry.net_value : entry.gross_value
			}));
			localSeries.setData(chartData);
			localChart.timeScale().fitContent();
		}

		// Handle resize
		const handleResize = () => {
			if (container && localChart) {
				localChart.applyOptions({ width: container.clientWidth });
			}
		};
		window.addEventListener('resize', handleResize);

		return () => {
			window.removeEventListener('resize', handleResize);
			localChart.remove();
			chart = null;
			baselineSeries = null;
		};
	});

	$effect(() => {
		const currentMode = mode;
		const currentData = data;
		if (baselineSeries && currentData && currentData.length > 0) {
			const chartData: BaselineData[] = currentData.map((entry) => ({
				time: entry.date as Time,
				value: currentMode === 'NET' ? entry.net_value : entry.gross_value
			}));
			baselineSeries.setData(chartData);
			chart?.timeScale().fitContent();
		}
	});
</script>

<div
	class="flex flex-col rounded-xl border"
	style="background-color: var(--bg-card); border-color: var(--border);"
>
	<div
		class="flex flex-col gap-4 border-b p-4 sm:flex-row sm:items-center sm:justify-between sm:p-5"
		style="border-color: var(--border);"
	>
		<div class="flex flex-col gap-1">
			<span class="font-600 text-xs" style="color: var(--fg)">Realized P/L</span>
			<span class="font-700 text-md" style="color: {isProfit ? 'var(--success)' : 'var(--danger)'}">
				{formatPnlDisplay(pnl, pnlPercentage)}
			</span>
		</div>

		<div class="w-full sm:w-42">
			<SegmentedControl size="sm" options={modeOptions} bind:value={mode} />
		</div>
	</div>

	<div class="p-4 sm:p-5">
		<div bind:this={container} class="w-full" style="min-height: 300px;"></div>
	</div>
</div>
