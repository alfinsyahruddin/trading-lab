<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import StrategyForm from '$lib/components/strategy/StrategyForm.svelte';
	import { getTradingStrategy, updateTradingStrategy, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { CreateStrategyPayload, TradingStrategy } from '$lib/types';

	let strategy = $state<TradingStrategy | null>(null);
	let fetching = $state(true);
	let loading = $state(false);
	let error = $state('');

	const strategyId = $derived($page.params.id);

	onMount(async () => {
		await loadStrategy();
	});

	async function loadStrategy() {
		fetching = true;
		try {
			const token = getToken();
			if (!token || !strategyId) {
				goto('/dashboard/strategies');
				return;
			}
			strategy = await getTradingStrategy(token, strategyId);
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to load strategy details.');
			goto('/dashboard/strategies');
		} finally {
			fetching = false;
		}
	}

	async function handleUpdate(payload: CreateStrategyPayload) {
		if (!strategy) return;
		loading = true;
		error = '';
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}

			const updated = await updateTradingStrategy(token, strategy.id, payload);
			toast.success(`Trading strategy "${updated.name}" updated successfully.`);
			goto('/dashboard/strategies');
		} catch (err) {
			error = err instanceof ApiError ? err.message : 'Failed to update trading strategy.';
		} finally {
			loading = false;
		}
	}

	function handleCancel() {
		goto('/dashboard/strategies');
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

<!-- Page Title -->
<div class="mb-5 sm:mb-6">
	<h1 class="font-700 text-xl sm:text-2xl" style="color: var(--fg)">Edit Trading Strategy</h1>
	<p class="mt-0.5 text-xs sm:text-sm" style="color: var(--fg-muted)">
		Modify risk parameters, targets, and condition rules for this strategy.
	</p>
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
	<StrategyForm
		initialData={strategy}
		{loading}
		{error}
		submitLabel="Save Changes"
		onsubmit={handleUpdate}
		oncancel={handleCancel}
	/>
{/if}
