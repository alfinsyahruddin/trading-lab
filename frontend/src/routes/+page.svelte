<script lang="ts">
	import { onMount } from 'svelte';
	import { getToken } from '$lib/helpers/session';
	import { cleanupRevealEngine } from '$lib/helpers/reveal';
	import ComingSoonModal from '$lib/components/ComingSoonModal.svelte';
	import HeroSection from '$lib/components/landing/HeroSection.svelte';
	import FeaturesSection from '$lib/components/landing/FeaturesSection.svelte';
	import ScreenshotsSection from '$lib/components/landing/ScreenshotsSection.svelte';
	import HowItWorksSection from '$lib/components/landing/HowItWorksSection.svelte';
	import CtaSection from '$lib/components/landing/CtaSection.svelte';
	import LandingFooter from '$lib/components/landing/LandingFooter.svelte';

	let isLoggedIn = $state(false);
	let showComingSoonModal = $state(false);

	function openComingSoon() {
		showComingSoonModal = true;
	}

	onMount(() => {
		isLoggedIn = !!getToken();

		return () => {
			cleanupRevealEngine();
		};
	});
</script>

<svelte:head>
	<title>Trading Lab | Alphabyte</title>
	<meta
		name="description"
		content="Build, simulate, and refine quantitative trading strategies with real IDX historical data without code."
	/>
	<meta name="robots" content="index, follow" />
	<link rel="canonical" href="https://trading-lab.xyz" />

	<!-- Open Graph / Facebook -->
	<meta property="og:site_name" content="Trading Lab" />
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://trading-lab.xyz" />
	<meta property="og:title" content="Trading Lab | Alphabyte" />
	<meta
		property="og:description"
		content="Build, simulate, and refine quantitative trading strategies with real IDX historical data without code."
	/>
	<meta property="og:image" content="https://trading-lab.xyz/dashboard-dark.webp" />
	<meta property="og:image:width" content="2196" />
	<meta property="og:image:height" content="1716" />
	<meta property="og:image:alt" content="Trading Lab platform dashboard and strategy backtester" />

	<!-- Twitter Cards -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:url" content="https://trading-lab.xyz" />
	<meta name="twitter:title" content="Trading Lab | Alphabyte" />
	<meta
		name="twitter:description"
		content="Build, simulate, and refine quantitative trading strategies with real IDX historical data without code."
	/>
	<meta name="twitter:image" content="https://trading-lab.xyz/dashboard-dark.webp" />
	<meta name="twitter:image:alt" content="Trading Lab platform dashboard and strategy backtester" />
</svelte:head>

<!-- Skip to main content link for keyboard and screen reader accessibility -->
<a
	href="#main-content"
	class="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-50 focus:rounded-md focus:bg-(--accent) focus:px-4 focus:py-2 focus:text-white focus:shadow-md focus:outline-none"
>
	Skip to content
</a>

<!-- ─── PAGE WRAPPER ─────────────────────────────────────── -->
<div
	class="min-h-screen overflow-x-clip bg-(--bg) font-sans text-(--fg) selection:bg-(--accent)/20 selection:text-(--fg)"
>
	<main id="main-content">
		<HeroSection {isLoggedIn} oncomingsoon={openComingSoon} />
		<FeaturesSection />
		<ScreenshotsSection />
		<HowItWorksSection />
		<CtaSection {isLoggedIn} oncomingsoon={openComingSoon} />
	</main>
	<LandingFooter {isLoggedIn} oncomingsoon={openComingSoon} />
</div>

<ComingSoonModal bind:open={showComingSoonModal} />
