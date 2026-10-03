<script lang="ts">
	import Icon from '@iconify/svelte';
	import { isComingSoon } from '#lib/helpers/config.js';

	interface Props {
		isLoggedIn: boolean;
		primaryAuthText?: string;
		primaryUnauthText?: string;
		secondaryText?: string;
		size?: 'md' | 'lg';
		oncomingsoon?: () => void;
	}

	let {
		isLoggedIn,
		primaryAuthText = 'Go to Dashboard',
		primaryUnauthText = 'Start Backtesting',
		secondaryText = 'Sign In',
		size = 'md',
		oncomingsoon
	}: Props = $props();

	function handleAuthClick(e: MouseEvent) {
		if (isComingSoon()) {
			e.preventDefault();
			oncomingsoon?.();
		}
	}

	const isLg = $derived(size === 'lg');

	const containerClass = $derived(
		isLg
			? 'flex w-full flex-col gap-2.5 sm:w-auto sm:flex-row sm:flex-wrap sm:items-center sm:justify-center sm:gap-3.5'
			: 'flex w-full flex-col gap-3 pt-1 sm:w-auto sm:flex-row sm:flex-wrap sm:items-center sm:gap-3.5'
	);

	const primaryBtnClass = $derived(
		isLg
			? 'btn-interactive group inline-flex items-center justify-center gap-2 rounded-md bg-(--accent) px-6 py-2.5 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) hover:shadow-md active:scale-97 sm:px-8 sm:py-3.5 sm:text-base'
			: 'btn-interactive group inline-flex items-center justify-center gap-2 rounded-md bg-(--accent) px-7 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) hover:shadow-md active:scale-97'
	);

	const secondaryBtnClass = $derived(
		isLg
			? 'btn-interactive inline-flex items-center justify-center rounded-md border border-(--border) bg-transparent px-5 py-2.5 text-sm font-medium text-(--fg) transition-all hover:border-(--accent)/50 hover:bg-(--accent-soft) active:scale-97 sm:px-6 sm:py-3.5 sm:text-base'
			: 'btn-interactive inline-flex items-center justify-center rounded-md border border-(--border) bg-transparent px-6 py-3 text-sm font-medium text-(--fg) transition-all hover:border-(--accent)/50 hover:bg-(--accent-soft) active:scale-97'
	);

	const arrowIconClass = $derived(
		isLg
			? 'size-4 transition-transform duration-150 group-hover:translate-x-0.5 sm:size-4.5'
			: 'size-4 transition-transform duration-150 group-hover:translate-x-0.5'
	);
</script>

<div class={containerClass}>
	{#if isLoggedIn}
		<a href="/dashboard" class={primaryBtnClass}>
			<span>{primaryAuthText}</span>
			<Icon icon="lucide:arrow-right" class={arrowIconClass} />
		</a>
	{:else}
		<a href="/register" onclick={handleAuthClick} class={primaryBtnClass}>
			<span>{primaryUnauthText}</span>
			<Icon icon="lucide:arrow-right" class={arrowIconClass} />
		</a>
		<a href="/login" onclick={handleAuthClick} class={secondaryBtnClass}>
			{secondaryText}
		</a>
	{/if}
</div>
