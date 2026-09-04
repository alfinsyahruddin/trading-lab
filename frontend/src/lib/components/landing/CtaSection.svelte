<script lang="ts">
	import { reveal } from '$lib/helpers/reveal';
	import LandingCtaButtons from './LandingCtaButtons.svelte';

	interface Props {
		isLoggedIn: boolean;
	}

	let { isLoggedIn }: Props = $props();
</script>

<section
	id="cta-section"
	use:reveal
	class="relative flex min-h-[45vh] flex-col items-center justify-center overflow-hidden border-t border-(--border) bg-(--bg-card) px-4 py-16 text-center sm:min-h-[52vh] sm:px-6 sm:py-24"
>
	<!-- Inner Blueprint / Grid Box Frame Pattern -->
	<div
		use:reveal
		class="pointer-events-none absolute inset-3 border border-(--border) bg-[linear-gradient(to_right,rgba(0,0,0,0.035)_1px,transparent_1px),linear-gradient(to_bottom,rgba(0,0,0,0.035)_1px,transparent_1px)] bg-size-[32px_32px] transition-opacity duration-1000 ease-out sm:inset-x-[max(5vw,1.5rem)] sm:inset-y-[12%] dark:bg-[linear-gradient(to_right,rgba(255,255,255,0.025)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,0.025)_1px,transparent_1px)]"
		aria-hidden="true"
	></div>

	<div
		use:reveal={{ delay: 100 }}
		class="relative z-10 mx-auto flex max-w-xl flex-col items-center gap-5 sm:gap-6"
	>
		<h2 class="text-2xl font-extrabold tracking-tight text-(--fg) sm:text-4xl lg:text-5xl">
			Stop Guessing.<br /><em class="font-bold text-(--accent) not-italic">Start Backtesting.</em>
		</h2>
		<p class="text-sm text-(--fg-muted) sm:text-base">
			Build your strategy. Test your edge. Trade with confidence.
		</p>
		<LandingCtaButtons
			{isLoggedIn}
			primaryAuthText="Open Dashboard"
			primaryUnauthText="Create Free Account"
			secondaryText="Sign In"
			size="lg"
		/>
	</div>
</section>
