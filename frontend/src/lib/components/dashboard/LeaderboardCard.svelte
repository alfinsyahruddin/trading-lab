<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import BacktestResultPreview from '$lib/components/backtest/BacktestResultPreview.svelte';
	import { formatTimeAgo } from '$lib/constants';
	import type { LeaderboardEntry } from '$lib/types';

	let {
		entry,
		rank,
		onstar
	}: {
		entry: LeaderboardEntry;
		rank: number;
		onstar: (id: string, starred: boolean) => void;
	} = $props();

	function formatRupiah(value: number): string {
		return new Intl.NumberFormat('id-ID').format(Math.round(value || 0));
	}

	function formatPnl(value: number, percentage: number): string {
		const sign = value >= 0 ? '+' : '-';
		return `${sign}Rp${formatRupiah(Math.abs(value))} (${value >= 0 ? '+' : ''}${percentage.toFixed(2)}%)`;
	}

	function formatDuration(months: number): string {
		if (months === 12) return '1 Year';
		if (months === 1) return '1 Month';
		return `${months} Months`;
	}

	let sparklineData = $derived(
		entry.portfolio_history.map((point) => ({
			date: point.date,
			net_value: point.net_value,
			gross_value: point.net_value
		}))
	);

	let isPositive = $derived(entry.net_pnl >= 0);
</script>

<div
	class="group flex cursor-pointer flex-col rounded-2xl border p-4 transition-all duration-200 hover:border-(--border-strong) hover:bg-(--bg-card-hover) sm:p-5"
	style="background-color: var(--bg-card); border-color: var(--border);"
	onclick={() => goto(`/dashboard/backtests/${entry.id}`)}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			goto(`/dashboard/backtests/${entry.id}`);
		}
	}}
	role="button"
	tabindex="0"
>
	<!-- Top Row: Rank + Name + Owner + Year + Strategy -->
	<div class="flex flex-col gap-2.5 sm:flex-row sm:items-center sm:justify-between">
		<div class="flex min-w-0 items-start gap-2.5 sm:items-center sm:gap-3">
			<div
				class="flex size-7 shrink-0 items-center justify-center rounded-lg text-xs font-bold text-white shadow-sm sm:size-8 sm:text-sm"
				class:bg-amber-500={rank === 1}
				class:bg-slate-400={rank === 2}
				class:bg-[#cd7f32]={rank === 3}
				class:bg-gray-200={rank > 3}
				class:dark:bg-gray-800={rank > 3}
				class:text-gray-600={rank > 3}
				class:dark:text-gray-400={rank > 3}
			>
				#{rank}
			</div>
			<div class="flex min-w-0 flex-1 flex-wrap items-baseline gap-x-2 gap-y-0.5">
				<h3
					class="text-base leading-snug font-bold wrap-break-word transition-colors group-hover:text-(--accent) sm:text-lg"
					style="color: var(--fg)"
				>
					{entry.name}
				</h3>
				<span class="shrink-0 text-xs" style="color: var(--fg-muted)">
					by {entry.owner_name}
				</span>
			</div>
		</div>

		<div class="flex shrink-0 flex-wrap items-center gap-1.5 self-start sm:gap-2 sm:self-center">
			<span
				class="rounded-md px-2 py-0.5 text-xs font-semibold"
				style="background-color: var(--bg-card-hover, #eee); color: var(--fg)"
			>
				{entry.year}
			</span>
			<span
				class="flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-semibold"
				style="background-color: var(--bg-card-hover, #eee); color: var(--fg-muted)"
			>
				<Icon icon="lucide:candlestick-chart" width="13" height="13" />
				<span class="max-w-32 truncate">{entry.strategy_name}</span>
			</span>
		</div>
	</div>

	<!-- Middle Row: Parameters Grid -->
	<div
		class="mt-3 grid grid-cols-2 gap-2 rounded-xl bg-(--bg-card-hover) p-3 sm:grid-cols-3 sm:gap-3"
	>
		<!-- Initial Cash -->
		<div class="flex flex-col">
			<span
				class="text-[11px] font-semibold tracking-wider uppercase"
				style="color: var(--fg-muted)"
			>
				Initial Cash
			</span>
			<span class="text-sm font-bold" style="color: var(--fg)">
				Rp {formatRupiah(entry.initial_cash)}
			</span>
		</div>

		<!-- Duration -->
		<div class="flex flex-col">
			<span
				class="text-[11px] font-semibold tracking-wider uppercase"
				style="color: var(--fg-muted)"
			>
				Duration
			</span>
			<span class="flex items-center gap-1 text-sm font-bold" style="color: var(--fg)">
				<Icon icon="lucide:calendar-range" width="13" height="13" style="color: var(--fg-muted);" />
				{formatDuration(entry.backtest_duration_months)}
			</span>
		</div>

		<!-- Trading Fees -->
		<div class="col-span-2 flex flex-col sm:col-span-1">
			<span
				class="text-[11px] font-semibold tracking-wider uppercase"
				style="color: var(--fg-muted)"
			>
				Fees (Buy / Sell)
			</span>
			<span class="flex items-center gap-1 text-sm font-bold" style="color: var(--fg)">
				<Icon icon="lucide:percent" width="13" height="13" style="color: var(--fg-muted);" />
				{entry.buy_fee_percentage}% / {entry.sell_fee_percentage}%
			</span>
		</div>
	</div>

	<!-- Bottom Row: Sparkline Chart, Net P/L, Stats & Star Action -->
	<div
		class="mt-3 flex flex-col gap-3 border-t pt-3 sm:flex-row sm:items-center sm:justify-between"
		style="border-color: var(--border);"
	>
		<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:gap-6">
			<!-- Primary Performance: Sparkline & Net P/L -->
			<div class="flex items-center gap-3.5">
				{#if sparklineData.length > 0}
					<div class="shrink-0">
						<BacktestResultPreview data={sparklineData} initialCash={entry.initial_cash} />
					</div>
				{/if}

				<!-- Net P/L -->
				<div class="flex min-w-0 flex-col">
					<span
						class="text-[10px] font-semibold tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Net P/L
					</span>
					<span
						class="text-sm font-bold"
						style="color: {isPositive ? 'var(--success)' : 'var(--danger)'}"
					>
						{formatPnl(entry.net_pnl, entry.net_pnl_percentage)}
					</span>
				</div>
			</div>

			<!-- Secondary Stats: Win Rate, Profit Factor, Trades -->
			<div
				class="grid grid-cols-3 gap-2 rounded-xl bg-(--bg-card-hover) p-2.5 text-center sm:flex sm:items-center sm:gap-6 sm:rounded-none sm:bg-transparent sm:p-0 sm:text-left"
			>
				<!-- Win Rate -->
				<div class="flex flex-col">
					<span
						class="text-[10px] font-semibold tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Win Rate
					</span>
					<span class="text-sm font-bold" style="color: var(--fg)">
						{entry.win_rate.toFixed(1)}%
					</span>
				</div>

				<!-- Profit Factor -->
				<div class="flex flex-col">
					<span
						class="text-[10px] font-semibold tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Profit Factor
					</span>
					<span class="text-sm font-bold" style="color: var(--fg)">
						{entry.profit_factor.toFixed(2)}
					</span>
				</div>

				<!-- Trades Processed -->
				<div class="flex flex-col">
					<span
						class="text-[10px] font-semibold tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Trades
					</span>
					<span class="text-sm font-bold" style="color: var(--fg)">
						{entry.trades_processed}
					</span>
				</div>
			</div>
		</div>

		<!-- Action & Timestamp Footer Row -->
		<div
			class="flex w-full items-center justify-between border-t pt-2.5 sm:w-auto sm:justify-end sm:gap-4 sm:border-0 sm:pt-0"
			style="border-color: var(--border);"
		>
			<div class="flex items-center gap-1 text-[11px] font-medium" style="color: var(--fg-muted);">
				<Icon icon="lucide:clock" width="12" height="12" />
				<span>{formatTimeAgo(entry.created_at)}</span>
			</div>

			<button
				type="button"
				class="btn-interactive flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-semibold transition-all duration-150 hover:bg-(--bg-card-hover)"
				style="border-color: var(--border); color: var(--fg);"
				onclick={(e) => {
					e.stopPropagation();
					onstar(entry.id, !entry.is_starred_by_me);
				}}
				aria-label={entry.is_starred_by_me ? 'Unstar backtest' : 'Star backtest'}
			>
				<Icon
					icon="lucide:star"
					width="15"
					height="15"
					class="transition-colors {entry.is_starred_by_me ? 'text-amber-400' : 'text-gray-400'}"
					style={entry.is_starred_by_me ? 'fill: currentColor;' : ''}
				/>
				<span>{entry.star_count}</span>
			</button>
		</div>
	</div>
</div>
