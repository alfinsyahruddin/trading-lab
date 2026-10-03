<script lang="ts">
	import { reveal } from '#lib/helpers/reveal.js';

	interface Props {
		lightSrc: string;
		darkSrc: string;
		alt: string;
		tag: string;
		description: string;
		delay?: number;
		containerClass?: string;
		imageBorderClass?: string;
		tagClass?: string;
		width?: number;
		height?: number;
	}

	let {
		lightSrc,
		darkSrc,
		alt,
		tag,
		description,
		delay = 0,
		containerClass = '',
		imageBorderClass = 'border-(--border) shadow-none dark:shadow-xl',
		tagClass = 'text-(--accent)',
		width,
		height
	}: Props = $props();
</script>

<div use:reveal={{ delay }} class={containerClass}>
	<div class="overflow-hidden rounded-xl border bg-(--bg-card) {imageBorderClass}">
		<img
			src={lightSrc}
			{alt}
			{width}
			{height}
			class="block h-auto w-full dark:hidden"
			loading="lazy"
			decoding="async"
		/>
		<img
			src={darkSrc}
			{alt}
			{width}
			{height}
			class="hidden h-auto w-full dark:block"
			loading="lazy"
			decoding="async"
		/>
	</div>
	<div class="mt-3 flex flex-col gap-0.5 max-md:mb-0 sm:mt-3.5 sm:gap-1">
		<span class="font-mono text-xs font-semibold tracking-wider uppercase {tagClass}">
			{tag}
		</span>
		<p class="text-sm text-(--fg-muted)">
			{description}
		</p>
	</div>
</div>
