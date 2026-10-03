<script lang="ts" generics="T extends string | boolean | number">
	import Icon from '@iconify/svelte';
	import type { SegmentOption } from '#lib/types.js';

	let {
		options,
		value = $bindable(),
		size = 'md',
		class: className = '',
		isInsideCard = true,
		onchange
	}: {
		options: SegmentOption<T>[];
		value: T;
		size?: 'sm' | 'md';
		class?: string;
		isInsideCard?: boolean;
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
	class="ios-segmented-track relative flex items-center p-0.5 select-none {size === 'sm'
		? 'h-7 rounded-[7px]'
		: 'h-8.5 rounded-[9px]'} {className || 'w-full'}"
	style={!isInsideCard ? '--segmented-light-bg: #e4e6ec;' : undefined}
	role="group"
>
	<!-- Sliding iOS Neutral Pill Indicator -->
	<div
		class="ios-segmented-thumb pointer-events-none absolute {size === 'sm'
			? 'rounded-[5px]'
			: 'rounded-[7px]'}"
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
	{#each options as opt, idx (opt.value)}
		{@const isSelected = opt.value === value}

		<!-- Optional Separator divider line between unselected segments -->
		{#if idx > 0}
			<div
				class="pointer-events-none absolute w-px transition-opacity duration-200 {size === 'sm'
					? 'h-3'
					: 'h-3.5'}"
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
			class="relative z-10 flex h-full flex-1 items-center justify-center gap-1.5 whitespace-nowrap transition-colors duration-150 focus:outline-none {size ===
			'sm'
				? 'rounded-[5px] px-2 text-xs'
				: 'rounded-[7px] px-3 text-[13px]'}"
			style="
				color: {isSelected ? 'var(--fg)' : 'var(--fg-muted)'};
				font-weight: 500;
			"
			aria-pressed={isSelected}
		>
			{#if opt.icon}
				<Icon icon={opt.icon} width={size === 'sm' ? 12 : 14} height={size === 'sm' ? 12 : 14} />
			{/if}
			<span class="whitespace-nowrap">{opt.label}</span>
		</button>
	{/each}
</div>

<style>
	.ios-segmented-track {
		background-color: var(--segmented-light-bg, var(--bg));
		border: 1px solid var(--border, #e2e8f0);
	}

	:global(.dark) .ios-segmented-track,
	:global(html.dark) .ios-segmented-track,
	:root.dark .ios-segmented-track {
		background-color: #2d3857;
		border: 1px solid var(--border, rgba(255, 255, 255, 0.1));
	}

	.ios-segmented-thumb {
		background-color: #ffffff;
		box-shadow:
			0px 2px 5px 0px rgba(0, 0, 0, 0.08),
			0px 1px 2px 0px rgba(0, 0, 0, 0.04);
		border: 0.5px solid rgba(0, 0, 0, 0.06);
	}

	:global(.dark) .ios-segmented-thumb,
	:global(html.dark) .ios-segmented-thumb,
	:root.dark .ios-segmented-thumb {
		background-color: #3c486a;
		box-shadow:
			0px 3px 8px 0px rgba(0, 0, 0, 0.35),
			0px 1px 3px 0px rgba(0, 0, 0, 0.2);
		border: 0.5px solid rgba(255, 255, 255, 0.15);
	}
</style>
