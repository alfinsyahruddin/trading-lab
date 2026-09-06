<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import StatusBadge from '$lib/components/backtest/StatusBadge.svelte';
	import BacktestResultPreview from '$lib/components/backtest/BacktestResultPreview.svelte';
	import { listBacktests, deleteBacktest, rerunBacktest, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import { formatTimeAgo } from '$lib/constants';
	import type { BacktestJob } from '$lib/types';

	let backtests = $state<BacktestJob[]>([]);
	let loading = $state(true);
	let pollInterval: ReturnType<typeof setInterval> | null = null;
	let rerunLoadingId = $state<string | null>(null);

	let deleteOpen = $state(false);
	let deleteLoading = $state(false);
	let deleteTarget = $state<BacktestJob | null>(null);

	const groupedBacktests = $derived.by(() => {
		const groups: Record<string, BacktestJob[]> = {};
		for (const bt of backtests) {
			const groupName = bt.strategy_name || 'Unknown Strategy';
			if (!groups[groupName]) {
				groups[groupName] = [];
			}
			groups[groupName].push(bt);
		}
		return Object.entries(groups).sort((a, b) => a[0].localeCompare(b[0]));
	});

	const hasActiveJobs = $derived(
		backtests.some((b) => b.status === 'PENDING' || b.status === 'PROCESSING')
	);

	onMount(async () => {
		await loadBacktests();
	});

	onDestroy(() => {
		stopPolling();
	});

	$effect(() => {
		if (hasActiveJobs) {
			if (!pollInterval) {
				pollInterval = setInterval(async () => {
					await loadBacktests(true);
				}, 5000);
			}
		} else {
			stopPolling();
		}

		return () => {
			stopPolling();
		};
	});

	async function loadBacktests(silent = false) {
		if (!silent) loading = true;
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}
			const data = await listBacktests(token);
			backtests = [...data].sort(
				(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
			);
		} catch (err) {
			if (!silent) {
				toast.error(err instanceof ApiError ? err.message : 'Failed to load backtests.');
			}
		} finally {
			if (!silent) loading = false;
		}
	}

	function stopPolling() {
		if (pollInterval) {
			clearInterval(pollInterval);
			pollInterval = null;
		}
	}

	function openDelete(bt: BacktestJob) {
		deleteTarget = bt;
		deleteOpen = true;
	}

	async function handleDelete() {
		if (!deleteTarget) return;
		deleteLoading = true;
		try {
			const token = getToken()!;
			await deleteBacktest(token, deleteTarget.id);
			backtests = backtests.filter((b) => b.id !== deleteTarget!.id);
			deleteOpen = false;
			toast.success(`Backtest "${deleteTarget.name}" deleted.`);
			deleteTarget = null;
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to delete backtest.');
			deleteOpen = false;
		} finally {
			deleteLoading = false;
		}
	}

	async function handleRerun(bt: BacktestJob) {
		if (rerunLoadingId) return;
		rerunLoadingId = bt.id;
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}
			const updated = await rerunBacktest(token, bt.id);
			backtests = backtests.map((b) => (b.id === bt.id ? { ...b, ...updated } : b));
			toast.success(`Backtest "${bt.name}" restarted.`);
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to rerun backtest.');
		} finally {
			rerunLoadingId = null;
		}
	}

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
</script>

<div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
	<div>
		<h1 class="font-700 text-2xl" style="color: var(--fg)">Backtests</h1>
		<p class="mt-0.5 text-sm" style="color: var(--fg-muted)">
			Run and evaluate strategies on historical market data.
		</p>
	</div>
	<a
		href="/dashboard/backtests/new"
		class="btn-interactive font-600 inline-flex w-full items-center justify-center gap-2 rounded-xl px-4 py-2.5 text-sm text-white shadow-sm hover:opacity-95 hover:shadow-md active:scale-95 sm:w-auto"
		style="background-color: var(--accent);"
	>
		<Icon icon="lucide:play" width="16" height="16" />
		<span>Run Backtest</span>
	</a>
</div>

{#if !loading}
	<p class="font-600 mb-4 text-xs tracking-wider uppercase" style="color: var(--fg-muted)">
		{backtests.length}
		{backtests.length === 1 ? 'backtest' : 'backtests'}
	</p>
{/if}

{#if loading}
	<div class="flex min-h-75 items-center justify-center">
		<div class="flex flex-col items-center gap-3">
			<Icon
				icon="lucide:loader-2"
				class="animate-spin"
				style="color: var(--accent);"
				width="32"
				height="32"
			/>
			<p class="font-500 text-sm" style="color: var(--fg-muted)">Loading backtests…</p>
		</div>
	</div>
{:else if backtests.length === 0}
	<div
		class="flex min-h-100 flex-col items-center justify-center rounded-2xl border p-8 text-center"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<div
			class="animate-float mb-4 flex size-16 items-center justify-center rounded-2xl"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:flask-conical" width="32" height="32" />
		</div>
		<h2 class="font-700 text-lg" style="color: var(--fg)">No Backtests Yet</h2>
		<p class="font-400 mt-1 max-w-md text-sm" style="color: var(--fg-muted)">
			Run your first backtest to see how your strategies perform on historical data.
		</p>
		<a
			href="/dashboard/backtests/new"
			class="btn-interactive font-600 mt-6 inline-flex items-center gap-2 rounded-xl px-5 py-2.5 text-sm text-white shadow-sm"
			style="background-color: var(--accent);"
		>
			<Icon icon="lucide:play" width="16" height="16" />
			<span>Run Backtest</span>
		</a>
	</div>
{:else}
	<div class="flex flex-col gap-8">
		{#each groupedBacktests as [strategyName, jobs] (strategyName)}
			<div class="flex flex-col gap-3">
				<div
					class="font-600 flex items-center gap-2 text-sm tracking-wider uppercase"
					style="color: var(--fg-muted)"
				>
					<Icon icon="lucide:candlestick-chart" width="16" height="16" />
					<span>{strategyName}</span>
				</div>

				<div class="flex flex-col gap-4">
					{#each jobs as job (job.id)}
						<div
							class="group flex cursor-pointer flex-col rounded-2xl border p-4 transition-all duration-200 hover:border-(--border-strong) hover:bg-(--bg-card-hover) sm:p-5"
							style="background-color: var(--bg-card); border-color: var(--border);"
							onclick={() => goto(`/dashboard/backtests/${job.id}`)}
							onkeydown={(e) => {
								if (e.key === 'Enter' || e.key === ' ') {
									e.preventDefault();
									goto(`/dashboard/backtests/${job.id}`);
								}
							}}
							role="button"
							tabindex="0"
						>
							<!-- Top Row: Title + Year + Status + Actions -->
							<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div class="flex flex-wrap items-center gap-2">
									<h3
										class="font-700 text-base leading-snug transition-colors group-hover:text-(--accent) sm:text-lg"
										style="color: var(--fg)"
									>
										{job.name}
									</h3>
									<span
										class="font-600 inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs"
										style="background-color: var(--bg-card-hover, #eee); color: var(--fg)"
									>
										{job.year}
									</span>
									<StatusBadge status={job.status} />

									<!-- Public / Private Status Icon Badge -->
									{#if job.is_public}
										<span
											class="font-600 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs"
											style="background-color: var(--accent-soft); color: var(--accent);"
											title="Public backtest"
										>
											<Icon icon="lucide:globe" width="12" height="12" />
											<span>Public</span>
										</span>
									{:else}
										<span
											class="font-600 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs"
											style="background-color: rgba(148, 163, 184, 0.15); color: var(--fg-muted);"
											title="Private backtest"
										>
											<Icon icon="lucide:lock" width="12" height="12" />
											<span>Private</span>
										</span>
									{/if}
								</div>

								<!-- Right Actions -->
								<div class="flex items-center gap-2 self-start sm:self-center">
									<button
										type="button"
										onclick={(e) => {
											e.stopPropagation();
											openDelete(job);
										}}
										class="btn-interactive flex size-8 items-center justify-center rounded-lg border transition-colors duration-150 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950"
										style="border-color: var(--border); color: var(--fg-muted);"
										title="Delete backtest"
										aria-label="Delete {job.name}"
									>
										<Icon icon="lucide:trash-2" width="14" height="14" />
									</button>
									<Icon
										icon="lucide:chevron-right"
										width="18"
										height="18"
										class="transition-transform duration-150 group-hover:translate-x-0.5"
										style="color: var(--fg-muted);"
									/>
								</div>
							</div>

							<!-- Middle Row: Parameters Grid (Initial Cash, Duration, Max Stocks, Fees) -->
							<div
								class="mt-4 grid grid-cols-2 gap-2 rounded-xl p-3 sm:grid-cols-4 sm:gap-3"
								style="background-color: var(--bg-card-hover, var(--bg));"
							>
								<!-- Initial Cash -->
								<div class="flex flex-col">
									<span
										class="font-600 text-[11px] tracking-wider uppercase"
										style="color: var(--fg-muted)"
									>
										Initial Cash
									</span>
									<span class="font-700 text-sm" style="color: var(--fg)">
										Rp {formatRupiah(job.initial_cash)}
									</span>
								</div>

								<!-- Duration -->
								<div class="flex flex-col">
									<span
										class="font-600 text-[11px] tracking-wider uppercase"
										style="color: var(--fg-muted)"
									>
										Duration
									</span>
									<span class="font-700 flex items-center gap-1 text-sm" style="color: var(--fg)">
										<Icon
											icon="lucide:calendar-range"
											width="13"
											height="13"
											style="color: var(--fg-muted);"
										/>
										{formatDuration(job.backtest_duration_months)}
									</span>
								</div>

								<!-- Max Holding Stocks -->
								<div class="flex flex-col">
									<span
										class="font-600 text-[11px] tracking-wider uppercase"
										style="color: var(--fg-muted)"
									>
										Max Holding
									</span>
									<span class="font-700 flex items-center gap-1 text-sm" style="color: var(--fg)">
										<Icon
											icon="lucide:layers"
											width="13"
											height="13"
											style="color: var(--fg-muted);"
										/>
										{job.max_holding_stocks} Stocks
									</span>
								</div>

								<!-- Trading Fees -->
								<div class="flex flex-col">
									<span
										class="font-600 text-[11px] tracking-wider uppercase"
										style="color: var(--fg-muted)"
									>
										Fees (Buy / Sell)
									</span>
									<span class="font-700 flex items-center gap-1 text-sm" style="color: var(--fg)">
										<Icon
											icon="lucide:percent"
											width="13"
											height="13"
											style="color: var(--fg-muted);"
										/>
										{job.buy_fee_percentage}% / {job.sell_fee_percentage}%
									</span>
								</div>
							</div>

							<!-- Bottom Row: Performance Results & Timestamp -->
							{#if job.status === 'DONE' && job.result}
								<div
									class="mt-4 flex flex-col gap-3 border-t pt-3 sm:flex-row sm:items-end sm:justify-between"
									style="border-color: var(--border);"
								>
									<div class="flex flex-wrap items-center gap-4 sm:gap-6">
										<!-- Sparkline Preview on the Left -->
										{#if job.portfolio_history && job.portfolio_history.length > 0}
											<div class="shrink-0">
												<BacktestResultPreview
													data={job.portfolio_history}
													initialCash={job.initial_cash}
												/>
											</div>
										{/if}

										<!-- Net PnL -->
										<div class="flex flex-col">
											<span
												class="font-600 text-[10px] tracking-wider uppercase"
												style="color: var(--fg-muted)"
											>
												Net P/L
											</span>
											<span
												class="font-700 text-sm"
												style="color: {job.result.net_pnl >= 0
													? 'var(--success)'
													: 'var(--danger)'}"
											>
												{formatPnl(job.result.net_pnl, job.result.net_pnl_percentage)}
											</span>
										</div>

										<!-- Win Rate -->
										<div class="flex flex-col">
											<span
												class="font-600 text-[10px] tracking-wider uppercase"
												style="color: var(--fg-muted)"
											>
												Win Rate
											</span>
											<span class="font-700 text-sm" style="color: var(--fg)">
												{job.result.win_rate.toFixed(1)}%
											</span>
										</div>

										<!-- Profit Factor -->
										<div class="flex flex-col">
											<span
												class="font-600 text-[10px] tracking-wider uppercase"
												style="color: var(--fg-muted)"
											>
												Profit Factor
											</span>
											<span class="font-700 text-sm" style="color: var(--fg)">
												{job.result.profit_factor.toFixed(2)}
											</span>
										</div>

										<!-- Trades Processed -->
										<div class="flex flex-col">
											<span
												class="font-600 text-[10px] tracking-wider uppercase"
												style="color: var(--fg-muted)"
											>
												Trades
											</span>
											<span class="font-700 text-sm" style="color: var(--fg)">
												{job.result.trades_processed}
											</span>
										</div>
									</div>

									<!-- Timestamp in Bottom Right -->
									<div
										class="font-500 flex shrink-0 items-center gap-1 self-end text-[11px]"
										style="color: var(--fg-muted);"
									>
										<Icon icon="lucide:clock" width="11" height="11" />
										<span>Created {formatTimeAgo(job.created_at)}</span>
									</div>
								</div>
							{:else if job.status === 'FAILED'}
								<div
									class="mt-4 flex flex-col gap-3 border-t pt-3 sm:flex-row sm:items-center sm:justify-between"
									style="border-color: var(--border);"
								>
									<p
										class="font-500 flex min-w-0 items-center gap-1.5 text-sm"
										style="color: var(--danger)"
									>
										<Icon icon="lucide:alert-circle" width="14" height="14" class="shrink-0" />
										<span class="wrap-break-word"
											>{job.error_message || 'Unknown error occurred.'}</span
										>
									</p>
									<div class="flex shrink-0 items-center gap-3 self-end sm:self-center">
										<button
											type="button"
											onclick={(e) => {
												e.stopPropagation();
												handleRerun(job);
											}}
											disabled={rerunLoadingId === job.id}
											class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs text-white shadow-2xs transition-all hover:opacity-95 active:scale-95 disabled:cursor-not-allowed disabled:opacity-50"
											style="background-color: var(--accent);"
											title="Run backtest again"
										>
											<Icon
												icon={rerunLoadingId === job.id ? 'lucide:loader-2' : 'lucide:rotate-cw'}
												class={rerunLoadingId === job.id ? 'animate-spin' : ''}
												width="13"
												height="13"
											/>
											<span>Run Again</span>
										</button>
										<div
											class="font-500 flex shrink-0 items-center gap-1 text-[11px]"
											style="color: var(--fg-muted);"
										>
											<Icon icon="lucide:clock" width="11" height="11" />
											<span>Created {formatTimeAgo(job.created_at)}</span>
										</div>
									</div>
								</div>
							{:else}
								<div
									class="mt-4 flex flex-col gap-2 border-t pt-3 sm:flex-row sm:items-end sm:justify-between"
									style="border-color: var(--border);"
								>
									<p
										class="font-500 flex items-center gap-1.5 text-sm"
										style="color: var(--fg-muted)"
									>
										<Icon
											icon="lucide:loader-2"
											class="animate-spin"
											width="14"
											height="14"
											style="color: var(--accent);"
										/>
										<span>Backtest simulation in progress…</span>
									</p>
									<div
										class="font-500 flex shrink-0 items-center gap-1 self-end text-[11px]"
										style="color: var(--fg-muted);"
									>
										<Icon icon="lucide:clock" width="11" height="11" />
										<span>Created {formatTimeAgo(job.created_at)}</span>
									</div>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
{/if}

<ConfirmModal
	bind:open={deleteOpen}
	title="Delete Backtest"
	message="Are you sure you want to delete backtest '{deleteTarget?.name}'? This action cannot be undone."
	confirmLabel="Delete Backtest"
	loading={deleteLoading}
	onconfirm={handleDelete}
/>
