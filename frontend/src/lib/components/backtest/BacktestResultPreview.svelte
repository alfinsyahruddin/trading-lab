<script lang="ts">
	import {
		createChart,
		ColorType,
		BaselineSeries,
		type IChartApi,
		type BaselineData,
		type Time
	} from 'lightweight-charts';
	import type { PortfolioHistoryEntry } from '$lib/types';

	let { data, initialCash }: { data: PortfolioHistoryEntry[]; initialCash: number } = $props();

	let container = $state<HTMLElement | null>(null);
	let chart = $state<IChartApi | null>(null);

	const colors = {
		upColor: '#10b981', // success
		downColor: '#ef4444' // danger
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

	
			const series = chart.addSeries(BaselineSeries, {
				baseValue: { type: 'price', price: initialCash },
				topLineColor: colors.upColor,
				topFillColor1: 'rgba(16, 185, 129, 0.28)',
				topFillColor2: 'rgba(16, 185, 129, 0.05)',
				bottomLineColor: colors.downColor,
				bottomFillColor1: 'rgba(239, 68, 68, 0.05)',
				bottomFillColor2: 'rgba(239, 68, 68, 0.28)',
				lineWidth: 2,
				crosshairMarkerVisible: false,
				priceLineVisible: false,
				lastValueVisible: false
			});

			const chartData: BaselineData[] = data.map((entry) => ({
				time: entry.date as Time,
				value: entry.net_value
			}));

			series.setData(chartData);
			chart.timeScale().fitContent();

		}
	});
</script>

<div bind:this={container} class="h-10 w-30"></div>
