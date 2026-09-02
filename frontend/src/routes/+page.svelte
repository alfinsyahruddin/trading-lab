<script lang="ts">
	import { onMount } from 'svelte';
	import { getToken } from '$lib/helpers/session';

	let isLoggedIn = $state(false);
	let scrollY = $state(0);
	let heroVisible = $state(false);
	let featuresVisible = $state(false);
	let screenshotVisible = $state(false);
	let howVisible = $state(false);

	onMount(() => {
		isLoggedIn = !!getToken();

		// Stagger reveal
		setTimeout(() => (heroVisible = true), 100);
		setTimeout(() => (featuresVisible = true), 600);

		// Scroll listeners
		const handleScroll = () => {
			scrollY = window.scrollY;
		};
		window.addEventListener('scroll', handleScroll, { passive: true });

		// Intersection observer for sections
		const io = new IntersectionObserver(
			(entries) => {
				entries.forEach((e) => {
					if (e.isIntersecting) {
						if (e.target.id === 'features-section') featuresVisible = true;
						if (e.target.id === 'screenshot-section') screenshotVisible = true;
						if (e.target.id === 'how-section') howVisible = true;
					}
				});
			},
			{ threshold: 0.15 }
		);

		const featureEl = document.getElementById('features-section');
		const screenshotEl = document.getElementById('screenshot-section');
		const howEl = document.getElementById('how-section');
		if (featureEl) io.observe(featureEl);
		if (screenshotEl) io.observe(screenshotEl);
		if (howEl) io.observe(howEl);

		return () => {
			window.removeEventListener('scroll', handleScroll);
			io.disconnect();
		};
	});

	// Parallax helpers
	function parallax(factor: number) {
		return `transform: translateY(${scrollY * factor}px)`;
	}
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
<div class="lp-root">
	<!-- ─── HERO ─────────────────────────────────────────── -->
	<section class="lp-hero">
		<div class="lp-terminal-panel" aria-hidden="true">
			<div class="lp-terminal-topline"><span>IDX / STRATEGY LAB</span><span>MARKET DATA</span></div>
			<div class="lp-terminal-chart">
				<svg viewBox="0 0 580 250" role="presentation">
					<path
						class="lp-chart-grid"
						d="M0 50H580M0 100H580M0 150H580M0 200H580M115 0V250M230 0V250M345 0V250M460 0V250"
					/>
					<path
						class="lp-chart-area"
						d="M0 205 L55 190 L100 196 L145 155 L195 170 L240 124 L290 140 L338 78 L390 104 L440 58 L490 72 L540 28 L580 42 V250 H0 Z"
					/>
					<path
						class="lp-chart-line"
						d="M0 205 L55 190 L100 196 L145 155 L195 170 L240 124 L290 140 L338 78 L390 104 L440 58 L490 72 L540 28 L580 42"
					/>
				</svg>
			</div>
			<div class="lp-terminal-metrics">
				<span>RETURN <b>+24.8%</b></span><span>SHARPE <b>1.82</b></span><span
					>STATUS <b>READY</b></span
				>
			</div>
		</div>

		<!-- Wordmark + copy -->
		<div class="lp-hero-content" class:lp-visible={heroVisible}>
			<!-- Logo wordmark -->
			<div class="lp-wordmark">
				<img src="/logo-dark.svg" alt="Trading Lab" class="lp-logo-img" />
			</div>

			<!-- Slogan -->
			<h1 class="lp-hero-headline">
				Everyone built a<br />trading strategy, but<br /><em>no one ever backtested it.</em>
			</h1>

			<!-- Sub-copy -->
			<p class="lp-hero-sub">
				Build your strategy. Test it on real IDX history.<br />Discover what actually works.
			</p>

			<!-- Stat strip -->
			<div class="lp-stat-strip">
				<div class="lp-stat">
					<span class="lp-stat-num">50+</span>
					<span class="lp-stat-label">IDX Indicators</span>
				</div>
				<div class="lp-stat-divider" aria-hidden="true"></div>
				<div class="lp-stat">
					<span class="lp-stat-num">4 Years</span>
					<span class="lp-stat-label">Historical Data</span>
				</div>
				<div class="lp-stat-divider" aria-hidden="true"></div>
				<div class="lp-stat">
					<span class="lp-stat-num">8+</span>
					<span class="lp-stat-label">Performance Metrics</span>
				</div>
			</div>

			<!-- CTAs -->
			<div class="lp-cta-row">
				{#if isLoggedIn}
					<a href="/dashboard" class="lp-btn-primary">Go to Dashboard</a>
				{:else}
					<a href="/register" class="lp-btn-primary">Start Backtesting</a>
					<a href="/login" class="lp-btn-ghost">Sign In</a>
				{/if}
			</div>
		</div>

		<!-- Scroll indicator -->
		<div class="lp-scroll-hint" aria-hidden="true">
			<div class="lp-scroll-dot"></div>
		</div>
	</section>

	<!-- ─── ORBITAL SEPARATOR ─────────────────────────────── -->
	<div class="lp-separator" aria-hidden="true">
		<svg viewBox="0 0 1440 60" preserveAspectRatio="none" class="lp-sep-svg">
			<path
				d="M0,60 C360,0 1080,0 1440,60 L1440,60 L0,60 Z"
				fill="var(--lp-section-bg)"
				opacity="0.8"
			/>
		</svg>
	</div>

	<!-- ─── FEATURES ──────────────────────────────────────── -->
	<section id="features-section" class="lp-section lp-features">
		<div class="lp-section-inner">
			<div class="lp-section-header" class:lp-visible={featuresVisible}>
				<p class="lp-eyebrow">The Platform</p>
				<h2 class="lp-section-headline">Your strategy deserves evidence.</h2>
				<p class="lp-section-sub">
					Stop guessing. Build a multi-condition filter, fire it against years of IDX data, and see
					the numbers.
				</p>
			</div>

			<div class="lp-features-grid" class:lp-visible={featuresVisible}>
				<!-- Feature 1: Strategy Builder -->
				<article class="lp-feature-card lp-fc-delay-0">
					<div class="lp-feature-icon" aria-hidden="true">
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
					<h3 class="lp-feature-title">Visual Strategy Builder</h3>
					<p class="lp-feature-desc">
						Compose multi-group screening rules with AND/OR logic across valuation, profitability,
						solvency, dividend, and technical indicators. No code, no Python.
					</p>
					<ul class="lp-feature-bullets">
						<li>P/E, P/B, ROE, DER, Dividend Yield</li>
						<li>Price, volume, and momentum signals</li>
						<li>Cross-metric comparisons</li>
					</ul>
				</article>

				<!-- Feature 2: Backtesting Engine -->
				<article class="lp-feature-card lp-fc-delay-1">
					<div class="lp-feature-icon" aria-hidden="true">
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
					<h3 class="lp-feature-title">Historical Backtest Engine</h3>
					<p class="lp-feature-desc">
						Run asynchronous backtests against real IDX daily data. Configure capital, fees,
						portfolio size, and duration. Get results in seconds.
					</p>
					<ul class="lp-feature-bullets">
						<li>Take-profit, stop-loss, holding limits</li>
						<li>Realistic broker fee modeling</li>
						<li>Win rate, Sharpe ratio, volatility</li>
					</ul>
				</article>

				<!-- Feature 3: AI Intelligence -->
				<article class="lp-feature-card lp-fc-delay-2 lp-fc-ai">
					<div class="lp-feature-icon lp-feature-icon-ai" aria-hidden="true">
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
					<h3 class="lp-feature-title lp-feature-title-ai">AI-Powered</h3>
					<p class="lp-feature-desc">
						AI reviews your parameters and rules against IDX market dynamics, offering one-click
						refinements. After each backtest, get a structured 5-point executive summary.
					</p>
					<ul class="lp-feature-bullets">
						<li>Risk-reward optimization hints</li>
						<li>Rule enhancement suggestions</li>
						<li>Qualitative backtest analysis</li>
					</ul>
				</article>

				<!-- Feature 4: Analytics -->
				<article class="lp-feature-card lp-fc-delay-3">
					<div class="lp-feature-icon" aria-hidden="true">
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
					<h3 class="lp-feature-title">Deep Analytics</h3>
					<p class="lp-feature-desc">
						Interactive equity curves (TradingView-powered), win/loss doughnut charts, trade
						telemetry logs, top gainers/losers, and sparkline previews on every card.
					</p>
					<ul class="lp-feature-bullets">
						<li>Net vs gross equity curve toggle</li>
						<li>Jakarta timezone rendering</li>
						<li>Full trade execution history</li>
					</ul>
				</article>
			</div>
		</div>
	</section>

	<!-- ─── SCREENSHOT PROOF ──────────────────────────────── -->
	<section id="screenshot-section" class="lp-section lp-screenshots">
		<div class="lp-section-inner">
			<div class="lp-section-header" class:lp-visible={screenshotVisible}>
				<p class="lp-eyebrow">Inside the Platform</p>
				<h2 class="lp-section-headline">Built for serious traders.</h2>
				<p class="lp-section-sub">
					Every screen is designed to surface what matters: data, clarity, and confidence.
				</p>
			</div>

			<div class="lp-screens-stack" class:lp-visible={screenshotVisible}>
				<!-- Main screenshot -->
				<div class="lp-screen-main" style={parallax(-0.03)}>
					<div class="lp-screen-frame">
						<img
							src="/screenshot-backtest.png"
							alt="Backtest results showing equity curve, win rate, and performance metrics"
							class="lp-screen-img"
							loading="lazy"
						/>
						<div class="lp-screen-glow" aria-hidden="true"></div>
					</div>
					<div class="lp-screen-label">
						<span class="lp-screen-tag">Backtest Results</span>
						<p class="lp-screen-caption">
							Equity curve, win rate, Sharpe ratio, and full trade log—everything in one view.
						</p>
					</div>
				</div>

				<!-- Secondary screenshot -->
				<div class="lp-screen-secondary">
					<div class="lp-screen-frame lp-screen-frame-sm">
						<img
							src="/screenshot-dashboard.png"
							alt="Dashboard showing community leaderboard and user stats"
							class="lp-screen-img"
							loading="lazy"
						/>
						<div class="lp-screen-glow" aria-hidden="true"></div>
					</div>
					<div class="lp-screen-label">
						<span class="lp-screen-tag">Community Dashboard</span>
						<p class="lp-screen-caption">
							Discover top-performing strategies on the IDX community leaderboard.
						</p>
					</div>
				</div>

				<!-- AI screenshot -->
				<div class="lp-screen-ai">
					<div class="lp-screen-frame lp-screen-frame-ai">
						<img
							src="/screenshot-ai.png"
							alt="AI strategy suggestions with accept or ignore actions"
							class="lp-screen-img"
							loading="lazy"
						/>
						<div class="lp-screen-glow lp-screen-glow-ai" aria-hidden="true"></div>
					</div>
					<div class="lp-screen-label">
						<span class="lp-screen-tag lp-screen-tag-ai">✦ AI Suggestions</span>
						<p class="lp-screen-caption">
							One-click accept or ignore for AI-generated parameter improvements.
						</p>
					</div>
				</div>

				<!-- AI Summary screenshot -->
				<div class="lp-screen-ai-summary">
					<div class="lp-screen-frame lp-screen-frame-ai-summary">
						<img
							src="/screenshot-ai-summary.png"
							alt="AI executive summary with qualitative performance breakdown"
							class="lp-screen-img"
							loading="lazy"
						/>
						<div class="lp-screen-glow lp-screen-glow-ai-summary" aria-hidden="true"></div>
					</div>
					<div class="lp-screen-label">
						<span class="lp-screen-tag lp-screen-tag-ai-summary">✦ AI Summary</span>
						<p class="lp-screen-caption">
							Instant 5-point qualitative breakdown and key takeaways for every backtest.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ─── HOW IT WORKS ─────────────────────────────────── -->
	<section id="how-section" class="lp-section lp-how">
		<div class="lp-section-inner">
			<div class="lp-section-header" class:lp-visible={howVisible}>
				<p class="lp-eyebrow">The Workflow</p>
				<h2 class="lp-section-headline">From hypothesis to evidence in three steps.</h2>
			</div>

			<ol class="lp-steps" aria-label="How Trading Lab works">
				<li class="lp-step">
					<div class="lp-step-num" aria-hidden="true">01</div>
					<div class="lp-step-body">
						<h3 class="lp-step-title">Build your strategy</h3>
						<p class="lp-step-desc">
							Use the visual rule builder to set screening conditions across 50+ IDX financial
							indicators. Configure take-profit, stop-loss, and holding limits.
						</p>
					</div>
				</li>
				<li class="lp-step">
					<div class="lp-step-num" aria-hidden="true">02</div>
					<div class="lp-step-body">
						<h3 class="lp-step-title">Run the backtest</h3>
						<p class="lp-step-desc">
							Select your year, starting capital, and broker fees. The engine simulates trades
							against real IDX daily data asynchronously—no waiting, no blocking.
						</p>
					</div>
				</li>
				<li class="lp-step">
					<div class="lp-step-num" aria-hidden="true">03</div>
					<div class="lp-step-body">
						<h3 class="lp-step-title">Analyze and refine</h3>
						<p class="lp-step-desc">
							Review your equity curve, win rate, Sharpe ratio, and AI executive summary. Share your
							winning strategies with the community—or keep them private.
						</p>
					</div>
				</li>
			</ol>
		</div>
	</section>

	<!-- ─── CTA FOOTER ───────────────────────────────────── -->
	<section class="lp-section lp-cta-section">
		<div class="lp-cta-cosmos" aria-hidden="true">
			<div class="lp-cta-ring lp-ring-1"></div>
			<div class="lp-cta-ring lp-ring-2"></div>
			<div class="lp-cta-ring lp-ring-3"></div>
		</div>
		<div class="lp-cta-content">
			<h2 class="lp-cta-headline">
				Stop Guessing.<br /><em>Start Backtesting.</em>
			</h2>
			<p class="lp-cta-sub">Build your strategy. Test your edge. Trade with confidence.</p>
			<div class="lp-cta-row">
				{#if isLoggedIn}
					<a href="/dashboard" class="lp-btn-primary lp-btn-lg">Open Dashboard</a>
				{:else}
					<a href="/register" class="lp-btn-primary lp-btn-lg">Create Free Account</a>
					<a href="/login" class="lp-btn-ghost">Sign In</a>
				{/if}
			</div>
		</div>
	</section>

	<!-- ─── FOOTER ───────────────────────────────────────── -->
	<footer class="lp-footer">
		<div class="lp-footer-inner">
			<div class="lp-footer-brand">
				<img src="/logo-dark.svg" alt="Trading Lab" class="lp-footer-logo" />
			</div>
			<p class="lp-footer-copy">
				© Copyright {new Date().getFullYear()} Trading Lab by Alphabyte. All rights reserved.
			</p>
			<nav class="lp-footer-links" aria-label="Footer navigation">
				<a href="/login" class="lp-footer-link">Sign In</a>
				<a href="/register" class="lp-footer-link">Get Started</a>
			</nav>
		</div>
	</footer>
</div>

<style>
	/* ─── FONTS ────────────────────────────────────────────── */
	:global(body):has(.lp-root) {
		font-family: 'Figtree', system-ui, sans-serif;
	}

	/* ─── CSS TOKENS ───────────────────────────────────────── */
	.lp-root {
		--lp-void: #070a12;
		--lp-navy: #0d1525;
		--lp-card: rgba(13, 21, 37, 0.72);
		--lp-card-border: rgba(48, 180, 201, 0.18);
		--lp-cyan: #30b4c9;
		--lp-cyan-glow: rgba(48, 180, 201, 0.25);
		--lp-gold: #f5c842;
		--lp-gold-dim: rgba(245, 200, 66, 0.15);
		--lp-fg: #e8f4f8;
		--lp-fg-muted: rgba(200, 225, 235, 0.55);
		--lp-fg-dim: rgba(150, 190, 210, 0.38);
		--lp-section-bg: #0d1525;
		--lp-positive: #22c55e;

		background-color: var(--lp-void);
		color: var(--lp-fg);
		font-family: 'Figtree', system-ui, sans-serif;
		overflow-x: hidden;
		min-height: 100vh;
	}

	/* ─── HERO ─────────────────────────────────────────────── */
	.lp-hero {
		position: relative;
		width: 100%;
		min-height: 100svh;
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		background-color: var(--lp-void);
	}

	/* Hero content */
	.lp-hero-content {
		position: relative;
		z-index: 3;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 1.75rem;
		max-width: 680px;
		width: 100%;
		padding: 2rem 2rem 6rem;
		margin-left: 6%;
		opacity: 0;
		transform: translateY(24px);
		transition:
			opacity 0.8s cubic-bezier(0.22, 1, 0.36, 1),
			transform 0.8s cubic-bezier(0.22, 1, 0.36, 1);
	}

	.lp-hero-content.lp-visible {
		opacity: 1;
		transform: translateY(0);
	}

	/* Wordmark */
	.lp-wordmark {
		display: flex;
		align-items: center;
	}

	.lp-logo-img {
		height: 36px;
		width: auto;
		display: block;
	}

	/* Headline */
	.lp-hero-headline {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: clamp(2.6rem, 6vw, 5rem);
		font-weight: 800;
		line-height: 1.05;
		letter-spacing: -0.02em;
		color: var(--lp-fg);
		margin: 0;
	}

	.lp-hero-headline em {
		font-style: italic;
		color: var(--lp-cyan);
		font-weight: 700;
	}

	/* Sub-copy */
	.lp-hero-sub {
		font-size: clamp(1rem, 1.6vw, 1.2rem);
		font-weight: 400;
		color: var(--lp-fg-muted);
		line-height: 1.65;
		margin: 0;
		max-width: 480px;
	}

	/* Stat strip */
	.lp-stat-strip {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.lp-stat {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.lp-stat-num {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1.9rem;
		font-weight: 700;
		color: var(--lp-cyan);
		line-height: 1;
		font-variant-numeric: tabular-nums;
	}

	.lp-stat-label {
		font-size: 0.7rem;
		font-weight: 500;
		letter-spacing: 0.08em;
		text-transform: uppercase;
		color: var(--lp-fg-dim);
	}

	.lp-stat-divider {
		width: 1px;
		height: 2.5rem;
		background: var(--lp-card-border);
	}

	/* CTA row */
	.lp-cta-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.lp-btn-primary {
		display: inline-flex;
		align-items: center;
		padding: 0.75rem 1.75rem;
		background: var(--lp-cyan);
		color: #fff;
		font-family: 'Figtree', system-ui, sans-serif;
		font-size: 0.9rem;
		font-weight: 600;
		letter-spacing: 0.01em;
		border-radius: 8px;
		text-decoration: none;
		transition:
			transform 0.18s ease,
			box-shadow 0.18s ease,
			background-color 0.18s ease;
		box-shadow: 0 0 20px var(--lp-cyan-glow);
	}

	.lp-btn-primary:hover {
		background: #2a9fb2;
		transform: translateY(-1px);
		box-shadow: 0 4px 28px rgba(48, 180, 201, 0.45);
	}

	.lp-btn-primary:active {
		transform: scale(0.97);
	}

	.lp-btn-primary.lp-btn-lg {
		padding: 0.9rem 2.25rem;
		font-size: 1rem;
	}

	.lp-btn-ghost {
		display: inline-flex;
		align-items: center;
		padding: 0.75rem 1.5rem;
		background: transparent;
		color: var(--lp-fg-muted);
		font-family: 'Figtree', system-ui, sans-serif;
		font-size: 0.9rem;
		font-weight: 500;
		border: 1px solid var(--lp-card-border);
		border-radius: 8px;
		text-decoration: none;
		transition:
			color 0.18s ease,
			border-color 0.18s ease,
			background 0.18s ease;
	}

	.lp-btn-ghost:hover {
		color: var(--lp-fg);
		border-color: rgba(48, 180, 201, 0.4);
		background: rgba(48, 180, 201, 0.06);
	}

	/* Scroll hint */
	.lp-scroll-hint {
		position: absolute;
		bottom: 2rem;
		left: 50%;
		transform: translateX(-50%);
		z-index: 3;
	}

	.lp-scroll-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--lp-cyan);
		animation: scrollFade 2.4s cubic-bezier(0.45, 0, 0.55, 1) infinite;
		opacity: 0.7;
	}

	@keyframes scrollFade {
		0%,
		100% {
			opacity: 0.7;
			transform: translateY(0);
		}
		50% {
			opacity: 0.2;
			transform: translateY(10px);
		}
	}

	/* ─── SEPARATOR ─────────────────────────────────────────── */
	.lp-separator {
		position: relative;
		margin-top: -2px;
		overflow: hidden;
		height: 60px;
	}

	.lp-sep-svg {
		width: 100%;
		height: 100%;
	}

	/* ─── SECTIONS (shared) ─────────────────────────────────── */
	.lp-section {
		background-color: var(--lp-section-bg);
		padding: 6rem 1.5rem;
	}

	.lp-section-inner {
		max-width: 1200px;
		margin: 0 auto;
	}

	.lp-section-header {
		text-align: center;
		margin-bottom: 4rem;
		opacity: 0;
		transform: translateY(20px);
		transition:
			opacity 0.7s ease,
			transform 0.7s ease;
	}

	.lp-section-header.lp-visible {
		opacity: 1;
		transform: translateY(0);
	}

	.lp-eyebrow {
		font-size: 0.7rem;
		font-weight: 600;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		color: var(--lp-cyan);
		margin: 0 0 1rem;
	}

	.lp-section-headline {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: clamp(2rem, 4vw, 3.2rem);
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.01em;
		color: var(--lp-fg);
		margin: 0 0 1rem;
	}

	.lp-section-sub {
		font-size: 1rem;
		font-weight: 400;
		color: var(--lp-fg-muted);
		line-height: 1.7;
		max-width: 560px;
		margin: 0 auto;
	}

	/* ─── FEATURES ──────────────────────────────────────────── */
	.lp-features {
		background: var(--lp-void);
		position: relative;
	}

	.lp-features::before {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(
			ellipse 80% 50% at 50% 0%,
			rgba(48, 180, 201, 0.04) 0%,
			transparent 70%
		);
		pointer-events: none;
	}

	.lp-features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
		gap: 1.5rem;
		opacity: 0;
		transform: translateY(24px);
		transition:
			opacity 0.7s ease 0.15s,
			transform 0.7s ease 0.15s;
	}

	.lp-features-grid.lp-visible {
		opacity: 1;
		transform: translateY(0);
	}

	.lp-feature-card {
		background: var(--lp-card);
		border: 1px solid var(--lp-card-border);
		border-radius: 16px;
		padding: 1.75rem;
		display: flex;
		flex-direction: column;
		gap: 0.85rem;
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		transition:
			border-color 0.25s ease,
			box-shadow 0.25s ease,
			transform 0.25s ease;
	}

	.lp-feature-card:hover {
		border-color: rgba(48, 180, 201, 0.38);
		box-shadow: 0 0 32px rgba(48, 180, 201, 0.1);
	}

	.lp-fc-ai {
		position: relative;
		background: linear-gradient(135deg, #fbae3310 0%, #fe426b10 33%, #de98fe10 66%, #38c1fb10 100%);
		border: 1px solid transparent;
	}

	.lp-fc-ai::before {
		content: '';
		position: absolute;
		inset: -1px;
		border-radius: 16px;
		padding: 1px;
		background: linear-gradient(135deg, #fbaf33 0%, #fe426b 33%, #de98fe 66%, #38c1fb 100%);
		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask-composite: exclude;
		pointer-events: none;
		opacity: 0.85;
		transition: opacity 0.25s ease;
	}

	.lp-fc-ai:hover {
		border-color: transparent;
		box-shadow:
			0 0 32px rgba(222, 152, 254, 0.18),
			0 0 32px rgba(56, 193, 251, 0.12);
	}

	.lp-fc-ai:hover::before {
		opacity: 1;
	}

	.lp-feature-icon {
		width: 44px;
		height: 44px;
		border-radius: 10px;
		background: rgba(48, 180, 201, 0.08);
		border: 1px solid rgba(48, 180, 201, 0.18);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.lp-feature-icon-ai {
		background: linear-gradient(135deg, rgba(222, 152, 254, 0.1) 0%, rgba(56, 193, 251, 0.1) 100%);
		border: 1px solid rgba(222, 152, 254, 0.25);
	}

	.lp-feature-title-ai {
		background: linear-gradient(135deg, #fbaf33 0%, #de98fe 50%, #38c1fb 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		width: fit-content;
	}

	.lp-feature-title {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1.35rem;
		font-weight: 700;
		color: var(--lp-fg);
		margin: 0;
		line-height: 1.2;
	}

	.lp-feature-desc {
		font-size: 0.88rem;
		color: var(--lp-fg-muted);
		line-height: 1.65;
		margin: 0;
	}

	.lp-feature-bullets {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.lp-feature-bullets li {
		font-size: 0.8rem;
		color: var(--lp-fg-dim);
		padding-left: 1rem;
		position: relative;
	}

	.lp-feature-bullets li::before {
		content: '—';
		position: absolute;
		left: 0;
		color: var(--lp-cyan);
		opacity: 0.6;
	}

	/* Staggered delays */
	.lp-fc-delay-0 {
		transition-delay: 0s;
	}
	.lp-fc-delay-1 {
		transition-delay: 0.08s;
	}
	.lp-fc-delay-2 {
		transition-delay: 0.16s;
	}
	.lp-fc-delay-3 {
		transition-delay: 0.24s;
	}

	/* ─── SCREENSHOTS ───────────────────────────────────────── */
	.lp-screenshots {
		background: linear-gradient(180deg, var(--lp-section-bg) 0%, var(--lp-void) 100%);
	}

	.lp-screens-stack {
		display: grid;
		grid-template-columns: 1fr 1fr;
		grid-template-rows: auto auto;
		gap: 1.5rem;
		opacity: 0;
		transform: translateY(24px);
		transition:
			opacity 0.7s ease 0.1s,
			transform 0.7s ease 0.1s;
	}

	.lp-screens-stack.lp-visible {
		opacity: 1;
		transform: translateY(0);
	}

	.lp-screen-main {
		grid-column: 1 / -1;
	}

	.lp-screen-secondary {
		grid-column: 1 / 2;
		grid-row: 2 / span 2;
	}

	.lp-screen-ai {
		grid-column: 2 / 3;
		grid-row: 2 / 3;
	}

	.lp-screen-ai-summary {
		grid-column: 2 / 3;
		grid-row: 3 / 4;
	}

	.lp-screen-frame {
		position: relative;
		border-radius: 12px;
		overflow: hidden;
		border: 1px solid var(--lp-card-border);
		box-shadow: 0 24px 80px rgba(0, 0, 0, 0.55);
	}

	.lp-screen-frame-sm {
		border-color: var(--lp-card-border);
	}

	.lp-screen-frame-ai {
		border-color: rgba(245, 200, 66, 0.2);
	}

	.lp-screen-frame-ai-summary {
		border-color: rgba(222, 152, 254, 0.25);
	}

	.lp-screen-img {
		width: 100%;
		height: auto;
		display: block;
	}

	.lp-screen-glow {
		position: absolute;
		inset: 0;
		border-radius: 12px;
		box-shadow: inset 0 0 0 1px rgba(48, 180, 201, 0.12);
		pointer-events: none;
	}

	.lp-screen-glow-ai {
		box-shadow: inset 0 0 0 1px rgba(245, 200, 66, 0.15);
	}

	.lp-screen-glow-ai-summary {
		box-shadow: inset 0 0 0 1px rgba(56, 193, 251, 0.2);
	}

	.lp-screen-label {
		margin-top: 0.85rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.lp-screen-tag {
		font-size: 0.7rem;
		font-weight: 600;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: var(--lp-cyan);
	}

	.lp-screen-tag-ai {
		color: var(--lp-gold);
	}

	.lp-screen-tag-ai-summary {
		background: linear-gradient(135deg, #de98fe 0%, #38c1fb 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		width: fit-content;
	}

	.lp-screen-caption {
		font-size: 0.85rem;
		color: var(--lp-fg-muted);
		line-height: 1.55;
		margin: 0;
	}

	/* ─── HOW IT WORKS ──────────────────────────────────────── */
	.lp-how {
		background: var(--lp-void);
		position: relative;
	}

	.lp-how::before {
		content: '';
		position: absolute;
		top: 0;
		left: 50%;
		transform: translateX(-50%);
		width: 1px;
		height: 100%;
		background: linear-gradient(
			to bottom,
			transparent,
			rgba(48, 180, 201, 0.2) 20%,
			rgba(48, 180, 201, 0.2) 80%,
			transparent
		);
		pointer-events: none;
	}

	.lp-steps {
		display: flex;
		flex-direction: column;
		gap: 3.5rem;
		list-style: none;
		padding: 0;
		margin: 0;
		counter-reset: none;
		max-width: 700px;
		margin: 0 auto;
	}

	.lp-step {
		display: flex;
		align-items: flex-start;
		gap: 2rem;
	}

	.lp-step-num {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 3rem;
		font-weight: 800;
		color: var(--lp-cyan);
		opacity: 0.3;
		line-height: 1;
		min-width: 3.5rem;
		font-variant-numeric: tabular-nums;
	}

	.lp-step-title {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--lp-fg);
		margin: 0 0 0.5rem;
	}

	.lp-step-desc {
		font-size: 0.92rem;
		color: var(--lp-fg-muted);
		line-height: 1.7;
		margin: 0;
	}

	/* ─── CTA SECTION ───────────────────────────────────────── */
	.lp-cta-section {
		background: var(--lp-void);
		position: relative;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		padding: 6rem 1.5rem;
	}

	/* Orbital ring animation */
	.lp-cta-cosmos {
		position: absolute;
		inset: 0;
		pointer-events: none;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lp-cta-ring {
		position: absolute;
		border-radius: 50%;
		border: 1px solid rgba(48, 180, 201, 0.12);
		animation: orbitRotate 40s linear infinite;
	}

	.lp-ring-1 {
		width: min(90vw, 600px);
		height: min(90vw, 600px);
		animation-duration: 40s;
	}

	.lp-ring-2 {
		width: min(60vw, 400px);
		height: min(60vw, 400px);
		border-color: rgba(48, 180, 201, 0.1);
		animation-duration: 25s;
		animation-direction: reverse;
	}

	.lp-ring-3 {
		width: min(30vw, 220px);
		height: min(30vw, 220px);
		border-color: rgba(48, 180, 201, 0.08);
		animation-duration: 15s;
	}

	@keyframes orbitRotate {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes pulsarBeat {
		0%,
		100% {
			transform: scale(1);
			box-shadow: 0 0 20px rgba(48, 180, 201, 0.4);
		}
		50% {
			transform: scale(1.5);
			box-shadow: 0 0 40px rgba(48, 180, 201, 0.7);
		}
	}

	.lp-cta-content {
		position: relative;
		z-index: 2;
		text-align: center;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
	}

	.lp-cta-headline {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: clamp(2.2rem, 5vw, 4rem);
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.015em;
		color: var(--lp-fg);
		margin: 0;
	}

	.lp-cta-headline em {
		font-style: italic;
		color: var(--lp-cyan);
	}

	.lp-cta-sub {
		font-size: 1rem;
		color: var(--lp-fg-muted);
		line-height: 1.6;
		margin: 0;
		max-width: 420px;
	}

	.lp-cta-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	/* ─── FOOTER ─────────────────────────────────────────────── */
	.lp-footer {
		background: var(--lp-void);
		border-top: 1px solid var(--lp-card-border);
		padding: 2rem 1.5rem;
	}

	.lp-footer-inner {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.lp-footer-brand {
		display: flex;
		align-items: center;
	}

	.lp-footer-logo {
		height: 22px;
		width: auto;
		display: block;
		opacity: 0.85;
	}

	.lp-footer-copy {
		font-size: 0.75rem;
		color: var(--lp-fg-dim);
		margin: 0;
		text-align: center;
		flex: 1;
	}

	.lp-footer-links {
		display: flex;
		gap: 1.25rem;
	}

	.lp-footer-link {
		font-size: 0.8rem;
		color: var(--lp-fg-dim);
		text-decoration: none;
		transition: color 0.18s ease;
	}

	.lp-footer-link:hover {
		color: var(--lp-cyan);
	}

	/* ─── RESPONSIVE ─────────────────────────────────────────── */
	@media (max-width: 768px) {
		.lp-hero-content {
			margin-left: 0;
			padding: 2rem 1.5rem 7rem;
			align-items: center;
			text-align: center;
		}

		.lp-hero-sub {
			text-align: center;
		}

		.lp-stat-strip {
			justify-content: center;
		}

		.lp-hero-headline {
			font-size: clamp(2.2rem, 8vw, 3rem);
		}

		.lp-screens-stack {
			grid-template-columns: 1fr;
		}

		.lp-screen-main,
		.lp-screen-secondary,
		.lp-screen-ai,
		.lp-screen-ai-summary {
			grid-column: 1 / -1;
			grid-row: auto;
		}

		.lp-step {
			flex-direction: column;
			gap: 0.75rem;
		}

		.lp-step-num {
			font-size: 2rem;
		}

		.lp-footer-inner {
			flex-direction: column;
			text-align: center;
		}

		.lp-footer-copy {
			order: 2;
		}

		.lp-footer-links {
			order: 1;
		}
	}

	@media (max-width: 480px) {
		.lp-features-grid {
			grid-template-columns: 1fr;
		}
	}

	/* ─── REDUCED MOTION ──────────────────────────────────────── */
	@media (prefers-reduced-motion: reduce) {
		.lp-cta-ring,
		.lp-scroll-dot {
			animation: none;
		}

		.lp-hero-content,
		.lp-section-header,
		.lp-features-grid,
		.lp-screens-stack {
			opacity: 1;
			transform: none;
			transition: none;
		}
	}

	/* ─── FOCUS STYLES ───────────────────────────────────────── */
	.lp-btn-primary:focus-visible,
	.lp-btn-ghost:focus-visible,
	.lp-footer-link:focus-visible {
		outline: 2px solid var(--lp-cyan);
		outline-offset: 3px;
		border-radius: 4px;
	}

	/* ─── SELECTION ──────────────────────────────────────────── */
	::selection {
		background: rgba(48, 180, 201, 0.25);
		color: var(--lp-fg);
	}

	/* ─── MINIMAL TERMINAL OVERRIDES ───────────────────────── */
	:global(body):has(.lp-root),
	.lp-root {
		font-family: 'Montserrat', system-ui, sans-serif;
		background: #2a344c;
	}

	.lp-root {
		--lp-void: #2a344c;
		--lp-navy: #303b5a;
		--lp-card: #303b5a;
		--lp-card-border: rgba(255, 255, 255, 0.1);
		--lp-fg: #f1f5f9;
		--lp-fg-muted: #94a3b8;
		--lp-fg-dim: rgba(148, 163, 184, 0.72);
		--lp-section-bg: #303b5a;
	}

	.lp-hero {
		min-height: 100svh;
		justify-content: flex-start;
		background-color: var(--lp-void);
		background-image:
			linear-gradient(rgba(255, 255, 255, 0.035) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.035) 1px, transparent 1px);
		background-size: 48px 48px;
	}

	.lp-hero::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			90deg,
			var(--lp-void) 22%,
			rgba(42, 52, 76, 0.82) 54%,
			rgba(42, 52, 76, 0.22)
		);
		pointer-events: none;
	}

	.lp-terminal-panel {
		position: absolute;
		right: max(4vw, 2rem);
		top: 50%;
		z-index: 1;
		width: min(46vw, 620px);
		padding: 1rem;
		border: 1px solid var(--lp-card-border);
		background: rgba(48, 59, 90, 0.74);
		box-shadow: 0 28px 70px rgba(15, 23, 42, 0.28);
		transform: translateY(-50%);
	}

	.lp-terminal-topline,
	.lp-terminal-metrics {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		font-family: 'IBM Plex Mono', monospace;
		font-size: 0.625rem;
		font-weight: 500;
		letter-spacing: 0.1em;
		color: var(--lp-fg-muted);
	}

	.lp-terminal-topline {
		padding: 0.15rem 0.15rem 0.9rem;
	}
	.lp-terminal-metrics {
		padding: 0.9rem 0.15rem 0.1rem;
	}
	.lp-terminal-metrics b {
		color: var(--lp-cyan);
		font-weight: 600;
	}
	.lp-terminal-chart svg {
		display: block;
		width: 100%;
		height: auto;
	}
	.lp-chart-grid {
		fill: none;
		stroke: rgba(255, 255, 255, 0.09);
		stroke-width: 1;
	}
	.lp-chart-area {
		fill: rgba(48, 180, 201, 0.08);
	}
	.lp-chart-line {
		fill: none;
		stroke: var(--lp-cyan);
		stroke-width: 2.5;
		stroke-linecap: round;
		stroke-linejoin: round;
	}

	.lp-hero-content {
		margin-left: max(6%, 2rem);
		max-width: 650px;
	}
	.lp-logo-img {
		height: 30px;
	}
	.lp-hero-headline,
	.lp-section-headline,
	.lp-feature-title,
	.lp-step-title,
	.lp-cta-headline {
		font-family: 'Montserrat', system-ui, sans-serif;
		letter-spacing: -0.04em;
	}
	.lp-hero-headline {
		font-size: clamp(2.45rem, 5vw, 4.35rem);
	}
	.lp-eyebrow,
	.lp-stat-label,
	.lp-screen-tag {
		font-family: 'IBM Plex Mono', monospace;
	}
	.lp-stat-strip {
		gap: 1.25rem;
		padding: 0.95rem 0;
		border-block: 1px solid var(--lp-card-border);
	}
	.lp-stat-divider {
		background: var(--lp-card-border);
	}
	.lp-btn-primary,
	.lp-btn-ghost {
		border-radius: 4px;
		font-family: 'Montserrat', system-ui, sans-serif;
	}
	.lp-btn-primary {
		box-shadow: none;
	}
	.lp-btn-primary:hover {
		box-shadow: 0 5px 18px rgba(48, 180, 201, 0.22);
	}

	.lp-separator {
		display: none;
	}
	.lp-section {
		padding-block: 6.5rem;
	}
	.lp-features,
	.lp-how {
		background: var(--lp-void);
	}
	.lp-features::before,
	.lp-how::before {
		display: none;
	}
	.lp-screenshots {
		background: var(--lp-section-bg);
	}
	.lp-feature-card {
		border-radius: 16px;
		box-shadow: none;
		backdrop-filter: none;
	}
	.lp-feature-card:hover {
		box-shadow: 0 14px 32px rgba(15, 23, 42, 0.18);
	}
	.lp-screen-frame {
		border-radius: 6px;
		box-shadow: 0 18px 48px rgba(15, 23, 42, 0.3);
	}
	.lp-screen-glow {
		border-radius: 6px;
	}
	.lp-step-num {
		font-family: 'IBM Plex Mono', monospace;
		font-size: 2.45rem;
	}

	.lp-cta-section {
		min-height: 52vh;
		background: var(--lp-section-bg);
	}
	.lp-cta-section::before {
		content: '';
		position: absolute;
		inset: 12% max(5vw, 1.5rem);
		border: 1px solid var(--lp-card-border);
		background-image:
			linear-gradient(90deg, rgba(255, 255, 255, 0.025) 1px, transparent 1px),
			linear-gradient(rgba(255, 255, 255, 0.025) 1px, transparent 1px);
		background-size: 32px 32px;
	}
	.lp-cta-cosmos {
		display: none;
	}
	.lp-footer {
		background: var(--lp-void);
	}

	@media (max-width: 768px) {
		.lp-hero {
			justify-content: center;
		}
		.lp-hero::before {
			background: rgba(42, 52, 76, 0.82);
		}
		.lp-terminal-panel {
			opacity: 0.22;
			right: -24%;
			width: 88vw;
		}
		.lp-hero-content {
			margin-left: 0;
		}
	}
</style>
