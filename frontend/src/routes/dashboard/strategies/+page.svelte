<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import Modal from '$lib/components/Modal.svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import TextField from '$lib/components/TextField.svelte';
	import {
		listTradingStrategies,
		deleteTradingStrategy,
		duplicateTradingStrategy,
		ApiError
	} from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import {
		STRATEGY_VARIABLES,
		formatRiskReward,
		formatTimeAgo,
		type StrategyVariableOption
	} from '$lib/constants';
	import type { StrategyRuleGroup, TradingStrategy } from '$lib/types';

	let strategies = $state<TradingStrategy[]>([]);
	let loading = $state(true);

	// Duplicate modal state
	let duplicateOpen = $state(false);
	let duplicateLoading = $state(false);
	let duplicateError = $state('');
	let duplicateTarget = $state<TradingStrategy | null>(null);
	let duplicateNewName = $state('');

	// Delete modal state
	let deleteOpen = $state(false);
	let deleteLoading = $state(false);
	let deleteTarget = $state<TradingStrategy | null>(null);

	onMount(async () => {
		await loadStrategies();
	});

	async function loadStrategies() {
		loading = true;
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}
			const data = await listTradingStrategies(token);
			strategies = [...data].sort(
				(a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
			);
		} catch (err) {
			if (err instanceof ApiError) {
				toast.error(err.message);
			} else {
				toast.error('Failed to load trading strategies.');
			}
		} finally {
			loading = false;
		}
	}

	function getVariableName(code: string): string {
		const found = STRATEGY_VARIABLES.find((v: StrategyVariableOption) => v.code === code);
		return found ? found.name : code;
	}

	// ── Duplicate Actions ───────────────────────────────────────────────────
	function openDuplicate(strategy: TradingStrategy) {
		duplicateTarget = strategy;
		duplicateNewName = `${strategy.name} (Copy)`;
		duplicateError = '';
		duplicateOpen = true;
	}

	async function handleDuplicate(e: Event) {
		e.preventDefault();
		if (!duplicateTarget) return;

		const trimmed = duplicateNewName.trim();
		if (!trimmed) {
			duplicateError = 'Strategy name cannot be empty.';
			return;
		}

		if (trimmed.toLowerCase() === duplicateTarget.name.toLowerCase()) {
			duplicateError = 'Please choose a different name for the duplicated strategy.';
			return;
		}

		duplicateLoading = true;
		duplicateError = '';
		try {
			const token = getToken()!;
			const cloned = await duplicateTradingStrategy(token, duplicateTarget.id, trimmed);
			strategies = [cloned, ...strategies];
			duplicateOpen = false;
			toast.success(`Strategy "${cloned.name}" created.`);
		} catch (err) {
			duplicateError =
				err instanceof ApiError ? err.message : 'Failed to duplicate trading strategy.';
		} finally {
			duplicateLoading = false;
		}
	}

	// ── Delete Actions ──────────────────────────────────────────────────────
	function openDelete(strategy: TradingStrategy) {
		deleteTarget = strategy;
		deleteOpen = true;
	}

	async function handleDelete() {
		if (!deleteTarget) return;
		deleteLoading = true;
		try {
			const token = getToken()!;
			await deleteTradingStrategy(token, deleteTarget.id);
			const name = deleteTarget.name;
			strategies = strategies.filter((s) => s.id !== deleteTarget!.id);
			deleteOpen = false;
			deleteTarget = null;
			toast.success(`Strategy "${name}" deleted.`);
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to delete strategy.');
			deleteOpen = false;
		} finally {
			deleteLoading = false;
		}
	}
</script>

<!-- Page Header -->
<div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
	<div>
		<h1 class="text-2xl font-700" style="color: var(--fg)">Trading Strategy</h1>
		<p class="mt-0.5 text-sm" style="color: var(--fg-muted)">
			Build, backtest, and deploy algorithmic trading strategies with multi-group rules.
		</p>
	</div>
	<a
		href="/dashboard/strategies/new"
		class="btn-interactive inline-flex items-center justify-center gap-2 rounded-xl px-4 py-2.5 text-sm font-600 text-white shadow-sm hover:shadow-md hover:opacity-95 active:scale-95 w-full sm:w-auto"
		style="background-color: var(--accent);"
	>
		<Icon icon="lucide:plus" width="16" height="16" />
		<span>Create Strategy</span>
	</a>
</div>

<!-- Strategy Count Info -->
{#if !loading}
	<p class="mb-4 text-xs font-600 uppercase tracking-wider" style="color: var(--fg-muted)">
		{strategies.length}
		{strategies.length === 1 ? 'strategy' : 'strategies'}
	</p>
{/if}

<!-- Main Content Area -->
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
			<p class="text-sm font-500" style="color: var(--fg-muted)">Loading strategies…</p>
		</div>
	</div>
{:else if strategies.length === 0}
	<!-- Empty State -->
	<div
		class="flex min-h-100 flex-col items-center justify-center rounded-2xl border p-8 text-center"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<div
			class="animate-float mb-4 flex h-16 w-16 items-center justify-center rounded-2xl"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:candlestick-chart" width="32" height="32" />
		</div>
		<h2 class="text-lg font-700" style="color: var(--fg)">No Trading Strategies Yet</h2>
		<p class="mt-1 max-w-md text-sm font-400" style="color: var(--fg-muted)">
			Create your first algorithmic strategy by specifying entry rules, take-profit targets,
			stop-loss limits, and holding periods.
		</p>
		<a
			href="/dashboard/strategies/new"
			class="btn-interactive mt-6 inline-flex items-center gap-2 rounded-xl px-5 py-2.5 text-sm font-600 text-white shadow-sm"
			style="background-color: var(--accent);"
		>
			<Icon icon="lucide:plus" width="16" height="16" />
			<span>Create Strategy</span>
		</a>
	</div>
{:else}
	<!-- Strategy Cards List (NOT a table) -->
	<div class="flex flex-col gap-4">
		{#each strategies as strategy (strategy.id)}
			<div
				class="group flex flex-col rounded-2xl border p-4 sm:p-5 transition-colors duration-200"
				style="background-color: var(--bg-card); border-color: var(--border);"
			>
				<!-- Top Row: Title + Visibility Badge + Action Buttons -->
				<div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
					<!-- Title & Visibility -->
					<div class="flex flex-col gap-1">
						<div class="flex items-center gap-2 flex-wrap">
							<h3 class="text-base sm:text-lg font-700 leading-snug" style="color: var(--fg)">
								{strategy.name}
							</h3>

							<!-- Public / Private Status Icon Badge -->
							{#if strategy.is_public}
								<span
									class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-600"
									style="background-color: var(--accent-soft); color: var(--accent);"
									title="Public strategy"
								>
									<Icon icon="lucide:globe" width="12" height="12" />
									<span>Public</span>
								</span>
							{:else}
								<span
									class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-600"
									style="background-color: rgba(148, 163, 184, 0.15); color: var(--fg-muted);"
									title="Private strategy"
								>
									<Icon icon="lucide:lock" width="12" height="12" />
									<span>Private</span>
								</span>
							{/if}
						</div>

						{#if strategy.description}
							<p class="text-xs sm:text-sm font-400 line-clamp-2" style="color: var(--fg-muted)">
								{strategy.description}
							</p>
						{/if}
					</div>

					<!-- Actions: Duplicate, Edit, Delete Buttons -->
					<div class="flex items-center gap-1.5 self-start flex-wrap">
						<!-- Duplicate Button -->
						<button
							type="button"
							onclick={() => openDuplicate(strategy)}
							class="btn-interactive flex h-8 items-center gap-1.5 rounded-lg border px-2.5 text-xs font-600 transition-colors duration-150 hover:bg-(--bg-card-hover)"
							style="border-color: var(--border); color: var(--fg);"
							title="Duplicate strategy"
							aria-label="Duplicate {strategy.name}"
						>
							<Icon icon="lucide:copy" width="13" height="13" />
							<span class="hidden sm:inline">Duplicate</span>
						</button>

						<!-- Edit Button -->
						<a
							href="/dashboard/strategies/{strategy.id}/edit"
							class="btn-interactive flex h-8 items-center gap-1.5 rounded-lg border px-2.5 text-xs font-600 transition-colors duration-150 hover:bg-(--bg-card-hover)"
							style="border-color: var(--border); color: var(--fg);"
							title="Edit strategy"
							aria-label="Edit {strategy.name}"
						>
							<Icon icon="lucide:pencil" width="13" height="13" />
							<span class="hidden sm:inline">Edit</span>
						</a>

						<!-- Delete Button -->
						<button
							type="button"
							onclick={() => openDelete(strategy)}
							class="btn-interactive flex h-8 w-8 items-center justify-center rounded-lg border transition-colors duration-150 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950"
							style="border-color: var(--border); color: var(--fg-muted);"
							title="Delete strategy"
							aria-label="Delete {strategy.name}"
						>
							<Icon icon="lucide:trash-2" width="14" height="14" />
						</button>
					</div>
				</div>

				<!-- Middle Row: Strategy Key Performance Parameters Grid -->
				<div
					class="mt-4 grid grid-cols-2 gap-2 sm:grid-cols-4 sm:gap-3 rounded-xl p-3"
					style="background-color: var(--bg-card-hover, var(--bg));"
				>
					<!-- TP -->
					<div class="flex flex-col">
						<span
							class="text-[11px] font-600 uppercase tracking-wider"
							style="color: var(--fg-muted)"
						>
							Take Profit
						</span>
						<span class="text-sm font-700" style="color: var(--success)">
							+{strategy.tp_percentage}%
						</span>
					</div>

					<!-- SL -->
					<div class="flex flex-col">
						<span
							class="text-[11px] font-600 uppercase tracking-wider"
							style="color: var(--fg-muted)"
						>
							Stop Loss
						</span>
						<span class="text-sm font-700" style="color: var(--danger)">
							-{strategy.sl_percentage}%
						</span>
					</div>

					<!-- Risk Reward Ratio -->
					<div class="flex flex-col">
						<span
							class="text-[11px] font-600 uppercase tracking-wider"
							style="color: var(--fg-muted)"
						>
							Risk : Reward
						</span>
						<span class="text-sm font-700" style="color: var(--accent)">
							{formatRiskReward(strategy.tp_percentage, strategy.sl_percentage)}
						</span>
					</div>

					<!-- Max Holding Period -->
					<div class="flex flex-col">
						<span
							class="text-[11px] font-600 uppercase tracking-wider"
							style="color: var(--fg-muted)"
						>
							Max Holding
						</span>
						<span class="flex items-center gap-1 text-sm font-700" style="color: var(--fg)">
							<Icon icon="lucide:clock" width="13" height="13" style="color: var(--fg-muted);" />
							{strategy.max_holding_period_days} Days
						</span>
					</div>
				</div>

				<!-- Bottom Row: Rules Tags Preview (Left) + Timestamps (Right) -->
				<div
					class="mt-4 flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between pt-3 border-t"
					style="border-color: var(--border);"
				>
					<!-- Left: Rules & Conditions Tags -->
					{#if strategy.rules && strategy.rules.length > 0}
						<div class="flex flex-1 flex-col gap-1.5 min-w-0">
							<div
								class="flex items-center gap-1.5 text-xs font-600"
								style="color: var(--fg-muted)"
							>
								<Icon icon="lucide:code-2" width="13" height="13" />
								<span>Rules & Conditions:</span>
							</div>

							<div class="flex flex-wrap items-center gap-1.5">
								{#each strategy.rules as group, gIdx}
									{#if gIdx > 0}
										<span
											class="rounded px-1.5 py-0.5 text-[10px] font-800 font-mono uppercase"
											style="background-color: var(--accent-soft); color: var(--accent);"
										>
											{strategy.rules[gIdx - 1]?.connector_to_next || 'AND'}
										</span>
									{/if}

									<div
										class="inline-flex flex-wrap items-center gap-1 rounded-xl border px-2 py-1"
										style="border-color: var(--border); background-color: var(--bg-card);"
									>
										{#each group.conditions as condition, cIdx}
											{#if cIdx > 0}
												<span class="text-[10px] font-700 font-mono" style="color: var(--accent);">
													{group.conditions[cIdx - 1]?.connector_to_next || 'AND'}
												</span>
											{/if}

											<span
												class="inline-flex items-center gap-1 rounded-md px-1.5 py-0.5 text-xs font-500"
												style="
													background-color: var(--bg-card-hover, var(--bg));
													color: var(--fg);
												"
											>
												<span class="font-600 font-mono text-[11px]" style="color: var(--accent);">
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
						class="flex flex-col sm:items-end gap-0.5 text-[11px] font-500 shrink-0 text-left sm:text-right"
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
		{/each}
	</div>
{/if}

<!-- ── Duplicate Strategy Modal ───────────────────────────────────────────── -->
<Modal bind:open={duplicateOpen} title="Duplicate Trading Strategy">
	{#snippet children()}
		<form id="duplicate-form" onsubmit={handleDuplicate} class="flex flex-col gap-4">
			<p class="text-sm font-400" style="color: var(--fg-muted)">
				Create a duplicate copy of <b>"{duplicateTarget?.name}"</b> with all its rules and parameters.
				Please choose a unique name.
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
					class="rounded-lg border px-4 py-3 text-sm font-500"
					style="background-color: rgba(239,68,68,0.08); border-color: rgba(239,68,68,0.3); color: var(--danger);"
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
			class="btn-interactive w-full sm:w-auto rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="duplicate-form"
			disabled={duplicateLoading || !duplicateNewName.trim()}
			class="btn-interactive w-full sm:w-auto inline-flex items-center justify-center gap-1.5 rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
			style="background-color: var(--accent); opacity: {duplicateLoading ? '0.7' : '1'};"
		>
			<Icon icon="lucide:copy" width="15" height="15" />
			<span>{duplicateLoading ? 'Duplicating…' : 'Duplicate Strategy'}</span>
		</button>
	{/snippet}
</Modal>

<!-- ── Delete Strategy Confirm Modal ──────────────────────────────────────── -->
<ConfirmModal
	bind:open={deleteOpen}
	title="Delete Trading Strategy"
	message={`Are you sure you want to delete strategy "${deleteTarget?.name}"? This action cannot be undone.`}
	confirmLabel="Delete Strategy"
	loading={deleteLoading}
	onconfirm={handleDelete}
/>
