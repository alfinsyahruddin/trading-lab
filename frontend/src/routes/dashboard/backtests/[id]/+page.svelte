<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import StatusBadge from '$lib/components/backtest/StatusBadge.svelte';
	import SegmentedControl from '$lib/components/SegmentedControl.svelte';
	import PortfolioChart from '$lib/components/backtest/PortfolioChart.svelte';
	import HalfDoughnutChart from '$lib/components/backtest/HalfDoughnutChart.svelte';
	import { getBacktest, getTradingStrategy, updateBacktest, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import { formatRiskReward, formatTimeAgo } from '$lib/constants';
	import type { BacktestJob, TradingStrategy } from '$lib/types';

	let job = $state<BacktestJob | null>(null);
	let strategy = $state<TradingStrategy | null>(null);
	let loading = $state(true);
	let error = $state('');
	let pollInterval: ReturnType<typeof setInterval> | null = null;
	let updatingVisibility = $state(false);

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
					} catch {
						// Polling error silently ignored
					}
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

	async function handleVisibilityChange(newIsPublic: boolean) {
		if (!job || updatingVisibility) return;
		if (job.is_public === newIsPublic) return;

		const prev = job.is_public;
		job.is_public = newIsPublic;
		updatingVisibility = true;

		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}
			const updated = await updateBacktest(token, job.id, { is_public: newIsPublic });
			job.is_public = updated.is_public;
			toast.success(`Backtest is now ${newIsPublic ? 'Public' : 'Private'}.`);
		} catch (err) {
			job.is_public = prev;
			toast.error(err instanceof ApiError ? err.message : 'Failed to update visibility.');
		} finally {
			updatingVisibility = false;
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

	function formatDuration(months: number): string {
		if (months === 12) return '1 Year';
		if (months === 1) return '1 Month';
		return `${months} Months`;
	}
</script>

<div class="mb-4">
	<a
		href="/dashboard/backtests"
		class="btn-interactive font-600 inline-flex items-center gap-1.5 text-xs transition-colors duration-150"
		style="color: var(--fg-muted);"
	>
		<Icon icon="lucide:arrow-left" width="14" height="14" />
		<span>Back to Backtests</span>
	</a>
</div>

{#if loading}
	<div class="flex min-h-75 items-center justify-center">
		<div class="flex flex-col items-center gap-3">
			<Icon icon="lucide:loader-2" class="text-accent animate-spin" width="32" height="32" />
			<p class="font-500 text-sm" style="color: var(--fg-muted)">Loading result…</p>
		</div>
	</div>
{:else if error}
	<div
		class="rounded-xl border p-6 text-center"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<p class="font-600 text-sm" style="color: var(--danger)">{error}</p>
	</div>
{:else if job}
	<!-- Backtest Card (like in backtest list, but without bottom section) -->
	<div class="mb-6">
		<div
			class="flex flex-col rounded-2xl border p-4 transition-colors duration-200 sm:p-5"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<!-- Top Row: Title + Year + Status + Visibility Toggle -->
			<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
				<div class="flex flex-wrap items-center gap-2">
					<h1 class="font-700 text-base leading-snug sm:text-lg" style="color: var(--fg)">
						{job.name}
					</h1>
					<span
						class="font-600 inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 text-xs"
						style="background-color: var(--bg-card-hover, #eee); color: var(--fg)"
					>
						<Icon icon="lucide:calendar" width="12" height="12" style="color: var(--fg-muted);" />
						<span>{job.year}</span>
					</span>
					<StatusBadge status={job.status} />
				</div>

				<!-- Small Public/Private Toggle -->
				<div class="w-38 self-start sm:self-auto">
					<SegmentedControl
						size="sm"
						options={[
							{ value: false, label: 'Private', icon: 'lucide:lock' },
							{ value: true, label: 'Public', icon: 'lucide:globe' }
						]}
						value={job.is_public}
						onchange={handleVisibilityChange}
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
						{formatRupiah(job.initial_cash)}
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
						<Icon icon="lucide:layers" width="13" height="13" style="color: var(--fg-muted);" />
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
						<Icon icon="lucide:percent" width="13" height="13" style="color: var(--fg-muted);" />
						{job.buy_fee_percentage}% / {job.sell_fee_percentage}%
					</span>
				</div>
			</div>
		</div>
	</div>

	{#if job.status === 'PENDING' || job.status === 'PROCESSING'}
		<div
			class="flex flex-col items-center justify-center rounded-xl border p-12 text-center"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<Icon
				icon="lucide:loader-2"
				class="mb-4 animate-spin"
				style="color: var(--accent)"
				width="40"
				height="40"
			/>
			<h2 class="font-700 mb-1 text-lg" style="color: var(--fg)">Backtest is running...</h2>
			<p class="font-500 text-sm" style="color: var(--fg-muted)">
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
					<h3 class="font-600 text-base" style="color: var(--danger)">Backtest Failed</h3>
					<p class="font-500 mt-1 text-sm" style="color: var(--danger); opacity: 0.9;">
						{job.error_message}
					</p>
				</div>
			</div>
		</div>
	{:else if job.status === 'DONE' && job.result}
		<div class="flex flex-col gap-6">
			<!-- Trading Strategy Card (like in trading strategy list) -->
			{#if strategy}
				<div
					class="group flex flex-col rounded-2xl border p-4 transition-colors duration-200 sm:p-5"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<!-- Top Row: Title + Visibility Badge + Action Buttons -->
					<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
						<!-- Title & Visibility -->
						<div class="flex flex-col gap-1">
							<div class="flex flex-wrap items-center gap-2">
								<h2 class="font-700 text-base leading-snug sm:text-lg" style="color: var(--fg)">
									{strategy.name}
								</h2>
							</div>

							{#if strategy.description}
								<p class="font-400 line-clamp-2 text-xs sm:text-sm" style="color: var(--fg-muted)">
									{strategy.description}
								</p>
							{/if}
						</div>

						<!-- Actions: Trading Strategy Detail Button -->
						<div class="flex flex-wrap items-center gap-1.5 self-start">
							<a
								href="/dashboard/strategies/{strategy.id}"
								class="btn-interactive font-600 flex h-8 items-center gap-1.5 rounded-lg border px-2.5 text-xs transition-colors duration-150 hover:bg-(--bg-card-hover)"
								style="border-color: var(--border); color: var(--fg);"
								title="Trading Strategy Detail"
								aria-label="View trading strategy detail for {strategy.name}"
							>
								<Icon icon="lucide:external-link" width="13" height="13" />
								<span>Trading Strategy Detail</span>
							</a>
						</div>
					</div>

					<!-- Middle Row: Strategy Key Performance Parameters Grid -->
					<div
						class="mt-4 grid grid-cols-2 gap-2 rounded-xl p-3 sm:grid-cols-4 sm:gap-3"
						style="background-color: var(--bg-card-hover, var(--bg));"
					>
						<!-- TP -->
						<div class="flex flex-col">
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)"
							>
								Take Profit
							</span>
							<span class="font-700 text-sm" style="color: var(--success)">
								+{strategy.tp_percentage}%
							</span>
						</div>

						<!-- SL -->
						<div class="flex flex-col">
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)"
							>
								Stop Loss
							</span>
							<span class="font-700 text-sm" style="color: var(--danger)">
								-{strategy.sl_percentage}%
							</span>
						</div>

						<!-- Risk Reward Ratio -->
						<div class="flex flex-col">
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)"
							>
								Risk : Reward
							</span>
							<span class="font-700 text-sm" style="color: var(--accent)">
								{formatRiskReward(strategy.tp_percentage, strategy.sl_percentage)}
							</span>
						</div>

						<!-- Max Holding Period -->
						<div class="flex flex-col">
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)"
							>
								Max Holding
							</span>
							<span class="font-700 flex items-center gap-1 text-sm" style="color: var(--fg)">
								<Icon icon="lucide:clock" width="13" height="13" style="color: var(--fg-muted);" />
								{strategy.max_holding_period_days} Days
							</span>
						</div>
					</div>

					<!-- Bottom Row: Rules Tags Preview (Left) + Timestamps (Right) -->
					<div
						class="mt-4 flex flex-col gap-3 border-t pt-3 sm:flex-row sm:items-end sm:justify-between"
						style="border-color: var(--border);"
					>
						<!-- Left: Rules & Conditions Tags -->
						{#if strategy.rules && strategy.rules.length > 0}
							<div class="flex min-w-0 flex-1 flex-col gap-1.5">
								<div
									class="font-600 flex items-center gap-1.5 text-xs"
									style="color: var(--fg-muted)"
								>
									<Icon icon="lucide:code-2" width="13" height="13" />
									<span>Rules & Conditions:</span>
								</div>

								<div class="flex flex-wrap items-center gap-1.5">
									{#each strategy.rules as group, gIdx (gIdx)}
										{#if gIdx > 0}
											<span
												class="font-800 rounded px-1.5 py-0.5 font-mono text-[10px] uppercase"
												style="background-color: var(--accent-soft); color: var(--accent);"
											>
												{strategy.rules[gIdx - 1]?.connector_to_next || 'AND'}
											</span>
										{/if}

										<div
											class="inline-flex flex-wrap items-center gap-1 rounded-xl border px-2 py-1"
											style="border-color: var(--border); background-color: var(--bg-card);"
										>
											{#each group.conditions as condition, cIdx (cIdx)}
												{#if cIdx > 0}
													<span
														class="font-700 font-mono text-[10px]"
														style="color: var(--accent);"
													>
														{group.conditions[cIdx - 1]?.connector_to_next || 'AND'}
													</span>
												{/if}

												<span
													class="font-500 inline-flex items-center gap-1 rounded-md px-1.5 py-0.5 text-xs"
													style="
														background-color: var(--bg-card-hover, var(--bg));
														color: var(--fg);
													"
												>
													<span
														class="font-600 font-mono text-[11px]"
														style="color: var(--accent);"
													>
														{condition.variable}
													</span>
													<span class="font-mono text-[11px]" style="color: var(--fg-muted);">
														{condition.operator}
													</span>
													<span class="font-600 text-[11px]">
														{condition.value}
													</span>
												</span>
											{/each}
										</div>
									{/each}
								</div>
							</div>
						{:else}
							<div class="flex-1"></div>
						{/if}

						<!-- Right: Timestamps (Edited at & Created at) -->
						<div
							class="font-500 flex shrink-0 flex-col gap-0.5 text-left text-[11px] sm:items-end sm:text-right"
							style="color: var(--fg-muted);"
						>
							<div class="inline-flex items-center gap-1">
								<Icon icon="lucide:clock" width="11" height="11" />
								<span>Edited {formatTimeAgo(strategy.updated_at)}</span>
							</div>
							<div class="inline-flex items-center gap-1">
								<Icon icon="lucide:calendar" width="11" height="11" />
								<span>Created {formatTimeAgo(strategy.created_at)}</span>
							</div>
						</div>
					</div>
				</div>
			{:else if job.strategy_name}
				<div
					class="flex items-center justify-between rounded-xl border p-4"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="flex items-center gap-2">
						<Icon
							icon="lucide:candlestick-chart"
							width="16"
							height="16"
							style="color: var(--accent);"
						/>
						<span class="font-700 text-sm" style="color: var(--fg)">{job.strategy_name}</span>
					</div>
				</div>
			{/if}

			<!-- Main Content Grid -->
			<div class="flex flex-col gap-6 lg:flex-row">
				<!-- Left: Chart & Basics (~60%) -->
				<div class="flex flex-col gap-6 lg:w-3/5">
					<PortfolioChart data={job.portfolio_history || []} initialCash={job.initial_cash} />

					<div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
						<div
							class="flex flex-col rounded-xl border p-4"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Initial Cash</span
							>
							<span class="font-700 mt-1 text-lg" style="color: var(--fg)"
								>{formatRupiah(job.initial_cash)}</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-4"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Available Cash</span
							>
							<span class="font-700 mt-1 text-lg" style="color: var(--fg)"
								>{formatRupiah(job.result.available_cash)}</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-4"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[11px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Trades Processed</span
							>
							<span class="font-700 mt-1 text-lg" style="color: var(--fg)"
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
									class="font-600 text-[11px] tracking-wider uppercase"
									style="color: var(--fg-muted)">Net P/L</span
								>
								<span
									class="font-700 mt-1 text-base"
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
									class="font-600 text-[11px] tracking-wider uppercase"
									style="color: var(--fg-muted)">Gross P/L</span
								>
								<span
									class="font-700 mt-1 text-base"
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
							<div class="mb-2 flex w-full items-start justify-between">
								<div class="flex flex-col">
									<span
										class="font-600 text-[11px] tracking-wider uppercase"
										style="color: var(--fg-muted)">Win Rate</span
									>
									<span class="font-700 mt-0.5 text-xl" style="color: var(--fg)"
										>{job.result.win_rate.toFixed(1)}%</span
									>
								</div>
								<div class="flex flex-col items-end">
									<span
										class="font-600 text-[11px] tracking-wider uppercase"
										style="color: var(--fg-muted)">Profit Factor</span
									>
									<span class="font-700 mt-0.5 text-xl" style="color: var(--fg)"
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
								class="font-600 text-[10px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Max Profit</span
							>
							<span class="font-700 mt-0.5 text-sm" style="color: var(--success)"
								>+{job.result.max_profit_percentage.toFixed(2)}%</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[10px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Max Loss</span
							>
							<span class="font-700 mt-0.5 text-sm" style="color: var(--danger)"
								>{job.result.max_loss_percentage.toFixed(2)}%</span
							>
						</div>

						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[10px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Avg Profit</span
							>
							<span class="font-700 mt-0.5 text-sm" style="color: var(--success)"
								>+{job.result.avg_profit_percentage.toFixed(2)}%</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[10px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Avg Loss</span
							>
							<span class="font-700 mt-0.5 text-sm" style="color: var(--danger)"
								>{job.result.avg_loss_percentage.toFixed(2)}%</span
							>
						</div>

						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[10px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Avg Hold Time</span
							>
							<span class="font-700 mt-0.5 text-sm" style="color: var(--fg)"
								>{job.result.avg_hold_time_days.toFixed(1)} Days</span
							>
						</div>
						<div
							class="flex flex-col rounded-xl border p-3.5"
							style="background-color: var(--bg-card); border-color: var(--border);"
						>
							<span
								class="font-600 text-[10px] tracking-wider uppercase"
								style="color: var(--fg-muted)">Total Fees</span
							>
							<span class="font-700 mt-0.5 text-sm" style="color: var(--danger)"
								>{formatRupiah(job.result.total_fees)}</span
							>
						</div>

						<div
							class="col-span-2 grid grid-cols-2 gap-4 border-t pt-4"
							style="border-color: var(--border);"
						>
							<div class="flex flex-col">
								<span
									class="font-600 text-[10px] tracking-wider uppercase"
									style="color: var(--fg-muted)">Avg Win Hold</span
								>
								<span class="font-700 mt-0.5 text-sm" style="color: var(--fg)"
									>{job.result.avg_win_hold_days.toFixed(1)} Days</span
								>
							</div>
							<div class="flex flex-col">
								<span
									class="font-600 text-[10px] tracking-wider uppercase"
									style="color: var(--fg-muted)">Avg Loss Hold</span
								>
								<span class="font-700 mt-0.5 text-sm" style="color: var(--fg)"
									>{job.result.avg_loss_hold_days.toFixed(1)} Days</span
								>
							</div>
							<div class="flex flex-col">
								<span
									class="font-600 text-[10px] tracking-wider uppercase"
									style="color: var(--fg-muted)">Sharpe Ratio</span
								>
								<span class="font-700 mt-0.5 text-sm" style="color: var(--fg)"
									>{job.result.sharpe_ratio.toFixed(2)}</span
								>
							</div>
							<div class="flex flex-col">
								<span
									class="font-600 text-[10px] tracking-wider uppercase"
									style="color: var(--fg-muted)">Portfolio Volatility</span
								>
								<span class="font-700 mt-0.5 text-sm" style="color: var(--fg)"
									>{job.result.portfolio_volatility.toFixed(2)}%</span
								>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Tables Grid -->
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
				<!-- Most Traded -->
				<div
					class="flex flex-col overflow-hidden rounded-xl border"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="border-b px-4 py-3" style="border-color: var(--border);">
						<h3 class="font-700 text-sm" style="color: var(--fg)">Most Traded</h3>
					</div>
					<div class="flex flex-col gap-3 p-4">
						{#each job.most_traded || [] as item (item.code)}
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									<span class="font-700 text-sm" style="color: var(--fg)">{stripJK(item.code)}</span
									>
									<span
										class="font-500 rounded px-1.5 py-0.5 text-xs"
										style="background-color: var(--bg-card-hover, #eee); color: var(--fg-muted)"
										>{item.total}x</span
									>
								</div>
								<span
									class="font-600 text-xs"
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
					class="flex flex-col overflow-hidden rounded-xl border"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="border-b px-4 py-3" style="border-color: var(--border);">
						<h3 class="font-700 text-sm" style="color: var(--fg)">Top Gainers</h3>
					</div>
					<div class="flex flex-col gap-3 p-4">
						{#each job.top_gainers || [] as item (item.code)}
							<div class="flex items-center justify-between">
								<span class="font-700 text-sm" style="color: var(--fg)">{stripJK(item.code)}</span>
								<span class="font-600 text-xs" style="color: var(--success)">
									{formatPnl(item.pnl, item.pnl_percentage)}
								</span>
							</div>
						{/each}
					</div>
				</div>

				<!-- Top Losers -->
				<div
					class="flex flex-col overflow-hidden rounded-xl border"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="border-b px-4 py-3" style="border-color: var(--border);">
						<h3 class="font-700 text-sm" style="color: var(--fg)">Top Losers</h3>
					</div>
					<div class="flex flex-col gap-3 p-4">
						{#each job.top_losers || [] as item (item.code)}
							<div class="flex items-center justify-between">
								<span class="font-700 text-sm" style="color: var(--fg)">{stripJK(item.code)}</span>
								<span class="font-600 text-xs" style="color: var(--danger)">
									{formatPnl(item.pnl, item.pnl_percentage)}
								</span>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- Trade History Table -->
			<div
				class="flex flex-col overflow-hidden rounded-xl border"
				style="background-color: var(--bg-card); border-color: var(--border);"
			>
				<div
					class="flex items-center justify-between border-b px-4 py-3"
					style="border-color: var(--border);"
				>
					<h3 class="font-700 text-sm" style="color: var(--fg)">Trade History</h3>
					<span class="font-500 text-xs" style="color: var(--fg-muted)"
						>{(job.trade_history || []).length} Trades</span
					>
				</div>

				<div class="overflow-x-auto">
					<table class="w-full border-collapse text-left text-sm">
						<thead>
							<tr
								class="border-b"
								style="border-color: var(--border); background-color: var(--bg-card-hover, rgba(0,0,0,0.02)); color: var(--fg-muted);"
							>
								<th class="font-600 px-4 py-3">Code</th>
								<th class="font-600 px-4 py-3">P/L</th>
								<th class="font-600 px-4 py-3">Exit Reason</th>
								<th class="font-600 px-4 py-3 text-right">Lot</th>
								<th class="font-600 px-4 py-3 text-right">Buy / Sell Price</th>
								<th class="font-600 px-4 py-3 text-right">Buy / Sell Value</th>
								<th class="font-600 px-4 py-3 text-right">Buy / Sell Date</th>
							</tr>
						</thead>
						<tbody>
							{#each job.trade_history || [] as t, idx (t.code + t.buy_date + idx)}
								{@const badge = getExitReasonBadge(t.exit_reason)}
								<tr
									class="border-b transition-colors hover:bg-(--bg-card-hover)"
									style="border-color: var(--border);"
								>
									<td class="font-700 px-4 py-3" style="color: var(--fg)">{stripJK(t.code)}</td>
									<td
										class="font-600 px-4 py-3"
										style="color: {t.pnl >= 0 ? 'var(--success)' : 'var(--danger)'}"
									>
										{formatPnl(t.pnl, t.pnl_percentage)}
									</td>
									<td class="px-4 py-3">
										<span
											class="font-700 inline-flex rounded-full px-2 py-0.5 text-[10px] tracking-wider whitespace-nowrap"
											style="background-color: {badge.bg}; color: {badge.color};"
										>
											{badge.label}
										</span>
									</td>
									<td class="font-500 px-4 py-3 text-right" style="color: var(--fg)">{t.lot}</td>
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
						<div class="font-500 p-8 text-center text-sm" style="color: var(--fg-muted)">
							No trades were executed in this backtest.
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
{/if}
