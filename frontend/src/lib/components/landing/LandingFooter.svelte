<script lang="ts">
	import { reveal } from '$lib/helpers/reveal';
	import { isComingSoon } from '$lib/helpers/config';
	import LandingLogo from './LandingLogo.svelte';

	interface Props {
		isLoggedIn: boolean;
		oncomingsoon?: () => void;
	}

	let { isLoggedIn, oncomingsoon }: Props = $props();

	function handleAuthClick(e: MouseEvent) {
		if (isComingSoon()) {
			e.preventDefault();
			oncomingsoon?.();
		}
	}
</script>

<footer
	id="footer-section"
	use:reveal
	class="border-t border-(--border) bg-(--bg) px-4 py-8 sm:px-6"
>
	<div
		use:reveal={{ delay: 50 }}
		class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 sm:flex-row"
	>
		<div class="flex items-center">
			<LandingLogo size="sm" />
		</div>
		<p class="text-center text-xs text-(--fg-muted)">
			© Copyright {new Date().getFullYear()} Trading Lab by Alphabyte. All rights reserved.
		</p>
		<nav class="flex items-center gap-5" aria-label="Footer navigation">
			{#if isLoggedIn}
				<a
					href="/dashboard"
					class="text-xs text-(--fg-muted) transition-colors hover:text-(--accent)"
				>
					Dashboard
				</a>
			{:else}
				<a
					href="/login"
					onclick={handleAuthClick}
					class="text-xs text-(--fg-muted) transition-colors hover:text-(--accent)"
				>
					Sign In
				</a>
				<a
					href="/register"
					onclick={handleAuthClick}
					class="text-xs text-(--fg-muted) transition-colors hover:text-(--accent)"
				>
					Get Started
				</a>
			{/if}
		</nav>
	</div>
</footer>
