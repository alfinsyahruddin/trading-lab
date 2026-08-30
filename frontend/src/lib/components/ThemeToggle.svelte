<script lang="ts">
	import Icon from '@iconify/svelte';
	import { toggleTheme, getCurrentTheme } from '$lib/helpers/theme';
	import { onMount } from 'svelte';

	let theme = $state<'dark' | 'light'>('light');

	onMount(() => {
		theme = getCurrentTheme();
	});

	function handleToggle() {
		theme = toggleTheme();
	}
</script>

<button
	onclick={handleToggle}
	class="btn-interactive flex h-9 w-9 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-(--bg-card-hover)"
	style="color: var(--fg-muted);"
	aria-label="Toggle theme"
	title={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if theme === 'dark'}
		<Icon icon="lucide:sun" width="18" height="18" />
	{:else}
		<Icon icon="lucide:moon" width="18" height="18" />
	{/if}
</button>
