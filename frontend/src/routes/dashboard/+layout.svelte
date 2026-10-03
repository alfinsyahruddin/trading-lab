<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';
	import { fetchStrategyMetadata } from '$lib/helpers/strategy-variables.svelte';

	let { children }: { children: Snippet } = $props();

	onMount(() => {
		fetchStrategyMetadata().catch(() => {
			// Silently ignore if not authorized or network failure; individual pages will handle their own state
		});
	});
</script>

<div class="min-h-screen" style="background-color: var(--bg);">
	<TopBar />
	<main class="mx-auto max-w-7xl px-4 py-6 sm:px-6 sm:py-8">
		{@render children()}
	</main>
</div>
