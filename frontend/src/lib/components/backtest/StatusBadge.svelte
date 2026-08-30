<script lang="ts">
	import type { BacktestStatus } from '$lib/types';

	let { status }: { status: BacktestStatus } = $props();

	const styleMap = {
		PENDING: { bg: 'rgba(148, 163, 184, 0.15)', text: 'var(--fg-muted)', pulse: false },
		PROCESSING: { bg: 'rgba(245, 158, 11, 0.15)', text: 'var(--warning)', pulse: true },
		DONE: { bg: 'rgba(16, 185, 129, 0.15)', text: 'var(--success)', pulse: false },
		FAILED: { bg: 'rgba(239, 68, 68, 0.15)', text: 'var(--danger)', pulse: false }
	};

	const currentStyle = $derived(styleMap[status]);
</script>

<span
	class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-600 {currentStyle.pulse
		? 'animate-pulse'
		: ''}"
	style="background-color: {currentStyle.bg}; color: {currentStyle.text};"
>
	{#if status === 'PENDING'}
		<span
			class="h-1.5 w-1.5 rounded-full"
			style="background-color: {currentStyle.text}; opacity: 0.7;"
		></span>
	{:else if status === 'PROCESSING'}
		<span
			class="h-1.5 w-1.5 rounded-full animate-bounce"
			style="background-color: {currentStyle.text};"
		></span>
	{:else if status === 'DONE'}
		<span class="h-1.5 w-1.5 rounded-full" style="background-color: {currentStyle.text};"></span>
	{:else if status === 'FAILED'}
		<span class="h-1.5 w-1.5 rounded-full" style="background-color: {currentStyle.text};"></span>
	{/if}
	<span>{status}</span>
</span>
