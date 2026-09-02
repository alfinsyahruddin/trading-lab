<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import StrategyForm from '$lib/components/strategy/StrategyForm.svelte';
	import { createTradingStrategy, getSettings, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { CreateStrategyPayload } from '$lib/types';

	let loading = $state(false);
	let error = $state('');
	let aiEnabled = $state(false);

	onMount(async () => {
		try {
			const token = getToken();
			if (token) {
				const settings = await getSettings(token);
				aiEnabled = settings.ai_enabled;
			}
		} catch {
			// Silently fallback to aiEnabled = false
		}
	});

	async function handleCreate(payload: CreateStrategyPayload) {
		loading = true;
		error = '';
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}

			const created = await createTradingStrategy(token, payload);
			toast.success(`Trading strategy "${created.name}" created successfully.`);
			goto('/dashboard/strategies');
		} catch (err) {
			error = err instanceof ApiError ? err.message : 'Failed to create trading strategy.';
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
	<h1 class="font-700 text-xl sm:text-2xl" style="color: var(--fg)">Create Trading Strategy</h1>
	<p class="mt-0.5 text-xs sm:text-sm" style="color: var(--fg-muted)">
		Configure target profit, stop loss, and multi-group conditional rules.
	</p>
</div>

<!-- Strategy Form -->
<StrategyForm
	{loading}
	{error}
	submitLabel="Create Strategy"
	enableAiSuggestions={aiEnabled}
	onsubmit={handleCreate}
	oncancel={handleCancel}
/>
