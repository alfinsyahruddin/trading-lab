<script lang="ts">
	import type { Snippet } from 'svelte';
	import { reveal } from '$lib/helpers/reveal';

	interface Props {
		title: string;
		description: string;
		bullets: string[];
		delay?: number;
		isAi?: boolean;
		icon?: Snippet;
	}

	let { title, description, bullets, delay = 0, isAi = false, icon }: Props = $props();
</script>

{#snippet bulletTriangle()}
	<svg class="mt-1 size-2 shrink-0 fill-(--accent)" viewBox="0 0 6 8" aria-hidden="true">
		<polygon points="0.4,1 5.6,4 0.4,7" />
	</svg>
{/snippet}

{#if isAi}
	<article
		use:reveal={{ delay }}
		class="group rounded-2xl bg-[linear-gradient(135deg,#fbaf33_0%,#fe426b_33%,#de98fe_66%,#38c1fb_100%)] p-px transition-all duration-200"
	>
		<div
			class="flex h-full flex-col gap-3 rounded-[15px] p-5 shadow-xs sm:p-6.5"
			style="background: linear-gradient(135deg, rgba(251, 175, 51, 0.1) 0%, rgba(254, 66, 107, 0.1) 33%, rgba(222, 152, 254, 0.1) 66%, rgba(56, 193, 251, 0.1) 100%), var(--bg-card);"
		>
			<div
				class="flex size-11 shrink-0 items-center justify-center rounded-xl border border-[rgba(222,152,254,0.25)]"
				aria-hidden="true"
			>
				{#if icon}
					{@render icon()}
				{/if}
			</div>
			<h3
				class="w-fit bg-[linear-gradient(135deg,#fbaf33_0%,#de98fe_50%,#38c1fb_100%)] bg-clip-text text-lg font-bold tracking-tight text-transparent"
			>
				{title}
			</h3>
			<p class="text-sm leading-relaxed font-light text-(--fg-muted)">
				{description}
			</p>
			<ul class="mt-1 flex flex-col gap-1.5 text-xs text-(--fg-muted)">
				{#each bullets as bullet (bullet)}
					<li class="flex items-start gap-2">
						{@render bulletTriangle()}
						<span>{bullet}</span>
					</li>
				{/each}
			</ul>
		</div>
	</article>
{:else}
	<article
		use:reveal={{ delay }}
		class="group flex flex-col gap-3 rounded-2xl border border-(--border) bg-(--bg-card) p-5 shadow-xs transition-all duration-200 hover:border-(--accent)/40 sm:p-6.5"
	>
		<div
			class="flex size-11 shrink-0 items-center justify-center rounded-xl border border-(--accent)/20 bg-(--accent-soft)"
			aria-hidden="true"
		>
			{#if icon}
				{@render icon()}
			{/if}
		</div>
		<h3 class="text-lg font-bold tracking-tight text-(--fg)">{title}</h3>
		<p class="text-sm leading-relaxed font-light text-(--fg-muted)">
			{description}
		</p>
		<ul class="mt-1 flex flex-col gap-1.5 text-xs text-(--fg-muted)">
			{#each bullets as bullet (bullet)}
				<li class="flex items-start gap-2">
					{@render bulletTriangle()}
					<span>{bullet}</span>
				</li>
			{/each}
		</ul>
	</article>
{/if}
