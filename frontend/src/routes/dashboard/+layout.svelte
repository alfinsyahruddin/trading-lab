<script lang="ts">
	import TopBar from '$lib/components/TopBar.svelte';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="min-h-screen" style="background-color: var(--bg);">
	<TopBar />
	<main class="mx-auto max-w-7xl px-4 py-6 sm:px-6 sm:py-8">
		{@render children()}
	</main>
</div>
