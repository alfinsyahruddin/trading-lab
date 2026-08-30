<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import StatusBadge from '$lib/components/backtest/StatusBadge.svelte';
	import BacktestResultPreview from '$lib/components/backtest/BacktestResultPreview.svelte';
	import { listBacktests, deleteBacktest, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { BacktestJob } from '$lib/types';

	let backtests = $state<BacktestJob[]>([]);
	let loading = $state(true);
	let pollInterval: ReturnType<typeof setInterval> | null = null;

	let deleteOpen = $state(false);
	let deleteLoading = $state(false);
	let deleteTarget = $state<BacktestJob | null>(null);

	const groupedBacktests = $derived.by(() => {
		const groups = new Map<string, BacktestJob[]>();
		for (const bt of backtests) {
			const groupName = bt.strategy_name || 'Unknown Strategy';
			if (!groups.has(groupName)) {
				groups.set(groupName, []);
			}
			groups.get(groupName)!.push(bt);
		}
		return Array.from(groups.entries()).sort((a, b) => a[0].localeCompare(b[0]));
	});

	onMount(async () => {
		await loadBacktests();
		startPollingIfNeeded();
	});

	onDestroy(() => {
		stopPolling();
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
			backtests = data.sort(
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

	function startPollingIfNeeded() {
		const hasActive = backtests.some((b) => b.status === 'PENDING' || b.status === 'PROCESSING');
		if (hasActive && !pollInterval) {
			pollInterval = setInterval(async () => {
				await loadBacktests(true);
				const stillActive = backtests.some(
					(b) => b.status === 'PENDING' || b.status === 'PROCESSING'
				);
				if (!stillActive) {
					stopPolling();
				}
			}, 5000);
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

	function formatRupiah(value: number): string {
		const abs = Math.abs(value);
		const formatted = new Intl.NumberFormat('id-ID').format(Math.round(abs));
		const sign = value >= 0 ? '+' : '-';
		return `${sign}Rp${formatted}`;
	}

	function formatPnl(value: number, percentage: number): string {
		return `${formatRupiah(value)} (${value >= 0 ? '+' : ''}${percentage.toFixed(2)}%)`;
	}
</script>

<div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
	<div>
		<h1 class="text-2xl font-700" style="color: var(--fg)">Backtests</h1>
		<p class="mt-0.5 text-sm" style="color: var(--fg-muted)">
			Run and evaluate strategies on historical market data.
		</p>
	</div>
	<a
		href="/dashboard/backtests/new"
		class="btn-interactive inline-flex items-center justify-center gap-2 rounded-xl px-4 py-2.5 text-sm font-600 text-white shadow-sm hover:shadow-md hover:opacity-95 active:scale-95 w-full sm:w-auto"
		style="background-color: var(--accent);"
	>
		<Icon icon="lucide:play" width="16" height="16" />
		<span>Run Backtest</span>
	</a>
</div>

{#if !loading}
	<p class="mb-4 text-xs font-600 uppercase tracking-wider" style="color: var(--fg-muted)">
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
			<p class="text-sm font-500" style="color: var(--fg-muted)">Loading backtests…</p>
		</div>
	</div>
{:else if backtests.length === 0}
	<div
		class="flex min-h-100 flex-col items-center justify-center rounded-2xl border p-8 text-center"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<div
			class="animate-float mb-4 flex h-16 w-16 items-center justify-center rounded-2xl"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:flask-conical" width="32" height="32" />
		</div>
		<h2 class="text-lg font-700" style="color: var(--fg)">No Backtests Yet</h2>
		<p class="mt-1 max-w-md text-sm font-400" style="color: var(--fg-muted)">
			Run your first backtest to see how your strategies perform on historical data.
		</p>
		<a
			href="/dashboard/backtests/new"
			class="btn-interactive mt-6 inline-flex items-center gap-2 rounded-xl px-5 py-2.5 text-sm font-600 text-white shadow-sm"
			style="background-color: var(--accent);"
		>
			<Icon icon="lucide:play" width="16" height="16" />
			<span>Run Backtest</span>
		</a>
	</div>
{:else}
	<div class="flex flex-col gap-8">
		{#each groupedBacktests as [strategyName, jobs]}
			<div class="flex flex-col gap-3">
				<div
					class="flex items-center gap-2 text-sm font-600 uppercase tracking-wider"
					style="color: var(--fg-muted)"
				>
					<Icon icon="lucide:candlestick-chart" width="16" height="16" />
					<span>{strategyName}</span>
				</div>

				<div class="flex flex-col gap-3">
					{#each jobs as job (job.id)}
						<div
							class="flex flex-col sm:flex-row sm:items-center justify-between rounded-xl border p-4 transition-colors duration-200"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<!-- Left Info -->
							<div class="flex flex-col gap-2">
								<div class="flex items-center gap-2 flex-wrap">
									<h3 class="text-base font-700" style="color: var(--fg)">{job.name}</h3>
									<span
										class="inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-600"
										style="background-color: var(--bg-card-hover, #eee); color: var(--fg)"
									>
										{job.year}
									</span>
									<StatusBadge status={job.status} />
								</div>

								{#if job.status === 'FAILED'}
									<p class="text-sm font-500" style="color: var(--danger)">
										{job.error_message || 'Unknown error occurred.'}
									</p>
								{:else if job.status === 'DONE' && job.result && job.portfolio_history}
									<div class="flex items-center gap-4 mt-2 flex-wrap">
										<div class="flex flex-col">
											<span
												class="text-[10px] font-600 uppercase tracking-wider"
												style="color: var(--fg-muted)">Net P/L</span
											>
											<span
												class="text-sm font-700"
												style="color: {job.result.net_pnl >= 0
													? 'var(--success)'
													: 'var(--danger)'}"
											>
												{formatPnl(job.result.net_pnl, job.result.net_pnl_percentage)}
											</span>
										</div>
										<div class="flex flex-col">
											<span
												class="text-[10px] font-600 uppercase tracking-wider"
												style="color: var(--fg-muted)">Win Rate</span
											>
											<span class="text-sm font-700" style="color: var(--fg)"
												>{job.result.win_rate.toFixed(1)}%</span
											>
										</div>

										<div class="ml-2 hidden sm:block">
											<BacktestResultPreview
												data={job.portfolio_history}
												initialCash={job.initial_cash}
											/>
										</div>
									</div>
								{/if}
							</div>

							<!-- Right Actions -->
							<div class="mt-4 sm:mt-0 flex items-center gap-2 self-start sm:self-center">
								{#if job.status === 'DONE'}
									<a
										href="/dashboard/backtests/{job.id}"
										class="btn-interactive flex h-8 items-center gap-1.5 rounded-lg border px-3 text-xs font-600 transition-colors duration-150 hover:bg-(--bg-card-hover)"
										style="border-color: var(--border); color: var(--fg);"
									>
										<Icon icon="lucide:eye" width="14" height="14" />
										<span>View</span>
									</a>
								{/if}
								<button
									type="button"
									onclick={() => openDelete(job)}
									class="btn-interactive flex h-8 w-8 items-center justify-center rounded-lg border transition-colors duration-150 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950"
									style="border-color: var(--border); color: var(--fg-muted);"
								>
									<Icon icon="lucide:trash-2" width="14" height="14" />
								</button>
							</div>
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
