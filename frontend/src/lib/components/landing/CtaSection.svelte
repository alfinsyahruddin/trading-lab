<script lang="ts">
	import { reveal } from '#lib/helpers/reveal.js';
	import LandingCtaButtons from './LandingCtaButtons.svelte';

	interface Props {
		isLoggedIn: boolean;
		oncomingsoon?: () => void;
	}

	let { isLoggedIn, oncomingsoon }: Props = $props();
</script>

<section
	id="cta-section"
	use:reveal
	class="relative flex min-h-[45vh] flex-col items-center justify-center overflow-hidden bg-(--bg-card) px-4 py-16 text-center sm:min-h-[52vh] sm:px-6 sm:py-24"
>
	<!-- Squares pattern background with vignette transparent effect in the sides -->
	<div
		use:reveal
		class="pointer-events-none absolute inset-0 bg-[linear-gradient(to_right,rgba(0,0,0,0.035)_1px,transparent_1px),linear-gradient(to_bottom,rgba(0,0,0,0.035)_1px,transparent_1px)] bg-size-[32px_32px] transition-opacity duration-1000 ease-out dark:bg-[linear-gradient(to_right,rgba(255,255,255,0.025)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,0.025)_1px,transparent_1px)]"
		style="-webkit-mask-image: radial-gradient(ellipse at center, black 30%, transparent 75%); mask-image: radial-gradient(ellipse at center, black 30%, transparent 75%);"
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
			{oncomingsoon}
		/>
	</div>
</section>
