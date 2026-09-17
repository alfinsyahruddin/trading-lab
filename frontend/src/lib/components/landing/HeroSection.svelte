<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import LandingLogo from './LandingLogo.svelte';
	import LandingCtaButtons from './LandingCtaButtons.svelte';
	import HeroChartWidget from './HeroChartWidget.svelte';
	import {
		LANDING_HERO_STATS,
		LANDING_TYPEWRITER_WORDS,
		type LandingHeroStat
	} from '$lib/constants/landing';

	interface Props {
		isLoggedIn: boolean;
		oncomingsoon?: () => void;
		statsDuration?: number;
	}

	let { isLoggedIn, oncomingsoon, statsDuration = 3600 }: Props = $props();

	let heroVisible = $state(false);
	let displayedWord = $state<string>(LANDING_TYPEWRITER_WORDS[0]);
	let animatedStats = $state<number[]>(LANDING_HERO_STATS.map(() => 0));

	function formatStatDisplay(stat: LandingHeroStat, currentVal: number): string {
		const prefix = stat.prefix ?? '';
		let suffix = stat.suffix ?? '';
		if (stat.suffix === ' Years' && currentVal === 1) {
			suffix = ' Year';
		}
		return `${prefix}${currentVal}${suffix}`;
	}

	onMount(() => {
		// Hero reveal shortly after mount
		const revealTimer = setTimeout(() => (heroVisible = true), 200);

		// Number increase animation from 0 -> target (ease-out)
		let statRafId: number | null = null;
		if (typeof window !== 'undefined') {
			if (window.matchMedia?.('(prefers-reduced-motion: reduce)')?.matches) {
				animatedStats = LANDING_HERO_STATS.map((s) => s.target);
			} else {
				let statStartTime: number | null = null;

				function stepStats(timestamp: number) {
					if (!statStartTime) statStartTime = timestamp;
					const elapsed = timestamp - statStartTime;
					const progress = Math.min(elapsed / statsDuration, 1);

					// Cubic ease-out deceleration curve
					const ease = 1 - Math.pow(1 - progress, 3);

					animatedStats = LANDING_HERO_STATS.map((stat) => Math.round(stat.target * ease));

					if (progress < 1) {
						statRafId = requestAnimationFrame(stepStats);
					} else {
						animatedStats = LANDING_HERO_STATS.map((stat) => stat.target);
					}
				}

				statRafId = requestAnimationFrame(stepStats);
			}
		}

		// Typewriter animation loop
		let typewriterTimer: ReturnType<typeof setTimeout> | null = null;
		let wordIdx = 0;
		let charIdx = LANDING_TYPEWRITER_WORDS[0].length;
		let deleting = false;

		function runTypewriter() {
			const currentTarget = LANDING_TYPEWRITER_WORDS[wordIdx];

			if (deleting) {
				charIdx--;
				displayedWord = currentTarget.slice(0, charIdx);
				if (charIdx <= 0) {
					deleting = false;
					wordIdx = (wordIdx + 1) % LANDING_TYPEWRITER_WORDS.length;
					typewriterTimer = setTimeout(runTypewriter, 250);
				} else {
					typewriterTimer = setTimeout(runTypewriter, 35);
				}
			} else {
				charIdx++;
				displayedWord = currentTarget.slice(0, charIdx);
				if (charIdx >= currentTarget.length) {
					deleting = true;
					typewriterTimer = setTimeout(runTypewriter, 3000);
				} else {
					typewriterTimer = setTimeout(runTypewriter, 65);
				}
			}
		}

		typewriterTimer = setTimeout(() => {
			deleting = true;
			runTypewriter();
		}, 3000);

		return () => {
			clearTimeout(revealTimer);
			if (typewriterTimer) clearTimeout(typewriterTimer);
			if (statRafId !== null) cancelAnimationFrame(statRafId);
		};
	});
</script>

<section
	class="relative flex min-h-svh w-full items-center justify-start overflow-hidden bg-(--bg) bg-[linear-gradient(to_right,rgba(0,0,0,0.075)_1px,transparent_1px),linear-gradient(to_bottom,rgba(0,0,0,0.075)_1px,transparent_1px)] bg-size-[48px_48px] dark:bg-[linear-gradient(to_right,rgba(255,255,255,0.075)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,0.075)_1px,transparent_1px)]"
>
	<!-- Background radial accent glow (top-left) -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
		<div
			class="absolute -top-32 -left-32 size-96 rounded-full opacity-20 sm:-top-40 sm:-left-40 sm:size-125 lg:size-150"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<!-- Background gradient overlay -->
	<div
		class="pointer-events-none absolute inset-0 bg-linear-to-r from-(--bg)/80 via-(--bg)/40 to-transparent md:to-(--bg)/10"
	></div>

	<!-- Top Right Theme Toggle -->
	<div class="absolute top-4 right-4 z-20 sm:top-8 sm:right-8 lg:right-12">
		<ThemeToggle />
	</div>

	<!-- Terminal Simulation Panel (Desktop, right side) -->
	<HeroChartWidget />

	<!-- Hero Content (Left side) -->
	<div
		class="relative z-10 mx-auto flex w-full max-w-6xl flex-col items-start gap-6 px-4 py-10 transition-all duration-1400 sm:gap-7 sm:px-8 sm:py-12 md:mx-0 md:ml-[max(6%,2rem)] md:max-w-xl lg:max-w-2xl"
		class:opacity-100={heroVisible}
		class:translate-y-0={heroVisible}
		class:opacity-0={!heroVisible}
		class:translate-y-6={!heroVisible}
	>
		<!-- Logo wordmark (original position above headline) -->
		<a href="/" class="shrink-0 transition-opacity duration-150 hover:opacity-85">
			<LandingLogo size="md" />
		</a>

		<!-- Slogan -->
		<h1 class="text-3xl font-extrabold tracking-tight text-(--fg) sm:text-5xl lg:text-6xl">
			Everyone built a<br /><span>{displayedWord}</span><span
				class="animate-cursor-blink ml-1 inline-block h-[0.82em] w-[2.5px] bg-(--accent) align-baseline sm:w-[3.5px]"
				aria-hidden="true"
			></span>, but<br /><em class="font-bold text-(--accent) not-italic"
				>no one ever<br />backtested it.</em
			>
		</h1>

		<!-- Sub-copy -->
		<p
			class="max-w-lg text-sm leading-relaxed font-light text-(--fg-muted) sm:text-base sm:leading-relaxed lg:text-lg"
		>
			Build your strategy. Test it on real IDX history.<br class="hidden sm:inline" />Discover what
			actually works.
		</p>

		<!-- Stat strip -->
		<div
			class="grid w-full grid-cols-3 divide-x divide-(--border) border-y border-(--border) py-3 sm:flex sm:w-auto sm:flex-wrap sm:items-center sm:gap-7 sm:divide-x-0 sm:py-3.5"
		>
			{#each LANDING_HERO_STATS as stat, idx (stat.label)}
				{#if idx > 0}
					<div class="hidden h-9 w-px bg-(--border) sm:block" aria-hidden="true"></div>
				{/if}
				<div class="flex flex-col gap-0.5 px-2 first:pl-0 last:pr-0 sm:px-0">
					<span class="font-mono text-xl font-bold text-(--accent) sm:text-3xl">
						{formatStatDisplay(stat, animatedStats[idx] ?? stat.target)}
					</span>
					<span
						class="font-mono text-[10px] font-medium tracking-wider text-(--fg-muted) uppercase sm:text-[11px]"
					>
						{stat.label}
					</span>
				</div>
			{/each}
		</div>

		<!-- CTAs -->
		<LandingCtaButtons
			{isLoggedIn}
			primaryAuthText="Go to Dashboard"
			primaryUnauthText="Join Trading Lab"
			secondaryText="Sign In"
			size="md"
			{oncomingsoon}
		/>
	</div>

	<!-- Scroll indicator -->
	<div
		class="pointer-events-none absolute inset-x-0 bottom-6 z-10 flex flex-col items-center justify-center transition-opacity duration-1400"
		class:opacity-100={heroVisible}
		class:opacity-0={!heroVisible}
		aria-hidden="true"
	>
		<Icon icon="lucide:chevrons-down" class="size-6 animate-bounce text-(--accent) opacity-80" />
	</div>
</section>
