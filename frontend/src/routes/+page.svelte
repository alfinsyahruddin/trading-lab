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
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
	<link
		href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600&family=Montserrat:wght@400;500;600;700;800&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<!-- ─── PAGE WRAPPER ─────────────────────────────────────── -->
<div
	class="min-h-screen overflow-x-clip bg-(--bg) font-sans text-(--fg) selection:bg-(--accent)/20 selection:text-(--fg)"
>
	<HeroSection {isLoggedIn} oncomingsoon={openComingSoon} />
	<FeaturesSection />
	<ScreenshotsSection />
	<HowItWorksSection />
	<CtaSection {isLoggedIn} oncomingsoon={openComingSoon} />
	<LandingFooter {isLoggedIn} oncomingsoon={openComingSoon} />
</div>

<ComingSoonModal bind:open={showComingSoonModal} />
