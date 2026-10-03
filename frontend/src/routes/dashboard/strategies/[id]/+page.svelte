<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import Modal from '$lib/components/Modal.svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import TextField from '$lib/components/TextField.svelte';
	import StatusBadge from '$lib/components/backtest/StatusBadge.svelte';
	import {
		getTradingStrategy,
		deleteTradingStrategy,
		duplicateTradingStrategy,
		listBacktests,
		ApiError
	} from '$lib/api';
	import { getToken, getUser } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import { formatRiskReward, formatTimeAgo } from '$lib/constants';
	import { findStrategyVariable } from '$lib/helpers/strategy-variables.svelte';
	import type { BacktestJob, TradingStrategy } from '$lib/types';

	let strategy = $state<TradingStrategy | null>(null);
	let backtests = $state<BacktestJob[]>([]);
	let fetching = $state(true);

	// Duplicate modal state
	let duplicateOpen = $state(false);
	let duplicateLoading = $state(false);
	let duplicateError = $state('');
	let duplicateNewName = $state('');

	// Delete modal state
	let deleteOpen = $state(false);
	let deleteLoading = $state(false);

	const strategyId = $derived($page.params.id);
	const currentUser = $derived(getUser());
	const isOwner = $derived(currentUser && strategy ? currentUser.id === strategy.user_id : false);

	const relatedBacktests = $derived(
		strategy
			? backtests.filter(
					(b) => b.strategy_id === strategy!.id || b.strategy_name === strategy!.name
				)
			: []
	);

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		fetching = true;
		try {
			const token = getToken();
			if (!token || !strategyId) {
				goto('/dashboard/strategies');
				return;
			}
			const [stratData, backtestData] = await Promise.all([
				getTradingStrategy(token, strategyId),
				listBacktests(token).catch(() => [] as BacktestJob[])
			]);
			strategy = stratData;
			backtests = backtestData.sort(
				(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
			);
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to load strategy details.');
			goto('/dashboard/strategies');
		} finally {
			fetching = false;
		}
	}

	function getVariableInfo(code: string): { name: string; category?: string } {
		const found = findStrategyVariable(code);
		return found ? { name: found.name, category: found.category } : { name: code };
	}

	// ── Duplicate Actions ───────────────────────────────────────────────────
	function openDuplicate() {
		if (!strategy) return;
		duplicateNewName = `${strategy.name} (Copy)`;
		duplicateError = '';
		duplicateOpen = true;
	}

	async function handleDuplicate(e: Event) {
		e.preventDefault();
		if (!strategy) return;

		const trimmed = duplicateNewName.trim();
		if (!trimmed) {
			duplicateError = 'Strategy name cannot be empty.';
			return;
		}

		if (trimmed.toLowerCase() === strategy.name.toLowerCase()) {
			duplicateError = 'Please choose a different name for the duplicated strategy.';
			return;
		}

		duplicateLoading = true;
		duplicateError = '';
		try {
			const token = getToken()!;
			const cloned = await duplicateTradingStrategy(token, strategy.id, trimmed);
			duplicateOpen = false;
			toast.success(`Strategy "${cloned.name}" duplicated successfully.`);
			goto(`/dashboard/strategies/${cloned.id}`);
		} catch (err) {
			duplicateError =
				err instanceof ApiError ? err.message : 'Failed to duplicate trading strategy.';
		} finally {
			duplicateLoading = false;
		}
	}

	// ── Delete Actions ──────────────────────────────────────────────────────
	function openDelete() {
		deleteOpen = true;
	}

	async function handleDelete() {
		if (!strategy) return;
		deleteLoading = true;
		try {
			const token = getToken()!;
			await deleteTradingStrategy(token, strategy.id);
			const name = strategy.name;
			deleteOpen = false;
			toast.success(`Strategy "${name}" deleted.`);
			goto('/dashboard/strategies');
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to delete strategy.');
			deleteOpen = false;
		} finally {
			deleteLoading = false;
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

<!-- Breadcrumb / Back Link -->
<div class="mb-4">
	<a
		href="/dashboard/strategies"
		class="btn-interactive font-600 inline-flex items-center gap-1.5 text-xs transition-colors duration-150"
		style="color: var(--fg-muted);"
	>
		<Icon icon="lucide:arrow-left" width="14" height="14" />
		<span>Back to Trading Strategies</span>
	</a>
</div>

{#if fetching}
	<div class="flex min-h-75 items-center justify-center">
		<div class="flex flex-col items-center gap-3">
			<Icon
				icon="lucide:loader-2"
				class="animate-spin"
				style="color: var(--accent);"
				width="32"
				height="32"
			/>
			<p class="font-500 text-sm" style="color: var(--fg-muted)">Loading strategy data…</p>
		</div>
	</div>
{:else if strategy}
	<div class="flex flex-col gap-6">
		<!-- ── Top Strategy Card ─────────────────────────────────────────── -->
		<div
			class="flex flex-col rounded-2xl border p-5 sm:p-6"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
				<!-- Title & Description -->
				<div class="flex flex-col gap-1.5">
					<div class="flex flex-wrap items-center gap-2.5">
						<h1 class="font-700 text-xl sm:text-2xl" style="color: var(--fg)">
							{strategy.name}
						</h1>
					</div>

					{#if !isOwner && strategy.owner}
						<div class="flex items-center gap-2 pt-0.5">
							<div
								class="avatar-glass flex size-6 shrink-0 items-center justify-center rounded-full border shadow-2xs"
								style="color: var(--fg);"
							>
								<Icon icon="lucide:user" width="13" height="13" />
							</div>
							<div class="flex flex-wrap items-center gap-1.5 text-xs">
								<span class="font-600" style="color: var(--fg)">{strategy.owner.name}</span>
								<span class="text-[11px]" style="color: var(--fg-muted)"
									>({strategy.owner.email})</span
								>
							</div>
						</div>
					{/if}

					{#if strategy.description}
						<p class="font-400 max-w-3xl text-sm" style="color: var(--fg-muted)">
							{strategy.description}
						</p>
					{/if}
				</div>

				<!-- Actions: Run Backtest, Duplicate, Edit, Delete -->
				<div class="flex flex-wrap items-center gap-2 self-start">
					{#if isOwner}
						<a
							href="/dashboard/backtests/new"
							class="btn-interactive font-600 flex h-9 items-center gap-1.5 rounded-xl px-3.5 text-xs text-white shadow-sm hover:opacity-95"
							style="background-color: var(--accent);"
						>
							<Icon icon="lucide:play" width="13" height="13" />
							<span>Run Backtest</span>
						</a>
					{/if}

					<button
						type="button"
						onclick={openDuplicate}
						class="btn-interactive font-600 flex h-9 items-center gap-1.5 rounded-xl border px-3 text-xs transition-colors duration-150 hover:bg-(--bg-card-hover)"
						style="border-color: var(--border); color: var(--fg);"
						title="Duplicate strategy"
					>
						<Icon icon="lucide:copy" width="13" height="13" />
						<span>Duplicate</span>
					</button>

					{#if isOwner}
						<a
							href="/dashboard/strategies/{strategy.id}/edit"
							class="btn-interactive font-600 flex h-9 items-center gap-1.5 rounded-xl border px-3 text-xs transition-colors duration-150 hover:bg-(--bg-card-hover)"
							style="border-color: var(--border); color: var(--fg);"
							title="Edit strategy"
						>
							<Icon icon="lucide:pencil" width="13" height="13" />
							<span>Edit</span>
						</a>

						<button
							type="button"
							onclick={openDelete}
							class="btn-interactive flex size-9 items-center justify-center rounded-xl border transition-colors duration-150 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950"
							style="border-color: var(--border); color: var(--fg-muted);"
							title="Delete strategy"
							aria-label="Delete strategy"
						>
							<Icon icon="lucide:trash-2" width="15" height="15" />
						</button>
					{/if}
				</div>
			</div>

			<!-- Parameters Grid -->
			<div
				class="mt-5 grid grid-cols-2 gap-3 rounded-xl p-4 sm:grid-cols-4 sm:gap-4"
				style="background-color: var(--bg-card-hover, var(--bg));"
			>
				<!-- Take Profit -->
				<div class="flex flex-col">
					<span
						class="font-600 text-[11px] tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Take Profit
					</span>
					<span class="font-700 text-base" style="color: var(--success)">
						+{strategy.tp_percentage}%
					</span>
				</div>

				<!-- Stop Loss -->
				<div class="flex flex-col">
					<span
						class="font-600 text-[11px] tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Stop Loss
					</span>
					<span class="font-700 text-base" style="color: var(--danger)">
						-{strategy.sl_percentage}%
					</span>
				</div>

				<!-- Risk Reward -->
				<div class="flex flex-col">
					<span
						class="font-600 text-[11px] tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Risk : Reward
					</span>
					<span class="font-700 text-base" style="color: var(--accent)">
						{formatRiskReward(strategy.tp_percentage, strategy.sl_percentage)}
					</span>
				</div>

				<!-- Max Holding -->
				<div class="flex flex-col">
					<span
						class="font-600 text-[11px] tracking-wider uppercase"
						style="color: var(--fg-muted)"
					>
						Max Holding
					</span>
					<span class="font-700 flex items-center gap-1.5 text-base" style="color: var(--fg)">
						<Icon icon="lucide:clock" width="14" height="14" style="color: var(--fg-muted);" />
						{strategy.max_holding_period_days} Days
					</span>
				</div>
			</div>

			<!-- Metadata Timestamps -->
			<div
				class="font-500 mt-4 flex flex-wrap items-center justify-between gap-2 border-t pt-3 text-[11px]"
				style="border-color: var(--border); color: var(--fg-muted);"
			>
				<div class="inline-flex items-center gap-1.5">
					<Icon icon="lucide:calendar" width="12" height="12" />
					<span>Created {formatTimeAgo(strategy.created_at)}</span>
				</div>
				<div class="inline-flex items-center gap-1.5">
					<Icon icon="lucide:clock" width="12" height="12" />
					<span>Last updated {formatTimeAgo(strategy.updated_at)}</span>
				</div>
			</div>
		</div>

		<!-- ── Strategy Rules & Backtest History (40% : 60%) ─────────────── -->
		<div class="grid grid-cols-1 items-start gap-6 {isOwner ? 'lg:grid-cols-[2fr_3fr]' : ''}">
			<!-- ── Strategy Rules & Conditions ───────────────────────────────── -->
			<div
				class="flex min-w-0 flex-col rounded-2xl border p-5 sm:p-6"
				style="background-color: var(--bg-card); border-color: var(--border);"
			>
				<div class="mb-4 flex items-center gap-2">
					<Icon icon="lucide:code-2" width="18" height="18" style="color: var(--accent);" />
					<h2 class="font-700 text-base sm:text-lg" style="color: var(--fg)">
						Rules & Entry Conditions
					</h2>
				</div>

				{#if strategy.rules && strategy.rules.length > 0}
					<div class="flex flex-col gap-4">
						{#each strategy.rules as group, gIdx (gIdx)}
							{#if gIdx > 0}
								<div class="flex items-center gap-3">
									<div class="h-px flex-1" style="background-color: var(--border);"></div>
									<span
										class="font-800 rounded-lg px-2.5 py-1 font-mono text-xs uppercase shadow-xs"
										style="background-color: var(--accent-soft); color: var(--accent);"
									>
										{strategy.rules[gIdx - 1]?.connector_to_next || 'AND'}
									</span>
									<div class="h-px flex-1" style="background-color: var(--border);"></div>
								</div>
							{/if}

							<div
								class="flex flex-col gap-3 rounded-xl border p-4"
								style="background-color: var(--bg-card-hover, var(--bg)); border-color: var(--border);"
							>
								<div class="flex items-center justify-between">
									<span
										class="font-600 text-xs tracking-wider uppercase"
										style="color: var(--fg-muted)"
									>
										Condition Group #{gIdx + 1}
									</span>
									<span class="font-500 text-xs" style="color: var(--fg-muted)">
										{group.conditions.length}
										{group.conditions.length === 1 ? 'condition' : 'conditions'}
									</span>
								</div>

								<div class="flex flex-col gap-2">
									{#each group.conditions as condition, cIdx (cIdx)}
										{#if cIdx > 0}
											<div class="flex items-center justify-center gap-2 py-0.5">
												<span class="font-700 font-mono text-[11px]" style="color: var(--accent);">
													{group.conditions[cIdx - 1]?.connector_to_next || 'AND'}
												</span>
											</div>
										{/if}

										{@const info = getVariableInfo(condition.variable)}
										<div
											class="grid grid-cols-1 items-center gap-2 rounded-lg border p-3 sm:grid-cols-[1fr_auto_1fr] sm:gap-4"
											style="background-color: var(--bg-card); border-color: var(--border);"
										>
											<!-- Left: Variable -->
											<div class="flex min-w-0 flex-wrap items-center gap-2">
												<span
													class="font-700 rounded-md px-2 py-1 font-mono text-xs"
													style="background-color: var(--accent-soft); color: var(--accent);"
												>
													{condition.variable}
												</span>
												{#if info.name !== condition.variable}
													<span class="font-500 text-xs" style="color: var(--fg)">
														{info.name}
													</span>
												{/if}
											</div>

											<!-- Center: Operator -->
											<div class="flex items-center justify-start sm:justify-center">
												<span
													class="font-700 rounded-md border px-2.5 py-1 font-mono text-xs shadow-2xs"
													style="background-color: var(--bg-card-hover, var(--bg)); border-color: var(--border); color: var(--fg-muted);"
												>
													{condition.operator}
												</span>
											</div>

											<!-- Right: Target Value -->
											<div class="flex items-center justify-start sm:justify-end">
												<span
													class="font-700 rounded-md px-2.5 py-1 font-mono text-xs"
													style="background-color: var(--bg-card-hover, var(--bg)); color: var(--fg);"
												>
													{condition.value}
												</span>
											</div>
										</div>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div
						class="rounded-xl border border-dashed p-8 text-center"
						style="border-color: var(--border);"
					>
						<p class="font-500 text-sm" style="color: var(--fg-muted)">
							No rules configured for this strategy yet.
						</p>
					</div>
				{/if}
			</div>

			<!-- ── Related Backtest Executions ───────────────────────────────── -->
			{#if isOwner}
				<div
					class="flex min-w-0 flex-col rounded-2xl border p-5 sm:p-6"
					style="background-color: var(--bg-card); border-color: var(--border);"
				>
					<div class="mb-4 flex items-center justify-between">
						<div class="flex items-center gap-2">
							<Icon icon="lucide:activity" width="18" height="18" style="color: var(--accent);" />
							<h2 class="font-700 text-base sm:text-lg" style="color: var(--fg)">
								Backtest History
							</h2>
						</div>

						<span class="font-500 text-xs" style="color: var(--fg-muted)">
							{relatedBacktests.length} Backtests
						</span>
					</div>

					{#if relatedBacktests.length > 0}
						<div class="flex flex-col gap-3">
							{#each relatedBacktests as bt (bt.id)}
								<a
									href="/dashboard/backtests/{bt.id}"
									class="btn-interactive group flex flex-col gap-3 rounded-xl border p-4 transition-all duration-150 hover:shadow-xs sm:flex-row sm:items-center sm:justify-between"
									style="background-color: var(--bg-card-hover, var(--bg)); border-color: var(--border);"
								>
									<div class="flex min-w-0 flex-col gap-1">
										<div class="flex flex-wrap items-center gap-2">
											<span class="font-700 truncate text-sm" style="color: var(--fg)">
												{bt.name}
											</span>
											<span
												class="font-600 inline-flex shrink-0 items-center gap-1 rounded-full px-2 py-0.5 text-[11px]"
												style="background-color: var(--bg-card); color: var(--fg)"
											>
												{bt.year}
											</span>
											<StatusBadge status={bt.status} />
										</div>

										<div
											class="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs"
											style="color: var(--fg-muted)"
										>
											<span>Initial: Rp {formatRupiah(bt.initial_cash)}</span>
											<span>•</span>
											<span>{formatDuration(bt.backtest_duration_months)}</span>
											<span>•</span>
											<span>Max {bt.max_holding_stocks} Stocks</span>
										</div>
									</div>

									<div class="flex shrink-0 items-center gap-4 self-end sm:self-center">
										{#if bt.status === 'DONE' && bt.result}
											<div class="flex flex-col text-right">
												<span
													class="font-600 text-[10px] tracking-wider uppercase"
													style="color: var(--fg-muted)"
												>
													Net P/L
												</span>
												<span
													class="font-700 text-sm"
													style="color: {bt.result.net_pnl >= 0
														? 'var(--success)'
														: 'var(--danger)'}"
												>
													{formatPnl(bt.result.net_pnl, bt.result.net_pnl_percentage)}
												</span>
											</div>
										{/if}

										<Icon
											icon="lucide:chevron-right"
											width="16"
											height="16"
											class="transition-transform duration-150 group-hover:translate-x-0.5"
											style="color: var(--fg-muted);"
										/>
									</div>
								</a>
							{/each}
						</div>
					{:else}
						<div
							class="rounded-xl border border-dashed p-8 text-center"
							style="border-color: var(--border);"
						>
							<p class="font-500 text-sm" style="color: var(--fg-muted)">
								No backtests run for this strategy yet.
							</p>
							<a
								href="/dashboard/backtests/new"
								class="btn-interactive font-600 mt-3 inline-flex items-center gap-1.5 text-xs"
								style="color: var(--accent);"
							>
								<Icon icon="lucide:play" width="13" height="13" />
								<span>Launch Backtest</span>
							</a>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>
{/if}

<!-- ── Duplicate Strategy Modal ───────────────────────────────────────────── -->
<Modal bind:open={duplicateOpen} title="Duplicate Trading Strategy">
	{#snippet children()}
		<form id="duplicate-form" onsubmit={handleDuplicate} class="flex flex-col gap-4">
			<p class="font-400 text-sm" style="color: var(--fg-muted)">
				Create a duplicate copy of <b>"{strategy?.name}"</b> with all its rules and parameters. Please
				choose a unique name.
			</p>
			<TextField
				label="New Strategy Name"
				type="text"
				placeholder="Strategy Name"
				bind:value={duplicateNewName}
				required
			/>

			{#if duplicateError}
				<div
					class="font-500 rounded-lg border px-4 py-3 text-sm"
					style="
						background-color: rgba(239, 68, 68, 0.1);
						border-color: rgba(239, 68, 68, 0.3);
						color: var(--danger);
					"
				>
					{duplicateError}
				</div>
			{/if}
		</form>
	{/snippet}

	{#snippet footer()}
		<button
			type="button"
			onclick={() => (duplicateOpen = false)}
			class="btn-interactive font-600 rounded-xl border px-4 py-2 text-xs transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border); color: var(--fg);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="duplicate-form"
			disabled={duplicateLoading || !duplicateNewName.trim()}
			class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-xl px-4 py-2 text-xs text-white shadow-sm hover:opacity-95 disabled:cursor-not-allowed disabled:opacity-50"
			style="background-color: var(--accent);"
		>
			{#if duplicateLoading}
				<Icon icon="lucide:loader-2" class="animate-spin" width="14" height="14" />
				<span>Duplicating…</span>
			{:else}
				<Icon icon="lucide:copy" width="14" height="14" />
				<span>Duplicate Strategy</span>
			{/if}
		</button>
	{/snippet}
</Modal>

<!-- ── Delete Confirmation Modal ──────────────────────────────────── -->
<ConfirmModal
	bind:open={deleteOpen}
	title="Delete Trading Strategy"
	message="Are you sure you want to delete strategy &quot;{strategy?.name}&quot;? This action cannot be undone."
	confirmLabel="Delete Strategy"
	loading={deleteLoading}
	onconfirm={handleDelete}
/>
