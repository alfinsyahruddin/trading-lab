<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import StatusBadge from '$lib/components/backtest/StatusBadge.svelte';
	import PortfolioChart from '$lib/components/backtest/PortfolioChart.svelte';
	import HalfDoughnutChart from '$lib/components/backtest/HalfDoughnutChart.svelte';
	import { getBacktest, getTradingStrategy, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { BacktestJob, TradingStrategy } from '$lib/types';

	let job = $state<BacktestJob | null>(null);
	let strategy = $state<TradingStrategy | null>(null);
	let loading = $state(true);
	let error = $state('');
	let pollInterval: ReturnType<typeof setInterval> | null = null;

	const id = $derived($page.params.id);

	onMount(async () => {
		await loadData();
	});

	onDestroy(() => {
		stopPolling();
	});

	async function loadData() {
		try {
			const token = getToken();
			if (!token) return goto('/login');

			if (!id) return;

			job = await getBacktest(token, id);
			strategy = await getTradingStrategy(token, job.strategy_id);

			if (job.status === 'PENDING' || job.status === 'PROCESSING') {
				startPolling();
			} else {
				stopPolling();
			}
		} catch (err) {
			error = err instanceof ApiError ? err.message : 'Failed to load backtest result.';
			toast.error(error);
		} finally {
			loading = false;
		}
	}

	function startPolling() {
		if (!pollInterval) {
			pollInterval = setInterval(async () => {
				const token = getToken();
				if (token && id) {
					try {
						job = await getBacktest(token, id);
						if (job.status === 'DONE' || job.status === 'FAILED') {
							stopPolling();
						}
					} catch (e) {}
				}
			}, 3000);
		}
	}

	function stopPolling() {
		if (pollInterval) {
			clearInterval(pollInterval);
			pollInterval = null;
		}
	}

	function stripJK(code: string): string {
		return code.replace('.JK', '');
	}

	function formatRupiah(value: number): string {
		const abs = Math.abs(value);
		const formatted = new Intl.NumberFormat('id-ID').format(Math.round(abs));
		const sign = value >= 0 ? '' : '-';
		return `${sign}Rp${formatted}`;
	}

	function formatPnl(value: number, percentage: number): string {
		const sign = value >= 0 ? '+' : '';
		return `${formatRupiah(value)} (${sign}${percentage.toFixed(2)}%)`;
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			weekday: 'short',
			day: 'numeric',
			month: 'short',
			year: 'numeric'
		});
	}

	function getExitReasonBadge(reason: string) {
		if (reason === 'STOP_LOSS')
			return { bg: 'rgba(239, 68, 68, 0.15)', color: 'var(--danger)', label: 'STOP LOSS' };
		if (reason === 'TAKE_PROFIT')
			return { bg: 'rgba(16, 185, 129, 0.15)', color: 'var(--success)', label: 'TAKE PROFIT' };
		return { bg: 'rgba(245, 158, 11, 0.15)', color: 'var(--warning)', label: 'MAX HOLD' };
	}
</script>

<div class="mb-4">
	<a
		href="/dashboard/backtests"
		class="btn-interactive inline-flex items-center gap-1.5 text-xs font-600 transition-colors duration-150"
		style="color: var(--fg-muted);"
	>
		<Icon icon="lucide:arrow-left" width="14" height="14" />
		<span>Back to Backtests</span>
	</a>
</div>

{#if loading}
	<div class="flex min-h-75 items-center justify-center">
		<div class="flex flex-col items-center gap-3">
			<Icon icon="lucide:loader-2" class="animate-spin text-accent" width="32" height="32" />
			<p class="text-sm font-500" style="color: var(--fg-muted)">Loading result…</p>
		</div>
	</div>
{:else if error}
	<div
		class="rounded-xl border p-6 text-center"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<p class="text-sm font-600" style="color: var(--danger)">{error}</p>
	</div>
{:else if job}
	<!-- Header -->
	<div class="mb-6">
		<div class="flex flex-col gap-2">
			<div class="flex items-center gap-3 flex-wrap">
				<h1 class="text-2xl font-700" style="color: var(--fg)">{job.name}</h1>
				<StatusBadge status={job.status} />
				<span
					class="inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-600"
					style="background-color: var(--bg-card-hover, #eee); color: var(--fg)"
				>
					{job.year}
				</span>
			</div>
			<p class="text-sm font-500 flex items-center gap-1.5" style="color: var(--fg-muted)">
				<Icon icon="lucide:candlestick-chart" width="16" height="16" />
				<span>{strategy?.name || job.strategy_name}</span>
			</p>
		</div>
	</div>

	{#if job.status === 'PENDING' || job.status === 'PROCESSING'}
		<div
			class="flex flex-col items-center justify-center rounded-xl border p-12 text-center"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<Icon
				icon="lucide:loader-2"
				class="animate-spin mb-4"
				style="color: var(--accent)"
				width="40"
				height="40"
			/>
			<h2 class="text-lg font-700 mb-1" style="color: var(--fg)">Backtest is running...</h2>
			<p class="text-sm font-500" style="color: var(--fg-muted)">
				This may take a few minutes depending on the duration and data size.
			</p>
		</div>
	{:else if job.status === 'FAILED'}
		<div
			class="rounded-xl border p-6"
			style="background-color: rgba(239, 68, 68, 0.05); border-color: rgba(239, 68, 68, 0.3);"
		>
			<div class="flex items-start gap-3">
				<Icon
					icon="lucide:alert-circle"
					width="24"
					height="24"
					style="color: var(--danger); margin-top: 2px;"
				/>
				<div>
					<h3 class="text-base font-600" style="color: var(--danger)">Backtest Failed</h3>
					<p class="mt-1 text-sm font-500" style="color: var(--danger); opacity: 0.9;">
						{job.error_message}
					</p>
				</div>
			</div>
		</div>
	{:else if job.status === 'DONE' && job.result}
		<div class="flex flex-col gap-6">
			<!-- Strategy Summary Card -->
			<div
				class="flex items-center gap-6 rounded-xl border p-4 overflow-x-auto"
				style="background-color: var(--bg-card); border-color: var(--border);"
			>
				<div class="flex flex-col min-w-max">
					<span class="text-[10px] font-600 uppercase tracking-wider" style="color: var(--fg-muted)"
						>TP Target</span
					>
					<span class="text-sm font-700" style="color: var(--success)"
						>+{strategy?.tp_percentage || '?'}%</span
					>
				</div>
				<div class="flex flex-col min-w-max">
					<span class="text-[10px] font-600 uppercase tracking-wider" style="color: var(--fg-muted)"
						>SL Limit</span
					>
					<span class="text-sm font-700" style="color: var(--danger)"
						>-{strategy?.sl_percentage || '?'}%</span
					>
				</div>
				<div class="flex flex-col min-w-max">
					<span class="text-[10px] font-600 uppercase tracking-wider" style="color: var(--fg-muted)"
						>Max Hold</span
					>
					<span class="text-sm font-700" style="color: var(--fg)"
						>{strategy?.max_holding_period_days || '?'} Days</span
					>
				</div>
				<div class="flex flex-col min-w-max border-l pl-6" style="border-color: var(--border);">
					<span class="text-[10px] font-600 uppercase tracking-wider" style="color: var(--fg-muted)"
						>Duration</span
					>
					<span class="text-sm font-700" style="color: var(--fg)"
						>{job.backtest_duration_months} Months</span
					>
				</div>
				<div class="flex flex-col min-w-max">
					<span class="text-[10px] font-600 uppercase tracking-wider" style="color: var(--fg-muted)"
						>Max Stocks</span
					>
					<span class="text-sm font-700" style="color: var(--fg)">{job.max_holding_stocks}</span>
				</div>
			</div>

			<!-- Main Content Grid -->
			<div class="flex flex-col lg:flex-row gap-6">
				<!-- Left: Chart & Basics (~60%) -->
				<div class="flex flex-col gap-6 lg:w-3/5">
					<PortfolioChart data={job.portfolio_history || []} initialCash={job.initial_cash} />

					<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
						<div
							class="flex flex-col rounded-xl border p-4"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[11px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Initial Cash</span
							>
							<span class="text-lg font-700 mt-1" style="color: var(--fg)"
								>{formatRupiah(job.initial_cash)}</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-4"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[11px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Available Cash</span
							>
							<span class="text-lg font-700 mt-1" style="color: var(--fg)"
								>{formatRupiah(job.result.available_cash)}</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-4"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[11px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Trades Processed</span
							>
							<span class="text-lg font-700 mt-1" style="color: var(--fg)"
								>{job.result.trades_processed}</span
							>
						</div>
					</div>
				</div>

				<!-- Right: Stats Grid (~40%) -->
				<div class="flex flex-col gap-4 lg:w-2/5">
					<div class="grid grid-cols-2 gap-4">
						<!-- PNLs -->
						<div class="col-span-2 grid grid-cols-2 gap-4">
							<div
								class="flex flex-col rounded-xl border p-4"
								style="background-color: var(--bg-card); border-color: var(--border);"
							>
								<span
									class="text-[11px] font-600 uppercase tracking-wider"
									style="color: var(--fg-muted)">Net P/L</span
								>
								<span
									class="text-base font-700 mt-1"
									style="color: {job.result.net_pnl >= 0 ? 'var(--success)' : 'var(--danger)'}"
								>
									{formatPnl(job.result.net_pnl, job.result.net_pnl_percentage)}
								</span>
							</div>
							<div
								class="flex flex-col rounded-xl border p-4"
								style="background-color: var(--bg-card); border-color: var(--border);"
							>
								<span
									class="text-[11px] font-600 uppercase tracking-wider"
									style="color: var(--fg-muted)">Gross P/L</span
								>
								<span
									class="text-base font-700 mt-1"
									style="color: {job.result.gross_pnl >= 0 ? 'var(--success)' : 'var(--danger)'}"
								>
									{formatPnl(job.result.gross_pnl, job.result.gross_pnl_percentage)}
								</span>
							</div>
						</div>

						<!-- Win/Loss Ratio Doughnut -->
						<div
							class="col-span-2 flex flex-col items-center rounded-xl border p-5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<div class="w-full flex justify-between items-start mb-2">
								<div class="flex flex-col">
									<span
										class="text-[11px] font-600 uppercase tracking-wider"
										style="color: var(--fg-muted)">Win Rate</span
									>
									<span class="text-xl font-700 mt-0.5" style="color: var(--fg)"
										>{job.result.win_rate.toFixed(1)}%</span
									>
								</div>
								<div class="flex flex-col items-end">
									<span
										class="text-[11px] font-600 uppercase tracking-wider"
										style="color: var(--fg-muted)">Profit Factor</span
									>
									<span class="text-xl font-700 mt-0.5" style="color: var(--fg)"
										>{job.result.profit_factor.toFixed(2)}</span
									>
								</div>
							</div>
							<HalfDoughnutChart wins={job.result.wins} losses={job.result.losses} />
						</div>

						<!-- More Stats -->
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[10px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Max Profit</span
							>
							<span class="text-sm font-700 mt-0.5" style="color: var(--success)"
								>+{job.result.max_profit_percentage.toFixed(2)}%</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[10px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Max Loss</span
							>
							<span class="text-sm font-700 mt-0.5" style="color: var(--danger)"
								>{job.result.max_loss_percentage.toFixed(2)}%</span
							>
						</div>

						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[10px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Avg Profit</span
							>
							<span class="text-sm font-700 mt-0.5" style="color: var(--success)"
								>+{job.result.avg_profit_percentage.toFixed(2)}%</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[10px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Avg Loss</span
							>
							<span class="text-sm font-700 mt-0.5" style="color: var(--danger)"
								>{job.result.avg_loss_percentage.toFixed(2)}%</span
							>
						</div>

						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[10px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Avg Hold Time</span
							>
							<span class="text-sm font-700 mt-0.5" style="color: var(--fg)"
								>{job.result.avg_hold_time_days.toFixed(1)} Days</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="text-[10px] font-600 uppercase tracking-wider"
								style="color: var(--fg-muted)">Total Fees</span
							>
							<span class="text-sm font-700 mt-0.5" style="color: var(--danger)"
								>{formatRupiah(job.result.total_fees)}</span
							>
						</div>

						<div
							class="col-span-2 grid grid-cols-2 gap-4 border-t pt-4"
							style="border-color: var(--border);"
						>
							<div class="flex flex-col">
								<span
									class="text-[10px] font-600 uppercase tracking-wider"
									style="color: var(--fg-muted)">Avg Win Hold</span
								>
								<span class="text-sm font-700 mt-0.5" style="color: var(--fg)"
									>{job.result.avg_win_hold_days.toFixed(1)} Days</span
								>
							</div>
							<div class="flex flex-col">
								<span
									class="text-[10px] font-600 uppercase tracking-wider"
									style="color: var(--fg-muted)">Avg Loss Hold</span
								>
								<span class="text-sm font-700 mt-0.5" style="color: var(--fg)"
									>{job.result.avg_loss_hold_days.toFixed(1)} Days</span
								>
							</div>
							<div class="flex flex-col">
								<span
									class="text-[10px] font-600 uppercase tracking-wider"
									style="color: var(--fg-muted)"
									>Sharpe Ratio <span class="lowercase normal-case font-400 opacity-70"
										>(Rf=2%)</span
									></span
								>
								<span class="text-sm font-700 mt-0.5" style="color: var(--fg)"
									>{job.result.sharpe_ratio.toFixed(2)}</span
								>
							</div>
							<div class="flex flex-col">
								<span
									class="text-[10px] font-600 uppercase tracking-wider"
									style="color: var(--fg-muted)">Portfolio Volatility</span
								>
								<span class="text-sm font-700 mt-0.5" style="color: var(--fg)"
									>{job.result.portfolio_volatility.toFixed(2)}%</span
								>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Tables Grid -->
			<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
				<!-- Most Traded -->
				<div
					class="flex flex-col rounded-xl border overflow-hidden"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="px-4 py-3 border-b" style="border-color: var(--border);">
						<h3 class="text-sm font-700" style="color: var(--fg)">Most Traded</h3>
					</div>
					<div class="p-4 flex flex-col gap-3">
						{#each job.most_traded || [] as item}
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									<span class="text-sm font-700" style="color: var(--fg)">{stripJK(item.code)}</span
									>
									<span
										class="text-xs font-500 rounded px-1.5 py-0.5"
										style="background-color: var(--bg-card-hover, #eee); color: var(--fg-muted)"
										>{item.total}x</span
									>
								</div>
								<span
									class="text-xs font-600"
									style="color: {item.pnl >= 0 ? 'var(--success)' : 'var(--danger)'}"
								>
									{formatPnl(item.pnl, item.pnl_percentage)}
								</span>
							</div>
						{/each}
					</div>
				</div>

				<!-- Top Gainers -->
				<div
					class="flex flex-col rounded-xl border overflow-hidden"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="px-4 py-3 border-b" style="border-color: var(--border);">
						<h3 class="text-sm font-700" style="color: var(--fg)">Top Gainers</h3>
					</div>
					<div class="p-4 flex flex-col gap-3">
						{#each job.top_gainers || [] as item}
							<div class="flex items-center justify-between">
								<span class="text-sm font-700" style="color: var(--fg)">{stripJK(item.code)}</span>
								<span class="text-xs font-600" style="color: var(--success)">
									{formatPnl(item.pnl, item.pnl_percentage)}
								</span>
							</div>
						{/each}
					</div>
				</div>

				<!-- Top Losers -->
				<div
					class="flex flex-col rounded-xl border overflow-hidden"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="px-4 py-3 border-b" style="border-color: var(--border);">
						<h3 class="text-sm font-700" style="color: var(--fg)">Top Losers</h3>
					</div>
					<div class="p-4 flex flex-col gap-3">
						{#each job.top_losers || [] as item}
							<div class="flex items-center justify-between">
								<span class="text-sm font-700" style="color: var(--fg)">{stripJK(item.code)}</span>
								<span class="text-xs font-600" style="color: var(--danger)">
									{formatPnl(item.pnl, item.pnl_percentage)}
								</span>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- Trade History Table -->
			<div
				class="flex flex-col rounded-xl border overflow-hidden"
				style="background-color: var(--bg-card); border-color: var(--border);"
			>
				<div
					class="px-4 py-3 border-b flex items-center justify-between"
					style="border-color: var(--border);"
				>
					<h3 class="text-sm font-700" style="color: var(--fg)">Trade History</h3>
					<span class="text-xs font-500" style="color: var(--fg-muted)"
						>{(job.trade_history || []).length} Trades</span
					>
				</div>

				<div class="overflow-x-auto">
					<table class="w-full text-left text-sm border-collapse">
						<thead>
							<tr
								class="border-b"
								style="border-color: var(--border); background-color: var(--bg-card-hover, rgba(0,0,0,0.02)); color: var(--fg-muted);"
							>
								<th class="px-4 py-3 font-600">Code</th>
								<th class="px-4 py-3 font-600">P/L</th>
								<th class="px-4 py-3 font-600">Exit Reason</th>
								<th class="px-4 py-3 font-600 text-right">Lot</th>
								<th class="px-4 py-3 font-600 text-right">Buy / Sell Price</th>
								<th class="px-4 py-3 font-600 text-right">Buy / Sell Value</th>
								<th class="px-4 py-3 font-600 text-right">Buy / Sell Date</th>
							</tr>
						</thead>
						<tbody>
							{#each job.trade_history || [] as t}
								{@const badge = getExitReasonBadge(t.exit_reason)}
								<tr
									class="border-b transition-colors hover:bg-(--bg-card-hover)"
									style="border-color: var(--border);"
								>
									<td class="px-4 py-3 font-700" style="color: var(--fg)">{stripJK(t.code)}</td>
									<td
										class="px-4 py-3 font-600"
										style="color: {t.pnl >= 0 ? 'var(--success)' : 'var(--danger)'}"
									>
										{formatPnl(t.pnl, t.pnl_percentage)}
									</td>
									<td class="px-4 py-3">
										<span
											class="inline-flex rounded-full px-2 py-0.5 text-[10px] font-700 tracking-wider whitespace-nowrap"
											style="background-color: {badge.bg}; color: {badge.color};"
										>
											{badge.label}
										</span>
									</td>
									<td class="px-4 py-3 text-right font-500" style="color: var(--fg)">{t.lot}</td>
									<td class="px-4 py-3 text-right">
										<div class="flex flex-col text-xs">
											<span style="color: var(--fg-muted)">B: {formatRupiah(t.buy_price)}</span>
											<span style="color: var(--fg)">S: {formatRupiah(t.sell_price)}</span>
										</div>
									</td>
									<td class="px-4 py-3 text-right">
										<div class="flex flex-col text-xs">
											<span style="color: var(--fg-muted)">B: {formatRupiah(t.buy_value)}</span>
											<span style="color: var(--fg)">S: {formatRupiah(t.sell_value)}</span>
										</div>
									</td>
									<td class="px-4 py-3 text-right">
										<div class="flex flex-col text-xs">
											<span style="color: var(--fg-muted)">B: {formatDate(t.buy_date)}</span>
											<span style="color: var(--fg)">S: {formatDate(t.sell_date)}</span>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
					{#if (job.trade_history || []).length === 0}
						<div class="p-8 text-center text-sm font-500" style="color: var(--fg-muted)">
							No trades were executed in this backtest.
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
{/if}
