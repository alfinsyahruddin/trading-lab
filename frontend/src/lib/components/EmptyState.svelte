<script lang="ts">
	import type { Snippet } from 'svelte';

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
			<svg
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
			>
				<path
					d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
				/>
			</svg>
		{/if}
	</div>
	<p class="text-sm font-500" style="color: var(--fg-muted)">{message}</p>
	{#if children}
		<div class="mt-4">
			{@render children()}
		</div>
	{/if}
</div>
