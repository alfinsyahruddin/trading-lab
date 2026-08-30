<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from '@iconify/svelte';

	let {
		message = 'No items found.',
		icon,
		children
	}: {
		message?: string;
		icon?: Snippet;
		children?: Snippet;
	} = $props();
</script>

<div class="flex flex-col items-center justify-center py-16 text-center">
	<div class="mb-4 opacity-40" style="color: var(--fg-muted)">
		{#if icon}
			{@render icon()}
		{:else}
			<Icon icon="lucide:clipboard-list" width="48" height="48" />
		{/if}
	</div>
	<p class="font-500 text-sm" style="color: var(--fg-muted)">{message}</p>
	{#if children}
		<div class="mt-4">
			{@render children()}
		</div>
	{/if}
</div>
