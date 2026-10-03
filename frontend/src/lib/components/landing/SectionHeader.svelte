<script lang="ts">
	import type { Snippet } from 'svelte';
	import { reveal } from '#lib/helpers/reveal.js';

	interface Props {
		tag: string;
		title?: string;
		description?: string;
		titleSnippet?: Snippet;
	}

	let { tag, title, description, titleSnippet }: Props = $props();
</script>

<div class="mb-10 text-center sm:mb-14" use:reveal>
	<p class="font-mono text-xs font-semibold tracking-widest text-(--accent) uppercase">
		{tag}
	</p>
	<h2
		class="mt-2 text-2xl font-extrabold tracking-tight text-(--fg) sm:mt-2.5 sm:text-4xl lg:text-5xl"
	>
		{#if titleSnippet}
			{@render titleSnippet()}
		{:else}
			{title}
		{/if}
	</h2>
	{#if description}
		<p class="mx-auto mt-3 max-w-xl text-sm text-(--fg-muted) sm:mt-3.5 sm:text-base">
			{description}
		</p>
	{/if}
</div>
