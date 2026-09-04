<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import Icon from '@iconify/svelte';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import { getToken } from '$lib/helpers/session';

	let isLoggedIn = $state(false);
	let heroVisible = $state(false);

	const typewriterWords = ['trading strategy', 'methodology', 'stock screener'];
	let displayedWord = $state(typewriterWords[0]);

	// ─── SCROLL REVEAL ENGINE ─────────────────────────────
	let revealObserver: IntersectionObserver | null = null;
	const revealNodes = new SvelteSet<HTMLElement>();
	let scrollListenerAttached = false;
	let rafId: number | null = null;

	function checkReveals() {
		if (typeof window === 'undefined') return;
		const vh = window.innerHeight || document.documentElement.clientHeight;
		const triggerBottom = vh * 0.94;
		const scrollY = window.scrollY || window.pageYOffset || document.documentElement.scrollTop || 0;
		const scrollHeight = Math.max(
			document.documentElement.scrollHeight,
			document.body.scrollHeight,
			document.documentElement.offsetHeight
		);
		const isAtPageBottom = vh + scrollY >= scrollHeight - 80;

		revealNodes.forEach((node) => {
			if (!node.classList.contains('is-revealed')) {
				const rect = node.getBoundingClientRect();
				if (isAtPageBottom || (rect.top <= triggerBottom && rect.bottom >= 0)) {
					node.classList.add('is-revealed');
					revealNodes.delete(node);
					revealObserver?.unobserve(node);
				}
			}
		});

		if (revealNodes.size === 0 && scrollListenerAttached && typeof window !== 'undefined') {
			window.removeEventListener('scroll', onScrollRaf);
			scrollListenerAttached = false;
		}
	}

	function onScrollRaf() {
		if (rafId !== null) return;
		rafId = requestAnimationFrame(() => {
			rafId = null;
			checkReveals();
		});
	}

	function getRevealObserver() {
		if (!revealObserver && typeof IntersectionObserver !== 'undefined') {
			revealObserver = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						if (entry.isIntersecting) {
							(entry.target as HTMLElement).classList.add('is-revealed');
							revealNodes.delete(entry.target as HTMLElement);
							revealObserver?.unobserve(entry.target);
						}
					});
				},
				{
					threshold: 0,
					rootMargin: '0px 0px -10px 0px'
				}
			);
		}
		return revealObserver;
	}

	function reveal(node: HTMLElement, options: { delay?: number; y?: number } = {}) {
		const delay = options.delay ?? 0;
		const y = options.y ?? 28;

		node.classList.add('reveal-on-scroll');
		node.style.setProperty('--reveal-y', `${y}px`);
		if (delay > 0) {
			node.style.transitionDelay = `${delay}ms`;
		}

		revealNodes.add(node);

		if (typeof window !== 'undefined') {
			const obs = getRevealObserver();
			obs?.observe(node);

			if (!scrollListenerAttached) {
				window.addEventListener('scroll', onScrollRaf, { passive: true });
				scrollListenerAttached = true;
			}

			requestAnimationFrame(() => checkReveals());
		}

		return {
			destroy() {
				revealNodes.delete(node);
				revealObserver?.unobserve(node);
			}
		};
	}

	onMount(() => {
		isLoggedIn = !!getToken();

		// Hero reveal shortly after mount
		setTimeout(() => (heroVisible = true), 100);

		// Typewriter animation loop
		let typewriterTimer: ReturnType<typeof setTimeout> | null = null;
		let wordIdx = 0;
		let charIdx = typewriterWords[0].length;
		let deleting = false;

		function runTypewriter() {
			const currentTarget = typewriterWords[wordIdx];

			if (deleting) {
				charIdx--;
				displayedWord = currentTarget.slice(0, charIdx);
				if (charIdx <= 0) {
					deleting = false;
					wordIdx = (wordIdx + 1) % typewriterWords.length;
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
			revealObserver?.disconnect();
			revealObserver = null;
			revealNodes.clear();
			if (scrollListenerAttached && typeof window !== 'undefined') {
				window.removeEventListener('scroll', onScrollRaf);
				scrollListenerAttached = false;
			}
			if (rafId !== null) cancelAnimationFrame(rafId);
			if (typewriterTimer) clearTimeout(typewriterTimer);
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
	<!-- ─── HERO ─────────────────────────────────────────── -->
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
		<div
			class="pointer-events-none absolute top-1/2 right-[max(4vw,2rem)] z-1 hidden w-[min(46vw,620px)] -translate-y-1/2 border border-(--border) bg-(--bg-card)/75 p-4 shadow-xl backdrop-blur-md transition-all duration-300 md:block dark:shadow-[0_28px_70px_rgba(15,23,42,0.35)]"
			aria-hidden="true"
		>
			<div
				class="flex items-center justify-between gap-4 p-1 pb-3 font-mono text-[10px] font-medium tracking-widest text-(--fg-muted)"
			>
				<span class="flex items-center gap-2">
					<span class="relative flex size-2">
						<span
							class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-75"
						></span>
						<span class="relative inline-flex size-2 rounded-full bg-emerald-500"></span>
					</span>
					<span>IDX • TRADING LAB</span>
				</span>
				<span>REAL MARKET DATA</span>
			</div>
			<div class="w-full">
				<svg viewBox="0 0 580 250" class="block h-auto w-full overflow-visible" role="presentation">
					<defs>
						<!-- Area gradient fill -->
						<linearGradient id="hero-chart-gradient" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" stop-color="var(--accent, #30b4c9)" stop-opacity="0.28" />
							<stop offset="65%" stop-color="var(--accent, #30b4c9)" stop-opacity="0.08" />
							<stop offset="100%" stop-color="var(--accent, #30b4c9)" stop-opacity="0" />
						</linearGradient>
						<!-- Subtle neon glow filter for chart line -->
						<filter id="hero-chart-glow" x="-20%" y="-20%" width="140%" height="140%">
							<feDropShadow
								dx="0"
								dy="0"
								stdDeviation="3"
								flood-color="var(--accent, #30b4c9)"
								flood-opacity="0.45"
							/>
						</filter>
					</defs>

					<!-- Grid Lines -->
					<path
						class="stroke-slate-200 dark:stroke-white/5"
						fill="none"
						stroke-width="1"
						d="M0 50H580M0 100H580M0 150H580M0 200H580M115 0V250M230 0V250M345 0V250M460 0V250"
					/>

					<!-- End of data vertical tracker guideline -->
					<line
						x1="580"
						y1="42"
						x2="580"
						y2="250"
						class="chart-tracker-line stroke-(--accent)/35"
						stroke-width="1.5"
						stroke-dasharray="3 3"
					/>

					<!-- Area Fill under curve with gradient and fade-in animation -->
					<path
						class="chart-area"
						fill="url(#hero-chart-gradient)"
						d="M0 205 L55 190 L100 196 L145 155 L195 170 L240 124 L290 140 L338 78 L390 104 L440 58 L490 72 L540 28 L580 42 V250 H0 Z"
					/>

					<!-- Chart Line with smooth draw-in animation -->
					<path
						class="chart-line stroke-(--accent)"
						fill="none"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
						pathLength="1000"
						filter="url(#hero-chart-glow)"
						d="M0 205 L55 190 L100 196 L145 155 L195 170 L240 124 L290 140 L338 78 L390 104 L440 58 L490 72 L540 28 L580 42"
					/>

					<!-- Pulse Dots at the end of the chart data (580, 42) -->
					<g class="chart-pulse-dot" transform="translate(580, 42)">
						<!-- Outer expanding pulse ring 1 -->
						<circle cx="0" cy="0" r="4" fill="var(--accent, #30b4c9)">
							<animate attributeName="r" from="4" to="18" dur="2.4s" repeatCount="indefinite" />
							<animate
								attributeName="opacity"
								from="0.75"
								to="0"
								dur="2.4s"
								repeatCount="indefinite"
							/>
						</circle>

						<!-- Outer expanding pulse ring 2 (staggered delay) -->
						<circle cx="0" cy="0" r="4" fill="var(--accent, #30b4c9)">
							<animate
								attributeName="r"
								from="4"
								to="12"
								dur="2.4s"
								begin="0.8s"
								repeatCount="indefinite"
							/>
							<animate
								attributeName="opacity"
								from="0.65"
								to="0"
								dur="2.4s"
								begin="0.8s"
								repeatCount="indefinite"
							/>
						</circle>

						<!-- Ambient glow aura -->
						<circle cx="0" cy="0" r="6" fill="var(--accent, #30b4c9)" opacity="0.4">
							<animate
								attributeName="opacity"
								values="0.2;0.55;0.2"
								dur="1.8s"
								repeatCount="indefinite"
							/>
						</circle>

						<!-- Solid accent core dot -->
						<circle cx="0" cy="0" r="4.5" fill="var(--accent, #30b4c9)" />

						<!-- Crisp white center focal dot -->
						<circle cx="0" cy="0" r="2" fill="#ffffff" />
					</g>
				</svg>
			</div>
			<div
				class="flex items-center justify-between gap-4 p-1 pt-3 font-mono text-[10px] font-medium tracking-widest text-(--fg-muted)"
			>
				<span>RETURN <b class="font-semibold text-(--accent)">+24.8%</b></span>
				<span>SHARPE RATIO <b class="font-semibold text-(--accent)">1.82</b></span>
				<span>WIN RATE <b class="font-semibold text-(--accent)">75%</b></span>
			</div>
		</div>

		<!-- Hero Content (Left side) -->
		<div
			class="relative z-10 mx-auto flex w-full max-w-6xl flex-col items-start gap-6 px-4 py-10 transition-all duration-700 sm:gap-7 sm:px-8 sm:py-12 md:mx-0 md:ml-[max(6%,2rem)] md:max-w-xl lg:max-w-2xl"
			class:opacity-100={heroVisible}
			class:translate-y-0={heroVisible}
			class:opacity-0={!heroVisible}
			class:translate-y-6={!heroVisible}
		>
			<!-- Logo wordmark (original position above headline) -->
			<a href="/" class="shrink-0 transition-opacity duration-150 hover:opacity-85">
				<img
					src="/logo-dark.svg"
					alt="Trading Lab"
					class="logo-dark-theme h-7 sm:h-8"
					style="max-width: 150px;"
				/>
				<img
					src="/logo-light.svg"
					alt="Trading Lab"
					class="logo-light-theme h-7 sm:h-8"
					style="max-width: 150px;"
				/>
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
				Build your strategy. Test it on real IDX history.<br class="hidden sm:inline" />Discover
				what actually works.
			</p>

			<!-- Stat strip -->
			<div
				class="grid w-full grid-cols-3 divide-x divide-(--border) border-y border-(--border) py-3 sm:flex sm:w-auto sm:flex-wrap sm:items-center sm:gap-7 sm:divide-x-0 sm:py-3.5"
			>
				<div class="flex flex-col gap-0.5 px-2 first:pl-0 sm:px-0">
					<span class="font-mono text-xl font-bold text-(--accent) sm:text-3xl">50+</span>
					<span
						class="font-mono text-[10px] font-medium tracking-wider text-(--fg-muted) uppercase sm:text-[11px]"
					>
						IDX Indicators
					</span>
				</div>
				<div class="hidden h-9 w-px bg-(--border) sm:block" aria-hidden="true"></div>
				<div class="flex flex-col gap-0.5 px-2 sm:px-0">
					<span class="font-mono text-xl font-bold text-(--accent) sm:text-3xl">4 Years</span>
					<span
						class="font-mono text-[10px] font-medium tracking-wider text-(--fg-muted) uppercase sm:text-[11px]"
					>
						Historical Data
					</span>
				</div>
				<div class="hidden h-9 w-px bg-(--border) sm:block" aria-hidden="true"></div>
				<div class="flex flex-col gap-0.5 px-2 last:pr-0 sm:px-0">
					<span class="font-mono text-xl font-bold text-(--accent) sm:text-3xl">8+</span>
					<span
						class="font-mono text-[10px] font-medium tracking-wider text-(--fg-muted) uppercase sm:text-[11px]"
					>
						Performance Metrics
					</span>
				</div>
			</div>

			<!-- CTAs -->
			<div
				class="flex w-full flex-col gap-3 pt-1 sm:w-auto sm:flex-row sm:flex-wrap sm:items-center sm:gap-3.5"
			>
				{#if isLoggedIn}
					<a
						href="/dashboard"
						class="btn-interactive group inline-flex items-center justify-center gap-2 rounded-md bg-(--accent) px-7 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) hover:shadow-md active:scale-97"
					>
						<span>Go to Dashboard</span>
						<Icon
							icon="lucide:arrow-right"
							class="size-4 transition-transform duration-150 group-hover:translate-x-0.5"
						/>
					</a>
				{:else}
					<a
						href="/register"
						class="btn-interactive group inline-flex items-center justify-center gap-2 rounded-md bg-(--accent) px-7 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) hover:shadow-md active:scale-97"
					>
						<span>Start Backtesting</span>
						<Icon
							icon="lucide:arrow-right"
							class="size-4 transition-transform duration-150 group-hover:translate-x-0.5"
						/>
					</a>
					<a
						href="/login"
						class="btn-interactive inline-flex items-center justify-center rounded-md border border-(--border) bg-transparent px-6 py-3 text-sm font-medium text-(--fg) transition-all hover:border-(--accent)/50 hover:bg-(--accent-soft) active:scale-97"
					>
						Sign In
					</a>
				{/if}
			</div>
		</div>

		<!-- Scroll indicator -->
		<div
			class="pointer-events-none absolute inset-x-0 bottom-6 z-10 flex flex-col items-center justify-center transition-opacity duration-700"
			class:opacity-100={heroVisible}
			class:opacity-0={!heroVisible}
			aria-hidden="true"
		>
			<Icon icon="lucide:chevrons-down" class="size-6 animate-bounce text-(--accent) opacity-80" />
		</div>
	</section>

	<!-- ─── FEATURES ──────────────────────────────────────── -->
	<section
		id="features-section"
		use:reveal
		class="relative bg-(--bg) px-4 py-16 sm:px-6 sm:py-24 lg:py-28"
	>
		<div class="mx-auto max-w-6xl">
			<div class="mb-10 text-center sm:mb-14" use:reveal>
				<p class="font-mono text-xs font-semibold tracking-widest text-(--accent) uppercase">
					The Platform
				</p>
				<h2
					class="mt-2 text-2xl font-extrabold tracking-tight text-(--fg) sm:mt-2.5 sm:text-4xl lg:text-5xl"
				>
					Your strategy deserves evidence.
				</h2>
				<p class="mx-auto mt-3 max-w-xl text-sm text-(--fg-muted) sm:mt-3.5 sm:text-base">
					Stop guessing. Build a multi-condition filter, fire it against years of IDX data, and see
					the numbers.
				</p>
			</div>

			<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
				<!-- Feature 1: Strategy Builder -->
				<article
					use:reveal={{ delay: 50 }}
					class="group flex flex-col gap-3 rounded-2xl border border-(--border) bg-(--bg-card) p-5 shadow-xs transition-all duration-200 hover:border-(--accent)/40 sm:p-6.5"
				>
					<div
						class="flex size-11 shrink-0 items-center justify-center rounded-xl border border-(--accent)/20 bg-(--accent-soft)"
						aria-hidden="true"
					>
						<svg
							width="28"
							height="28"
							viewBox="0 0 28 28"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<rect x="3" y="8" width="10" height="2" rx="1" fill="#30B4C9" />
							<rect x="15" y="8" width="10" height="2" rx="1" fill="#30B4C9" opacity="0.4" />
							<rect x="3" y="14" width="6" height="2" rx="1" fill="#30B4C9" opacity="0.6" />
							<rect x="11" y="14" width="14" height="2" rx="1" fill="#30B4C9" />
							<rect x="3" y="20" width="12" height="2" rx="1" fill="#30B4C9" opacity="0.4" />
							<rect x="17" y="20" width="8" height="2" rx="1" fill="#30B4C9" opacity="0.7" />
							<circle cx="13" cy="8" r="2" fill="none" stroke="#30B4C9" stroke-width="1.5" />
							<circle cx="9" cy="14" r="2" fill="none" stroke="#30B4C9" stroke-width="1.5" />
							<circle cx="15" cy="20" r="2" fill="none" stroke="#30B4C9" stroke-width="1.5" />
						</svg>
					</div>
					<h3 class="text-lg font-bold tracking-tight text-(--fg)">Visual Strategy Builder</h3>
					<p class="text-sm leading-relaxed font-light text-(--fg-muted)">
						Compose multi-group screening rules with AND/OR logic across valuation, profitability,
						solvency, dividend, and technical indicators. No code, no Python.
					</p>
					<ul class="mt-1 flex flex-col gap-1.5 text-xs text-(--fg-muted)">
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> P/E, P/B, ROE, DER, Dividend Yield
						</li>
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Price, volume, and momentum signals
						</li>
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Cross-metric comparisons
						</li>
					</ul>
				</article>

				<!-- Feature 2: Backtesting Engine -->
				<article
					use:reveal={{ delay: 150 }}
					class="group flex flex-col gap-3 rounded-2xl border border-(--border) bg-(--bg-card) p-5 shadow-xs transition-all duration-200 hover:border-(--accent)/40 sm:p-6.5"
				>
					<div
						class="flex size-11 shrink-0 items-center justify-center rounded-xl border border-(--accent)/20 bg-(--accent-soft)"
						aria-hidden="true"
					>
						<svg
							width="28"
							height="28"
							viewBox="0 0 28 28"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<polyline
								points="3,20 8,12 13,16 18,8 25,10"
								stroke="#30B4C9"
								stroke-width="1.75"
								stroke-linecap="round"
								stroke-linejoin="round"
								fill="none"
							/>
							<path
								d="M3,20 L8,12 L13,16 L18,8 L25,10 L25,20 Z"
								fill="#30B4C9"
								fill-opacity="0.08"
							/>
							<circle cx="8" cy="12" r="1.5" fill="#30B4C9" />
							<circle cx="13" cy="16" r="1.5" fill="#30B4C9" opacity="0.7" />
							<circle cx="18" cy="8" r="1.5" fill="#30B4C9" />
						</svg>
					</div>
					<h3 class="text-lg font-bold tracking-tight text-(--fg)">Reliable Backtest Engine</h3>
					<p class="text-sm leading-relaxed font-light text-(--fg-muted)">
						Run asynchronous backtests against real IDX daily data. Configure capital, fees,
						portfolio size, and duration. Get results in seconds.
					</p>
					<ul class="mt-1 flex flex-col gap-1.5 text-xs text-(--fg-muted)">
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Take-profit, stop-loss, holding limits
						</li>
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Realistic broker fee modeling
						</li>
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Win rate, Sharpe ratio, volatility
						</li>
					</ul>
				</article>

				<!-- Feature 3: AI Intelligence -->
				<article
					use:reveal={{ delay: 250 }}
					class="group rounded-2xl bg-[linear-gradient(135deg,#fbaf33_0%,#fe426b_33%,#de98fe_66%,#38c1fb_100%)] p-px transition-all duration-200"
				>
					<div
						class="flex h-full flex-col gap-3 rounded-[15px] p-5 shadow-xs sm:p-6.5"
						style="background: linear-gradient(135deg, rgba(251, 175, 51, 0.1) 0%, rgba(254, 66, 107, 0.1) 33%, rgba(222, 152, 254, 0.1) 66%, rgba(56, 193, 251, 0.1) 100%), var(--bg-card);"
					>
						<div
							class="flex size-11 shrink-0 items-center justify-center rounded-xl border border-[rgba(222,152,254,0.25)]"
							aria-hidden="true"
						>
							<svg
								width="24"
								height="24"
								viewBox="0 0 607 607"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
								class="shrink-0"
							>
								<defs>
									<linearGradient id="landingAiIconGrad" x1="0%" y1="0%" x2="100%" y2="100%">
										<stop offset="0%" stop-color="#fbaf33" />
										<stop offset="70%" stop-color="#de98fe" />
										<stop offset="100%" stop-color="#38c1fb" />
									</linearGradient>
								</defs>
								<path
									d="M279.22 488.635C295.407 525.561 303.5 565.016 303.5 607C303.5 565.016 311.34 525.561 327.021 488.635C343.208 451.709 364.959 419.589 392.274 392.274C419.589 364.959 451.709 343.461 488.635 327.78C525.561 311.593 565.016 303.5 607 303.5C565.016 303.5 525.561 295.66 488.635 279.979C452.696 264.483 420.004 242.345 392.274 214.726C364.655 186.996 342.517 154.304 327.021 118.365C311.34 81.4392 303.5 41.9842 303.5 0C303.5 41.9842 295.407 81.4392 279.22 118.365C263.539 155.291 242.041 187.411 214.726 214.726C186.996 242.345 154.304 264.483 118.365 279.979C81.4392 295.66 41.9842 303.5 0 303.5C41.9842 303.5 81.4392 311.593 118.365 327.78C155.291 343.461 187.411 364.959 214.726 392.274C242.041 419.589 263.539 451.709 279.22 488.635Z"
									fill="url(#landingAiIconGrad)"
								/>
							</svg>
						</div>
						<h3
							class="w-fit bg-[linear-gradient(135deg,#fbaf33_0%,#de98fe_50%,#38c1fb_100%)] bg-clip-text text-lg font-bold tracking-tight text-transparent"
						>
							AI-Powered
						</h3>
						<p class="text-sm leading-relaxed font-light text-(--fg-muted)">
							AI reviews your strategies, offering one-click refinements. After each backtest, get a
							structured 5-point executive summary.
						</p>
						<ul class="mt-1 flex flex-col gap-1.5 text-xs text-(--fg-muted)">
							<li class="flex items-start gap-2">
								<span class="font-bold text-(--accent)">—</span> Risk-reward optimization hints
							</li>
							<li class="flex items-start gap-2">
								<span class="font-bold text-(--accent)">—</span> Rule enhancement suggestions
							</li>
							<li class="flex items-start gap-2">
								<span class="font-bold text-(--accent)">—</span> Qualitative backtest analysis
							</li>
						</ul>
					</div>
				</article>

				<!-- Feature 4: Analytics -->
				<article
					use:reveal={{ delay: 350 }}
					class="group flex flex-col gap-3 rounded-2xl border border-(--border) bg-(--bg-card) p-5 shadow-xs transition-all duration-200 hover:border-(--accent)/40 sm:p-6.5"
				>
					<div
						class="flex size-11 shrink-0 items-center justify-center rounded-xl border border-(--accent)/20 bg-(--accent-soft)"
						aria-hidden="true"
					>
						<svg
							width="28"
							height="28"
							viewBox="0 0 28 28"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<circle cx="14" cy="14" r="9" stroke="#30B4C9" stroke-width="1.5" fill="none" />
							<path d="M14 14 L14 5" stroke="#F5C842" stroke-width="2" stroke-linecap="round" />
							<path
								d="M14 14 L21 17"
								stroke="#30B4C9"
								stroke-width="1.5"
								stroke-linecap="round"
								opacity="0.7"
							/>
							<circle cx="14" cy="14" r="2" fill="#30B4C9" />
						</svg>
					</div>
					<h3 class="text-lg font-bold tracking-tight text-(--fg)">Deep Analytics</h3>
					<p class="text-sm leading-relaxed font-light text-(--fg-muted)">
						Interactive equity charts (TradingView-powered), win/loss doughnut charts, trade
						telemetry logs, top gainers/losers, and sparkline previews on every card.
					</p>
					<ul class="mt-1 flex flex-col gap-1.5 text-xs text-(--fg-muted)">
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Net vs gross equity curve toggle
						</li>
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Win Rate, Sharpe ratio, and volatility
							metrics
						</li>
						<li class="flex items-start gap-2">
							<span class="font-bold text-(--accent)">—</span> Full trade execution history
						</li>
					</ul>
				</article>
			</div>
		</div>
	</section>

	<!-- ─── SCREENSHOT PROOF ──────────────────────────────── -->
	<section
		id="screenshot-section"
		use:reveal
		class="border-t border-(--border) bg-(--bg-card)/40 px-4 py-16 sm:px-6 sm:py-24 lg:py-28"
	>
		<div class="mx-auto max-w-6xl">
			<div class="mb-10 text-center sm:mb-14" use:reveal>
				<p class="font-mono text-xs font-semibold tracking-widest text-(--accent) uppercase">
					Inside the Platform
				</p>
				<h2
					class="mt-2 text-2xl font-extrabold tracking-tight text-(--fg) sm:mt-2.5 sm:text-4xl lg:text-5xl"
				>
					Built for serious traders.
				</h2>
				<p class="mx-auto mt-3 max-w-xl text-sm text-(--fg-muted) sm:mt-3.5 sm:text-base">
					Every screen is designed to surface what matters: data, clarity, and confidence.
				</p>
			</div>

			<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
				<!-- Main screenshot -->
				<div use:reveal={{ delay: 50 }} class="max-md:mb-0 md:col-span-2 md:mb-6">
					<div
						class="overflow-hidden rounded-xl border border-(--border) bg-(--bg-card) shadow-none dark:shadow-2xl"
					>
						<img
							src="/backtest-light.png"
							alt="Backtest results showing equity curve, win rate, and performance metrics"
							class="block h-auto w-full dark:hidden"
							loading="lazy"
						/>
						<img
							src="/backtest-dark.png"
							alt="Backtest results showing equity curve, win rate, and performance metrics"
							class="hidden h-auto w-full dark:block"
							loading="lazy"
						/>
					</div>
					<div class="mt-3 flex flex-col gap-0.5 max-md:mb-0 sm:mt-3.5 sm:gap-1">
						<span class="font-mono text-xs font-semibold tracking-wider text-(--accent) uppercase">
							Backtest Results
						</span>
						<p class="text-sm text-(--fg-muted)">
							Equity curve, win rate, Sharpe ratio, and full trade log—everything in one view.
						</p>
					</div>
				</div>

				<!-- Secondary screenshot -->
				<div
					use:reveal={{ delay: 100 }}
					class="flex flex-col md:col-span-1 md:col-start-1 md:row-span-2 md:row-start-2"
				>
					<div
						class="overflow-hidden rounded-xl border border-(--border) bg-(--bg-card) shadow-none dark:shadow-xl"
					>
						<img
							src="/dashboard-light.png"
							alt="Dashboard showing community leaderboard and user stats"
							class="block h-auto w-full dark:hidden"
							loading="lazy"
						/>
						<img
							src="/dashboard-dark.png"
							alt="Dashboard showing community leaderboard and user stats"
							class="hidden h-auto w-full dark:block"
							loading="lazy"
						/>
					</div>
					<div class="mt-3 flex flex-col gap-0.5 sm:mt-3.5 sm:gap-1">
						<span class="font-mono text-xs font-semibold tracking-wider text-(--accent) uppercase">
							Community Dashboard
						</span>
						<p class="text-sm text-(--fg-muted)">
							Discover top-performing strategies on the IDX community leaderboard.
						</p>
					</div>
				</div>

				<!-- AI screenshot -->
				<div
					use:reveal={{ delay: 200 }}
					class="flex flex-col md:col-span-1 md:col-start-2 md:row-start-2"
				>
					<div
						class="overflow-hidden rounded-xl border border-[rgba(245,200,66,0.2)] bg-(--bg-card) shadow-none dark:shadow-xl"
					>
						<img
							src="/ai-suggestions-light.png"
							alt="AI strategy suggestions with accept or ignore actions"
							class="block h-auto w-full dark:hidden"
							loading="lazy"
						/>
						<img
							src="/ai-suggestions-dark.png"
							alt="AI strategy suggestions with accept or ignore actions"
							class="hidden h-auto w-full dark:block"
							loading="lazy"
						/>
					</div>
					<div class="mt-3 flex flex-col gap-0.5 sm:mt-3.5 sm:gap-1">
						<span class="font-mono text-xs font-semibold tracking-wider text-[#f5c842] uppercase">
							✦ AI Suggestions
						</span>
						<p class="text-sm text-(--fg-muted)">
							One-click accept or ignore for AI-generated parameter improvements.
						</p>
					</div>
				</div>

				<!-- AI Summary screenshot -->
				<div
					use:reveal={{ delay: 300 }}
					class="flex flex-col md:col-span-1 md:col-start-2 md:row-start-3"
				>
					<div
						class="overflow-hidden rounded-xl border border-[rgba(222,152,254,0.25)] bg-(--bg-card) shadow-none dark:shadow-xl"
					>
						<img
							src="/ai-summary-light.png"
							alt="AI executive summary with qualitative performance breakdown"
							class="block h-auto w-full dark:hidden"
							loading="lazy"
						/>
						<img
							src="/ai-summary-dark.png"
							alt="AI executive summary with qualitative performance breakdown"
							class="hidden h-auto w-full dark:block"
							loading="lazy"
						/>
					</div>
					<div class="mt-3 flex flex-col gap-0.5 sm:mt-3.5 sm:gap-1">
						<span
							class="w-fit bg-[linear-gradient(135deg,#de98fe_0%,#38c1fb_100%)] bg-clip-text font-mono text-xs font-semibold tracking-wider text-transparent uppercase"
						>
							✦ AI Summary
						</span>
						<p class="text-sm text-(--fg-muted)">
							5-point qualitative breakdown and key takeaways for every backtest.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─── HOW IT WORKS ─────────────────────────────────── -->
	<section
		id="how-section"
		use:reveal
		class="border-t border-(--border) bg-(--bg) px-4 py-16 sm:px-6 sm:py-24 lg:py-28"
	>
		<div class="mx-auto max-w-4xl">
			<div class="mb-10 text-center sm:mb-14" use:reveal>
				<p class="font-mono text-xs font-semibold tracking-widest text-(--accent) uppercase">
					The Workflow
				</p>
				<h2
					class="mt-2 text-2xl font-extrabold tracking-tight text-(--fg) sm:mt-2.5 sm:text-4xl lg:text-5xl"
				>
					From hypothesis to <span class="relative inline-block whitespace-nowrap">
						<span class="relative z-10">evidence</span>
						<img
							src="/paint-underline.png"
							alt=""
							aria-hidden="true"
							class="pointer-events-none absolute -bottom-1.5 left-0 z-0 h-2.5 w-[104%] max-w-none object-fill select-none sm:-bottom-2.5 sm:h-3.5 lg:-bottom-3.5 lg:h-4.5"
						/>
					</span>.
				</h2>
			</div>

			<ol class="flex flex-col gap-8 sm:gap-10" aria-label="How Trading Lab works">
				<li use:reveal={{ delay: 100 }} class="flex items-start gap-4 sm:gap-8">
					<div
						class="min-w-10 font-mono text-3xl font-extrabold text-(--accent)/35 sm:min-w-14 sm:text-5xl"
						aria-hidden="true"
					>
						01
					</div>
					<div class="flex flex-col gap-1 sm:gap-1.5">
						<h3 class="text-lg font-bold tracking-tight text-(--fg) sm:text-2xl">
							Build your strategy
						</h3>
						<p class="text-sm leading-relaxed text-(--fg-muted) sm:text-base">
							Use the visual rule builder to set screening conditions across 50+ IDX financial
							indicators. Configure take-profit, stop-loss, and holding limits.
						</p>
					</div>
				</li>
				<li use:reveal={{ delay: 200 }} class="flex items-start gap-4 sm:gap-8">
					<div
						class="min-w-10 font-mono text-3xl font-extrabold text-(--accent)/35 sm:min-w-14 sm:text-5xl"
						aria-hidden="true"
					>
						02
					</div>
					<div class="flex flex-col gap-1 sm:gap-1.5">
						<h3 class="text-lg font-bold tracking-tight text-(--fg) sm:text-2xl">
							Run the backtest
						</h3>
						<p class="text-sm leading-relaxed text-(--fg-muted) sm:text-base">
							Select your year, starting capital, and broker fees. The engine simulates trades
							against real IDX daily data asynchronously—no waiting, no blocking.
						</p>
					</div>
				</li>
				<li use:reveal={{ delay: 300 }} class="flex items-start gap-4 sm:gap-8">
					<div
						class="min-w-10 font-mono text-3xl font-extrabold text-(--accent)/35 sm:min-w-14 sm:text-5xl"
						aria-hidden="true"
					>
						03
					</div>
					<div class="flex flex-col gap-1 sm:gap-1.5">
						<h3 class="text-lg font-bold tracking-tight text-(--fg) sm:text-2xl">
							Analyze and refine
						</h3>
						<p class="text-sm leading-relaxed text-(--fg-muted) sm:text-base">
							Review your equity curve, win rate, Sharpe ratio, and AI executive summary. Share your
							winning strategies with the community—or keep them private.
						</p>
					</div>
				</li>
			</ol>
		</div>
	</section>

	<!-- ─── CTA FOOTER ───────────────────────────────────── -->
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
			<div
				class="flex w-full flex-col gap-2.5 sm:w-auto sm:flex-row sm:flex-wrap sm:items-center sm:justify-center sm:gap-3.5"
			>
				{#if isLoggedIn}
					<a
						href="/dashboard"
						class="btn-interactive group inline-flex items-center justify-center gap-2 rounded-md bg-(--accent) px-6 py-2.5 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) hover:shadow-md active:scale-97 sm:px-8 sm:py-3.5 sm:text-base"
					>
						<span>Open Dashboard</span>
						<Icon
							icon="lucide:arrow-right"
							class="size-4 transition-transform duration-150 group-hover:translate-x-0.5 sm:size-4.5"
						/>
					</a>
				{:else}
					<a
						href="/register"
						class="btn-interactive group inline-flex items-center justify-center gap-2 rounded-md bg-(--accent) px-6 py-2.5 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) hover:shadow-md active:scale-97 sm:px-8 sm:py-3.5 sm:text-base"
					>
						<span>Create Free Account</span>
						<Icon
							icon="lucide:arrow-right"
							class="size-4 transition-transform duration-150 group-hover:translate-x-0.5 sm:size-4.5"
						/>
					</a>
					<a
						href="/login"
						class="btn-interactive inline-flex items-center justify-center rounded-md border border-(--border) bg-transparent px-5 py-2.5 text-sm font-medium text-(--fg) transition-all hover:border-(--accent)/50 hover:bg-(--accent-soft) active:scale-97 sm:px-6 sm:py-3.5 sm:text-base"
					>
						Sign In
					</a>
				{/if}
			</div>
		</div>
	</section>

	<!-- ─── FOOTER ───────────────────────────────────────── -->
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
				<img
					src="/logo-dark.svg"
					alt="Trading Lab"
					class="logo-dark-theme h-6"
					style="max-width: 130px;"
				/>
				<img
					src="/logo-light.svg"
					alt="Trading Lab"
					class="logo-light-theme h-6"
					style="max-width: 130px;"
				/>
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
						class="text-xs text-(--fg-muted) transition-colors hover:text-(--accent)"
					>
						Sign In
					</a>
					<a
						href="/register"
						class="text-xs text-(--fg-muted) transition-colors hover:text-(--accent)"
					>
						Get Started
					</a>
				{/if}
			</nav>
		</div>
	</footer>
</div>
