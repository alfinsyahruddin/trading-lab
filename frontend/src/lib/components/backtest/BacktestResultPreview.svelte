<script lang="ts">
	import {
		createChart,
		ColorType,
		AreaSeries,
		type IChartApi,
		type AreaData,
		type Time
	} from 'lightweight-charts';
	import type { PortfolioHistoryEntry } from '$lib/types';

	let { data, initialCash }: { data: PortfolioHistoryEntry[]; initialCash: number } = $props();

	let container = $state<HTMLElement | null>(null);
	let chart = $state<IChartApi | null>(null);

	const finalValue = $derived(data.length > 0 ? data[data.length - 1].net_value : initialCash);
	const isProfit = $derived(finalValue >= initialCash);

	const colors = {
		upLine: '#10b981', // success
		upFillTop: 'rgba(16, 185, 129, 0.4)',
		upFillBottom: 'rgba(16, 185, 129, 0.0)',
		downLine: '#ef4444', // danger
		downFillTop: 'rgba(239, 68, 68, 0.4)',
		downFillBottom: 'rgba(239, 68, 68, 0.0)'
	};

	$effect(() => {
		if (container && !chart && data.length > 0) {
			chart = createChart(container, {
				width: container.clientWidth || 120,
				height: container.clientHeight || 40,
				layout: {
					background: { type: ColorType.Solid, color: 'transparent' }
				},
				grid: {
					vertLines: { visible: false },
					horzLines: { visible: false }
				},
				rightPriceScale: { visible: false },
				leftPriceScale: { visible: false },
				timeScale: {
					visible: false,
					fixLeftEdge: true,
					fixRightEdge: true
				},
				crosshair: { mode: 0, vertLine: { visible: false }, horzLine: { visible: false } },
				handleScroll: false,
				handleScale: false
			});

			const series = chart.addSeries(AreaSeries, {
				lineColor: isProfit ? colors.upLine : colors.downLine,
				topColor: isProfit ? colors.upFillTop : colors.downFillTop,
				bottomColor: isProfit ? colors.upFillBottom : colors.downFillBottom,
				lineWidth: 2,
				crosshairMarkerVisible: false,
				priceLineVisible: false
			});

			const chartData: AreaData[] = data.map((entry) => ({
				time: entry.date as Time,
				value: entry.net_value
			}));

			series.setData(chartData);
			chart.timeScale().fitContent();
		}
	});
</script>

<div bind:this={container} class="h-10 w-30"></div>
