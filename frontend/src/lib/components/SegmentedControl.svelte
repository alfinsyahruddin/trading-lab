<script lang="ts" generics="T extends string | boolean | number">
	import Icon from '@iconify/svelte';
	import type { SegmentOption } from '$lib/types';

	let {
		options,
		value = $bindable(),
		onchange
	}: {
		options: SegmentOption<T>[];
		value: T;
		onchange?: (newValue: T) => void;
	} = $props();

	const selectedIndex = $derived(
		Math.max(
			0,
			options.findIndex((opt) => opt.value === value)
		)
	);

	function handleSelect(optionValue: T) {
		value = optionValue;
		onchange?.(optionValue);
	}
</script>

<div
	class="ios-segmented-track relative inline-flex h-8.5 items-center rounded-[9px] p-0.5 select-none"
	role="group"
>
	<!-- Sliding iOS Neutral Pill Indicator -->
	<div
		class="ios-segmented-thumb absolute rounded-[7px] pointer-events-none"
		style="
			top: 2px;
			bottom: 2px;
			left: 2px;
			width: calc((100% - 4px) / {options.length});
			transform: translate3d(calc({selectedIndex} * 100%), 0, 0);
			transition: transform 0.28s cubic-bezier(0.32, 0.72, 0, 1);
			will-change: transform;
		"
	></div>

	<!-- Segment Buttons -->
	{#each options as opt, idx}
		{@const isSelected = opt.value === value}

		<!-- Optional Separator divider line between unselected segments -->
		{#if idx > 0}
			<div
				class="pointer-events-none absolute h-3.5 w-px transition-opacity duration-200"
				style="
					left: calc({idx} * (100% / {options.length}));
					background-color: var(--border-strong);
					opacity: {selectedIndex === idx || selectedIndex === idx - 1 ? '0' : '0.6'};
				"
			></div>
		{/if}

		<button
			type="button"
			onclick={() => handleSelect(opt.value)}
			class="relative z-10 flex h-full flex-1 items-center justify-center gap-1.5 rounded-[7px] px-3 text-[13px] transition-colors duration-150 focus:outline-none"
			style="
				color: {isSelected ? 'var(--fg)' : 'var(--fg-muted)'};
				font-weight: 500;
			"
			aria-pressed={isSelected}
		>
			{#if opt.icon}
				<Icon icon={opt.icon} width="14" height="14" />
			{/if}
			<span>{opt.label}</span>
		</button>
	{/each}
</div>

<style>
	.ios-segmented-track {
		background-color: rgba(118, 118, 128, 0.12);
		border: 1px solid rgba(0, 0, 0, 0.04);
	}

	:global(.dark) .ios-segmented-track,
	:root.dark .ios-segmented-track {
		background-color: rgba(118, 118, 128, 0.24);
		border: 1px solid rgba(255, 255, 255, 0.06);
	}

	.ios-segmented-thumb {
		background-color: #ffffff;
		box-shadow:
			0px 3px 8px 0px rgba(0, 0, 0, 0.12),
			0px 3px 1px 0px rgba(0, 0, 0, 0.04);
		border: 0.5px solid rgba(0, 0, 0, 0.04);
	}

	:global(.dark) .ios-segmented-thumb,
	:global(html.dark) .ios-segmented-thumb,
	:root.dark .ios-segmented-thumb {
		background-color: rgba(255, 255, 255, 0.2);
		box-shadow:
			0px 3px 8px 0px rgba(0, 0, 0, 0.3),
			0px 1px 2px 0px rgba(0, 0, 0, 0.15);
		border: 0.5px solid rgba(255, 255, 255, 0.12);
	}
</style>
