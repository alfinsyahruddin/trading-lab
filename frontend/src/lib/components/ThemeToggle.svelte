<script lang="ts">
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
	class="flex h-9 w-9 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-(--bg-card-hover)"
	style="color: var(--fg-muted);"
	aria-label="Toggle theme"
	title={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if theme === 'dark'}
		<!-- Sun icon -->
		<svg
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			<circle cx="12" cy="12" r="5" />
			<path
				d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"
			/>
		</svg>
	{:else}
		<!-- Moon icon -->
		<svg
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			<path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" />
		</svg>
	{/if}
</button>
