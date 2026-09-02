<script lang="ts">
	import { onMount } from 'svelte';
	import { getToken } from '$lib/helpers/session';

	let isLoggedIn = $state(false);
	let scrollY = $state(0);
	let heroVisible = $state(false);
	let featuresVisible = $state(false);
	let screenshotVisible = $state(false);
	let socialVisible = $state(false);

	// Canvas refs
	let canvasEl: HTMLCanvasElement | null = null;
	let animFrameId: number;

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
						if (e.target.id === 'social-section') socialVisible = true;
					}
				});
			},
			{ threshold: 0.15 }
		);

		const featureEl = document.getElementById('features-section');
		const screenshotEl = document.getElementById('screenshot-section');
		const socialEl = document.getElementById('social-section');
		if (featureEl) io.observe(featureEl);
		if (screenshotEl) io.observe(screenshotEl);
		if (socialEl) io.observe(socialEl);

		// Starfield canvas
		if (canvasEl) {
			initStarfield(canvasEl);
		}

		return () => {
			window.removeEventListener('scroll', handleScroll);
			io.disconnect();
			cancelAnimationFrame(animFrameId);
		};
	});

	function initStarfield(canvas: HTMLCanvasElement) {
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		let w = (canvas.width = window.innerWidth);
		let h = (canvas.height = window.innerHeight);

		const stars: { x: number; y: number; r: number; a: number; speed: number; twinkle: number }[] =
			[];
		const count = Math.min(250, Math.floor((w * h) / 6000));

		for (let i = 0; i < count; i++) {
			stars.push({
				x: Math.random() * w,
				y: Math.random() * h,
				r: Math.random() * 1.2 + 0.2,
				a: Math.random(),
				speed: Math.random() * 0.003 + 0.001,
				twinkle: Math.random() * Math.PI * 2
			});
		}

		function draw() {
			ctx!.clearRect(0, 0, w, h);
			for (const s of stars) {
				s.twinkle += s.speed;
				const alpha = 0.3 + 0.5 * ((Math.sin(s.twinkle) + 1) / 2);
				ctx!.beginPath();
				ctx!.arc(s.x, s.y, s.r, 0, Math.PI * 2);
				ctx!.fillStyle = `rgba(180,230,255,${alpha})`;
				ctx!.fill();
			}
			animFrameId = requestAnimationFrame(draw);
		}

		draw();

		const onResize = () => {
			w = canvas.width = window.innerWidth;
			h = canvas.height = window.innerHeight;
		};
		window.addEventListener('resize', onResize);
	}

	// Parallax helpers
	function parallax(factor: number) {
		return `transform: translateY(${scrollY * factor}px)`;
	}
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
	<link
		href="https://fonts.googleapis.com/css2?family=Barlow+Condensed:ital,wght@0,300;0,400;0,600;0,700;0,800;0,900;1,400;1,700&family=Figtree:wght@300;400;500;600&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<!-- ─── PAGE WRAPPER ─────────────────────────────────────── -->
<div class="lp-root">
	<!-- ─── HERO ─────────────────────────────────────────── -->
	<section class="lp-hero">
		<!-- Nebula image layer -->
		<div class="lp-nebula-layer" style={parallax(0.12)}>
			<img
				src="/landing-nebula.jpg"
				alt=""
				aria-hidden="true"
				class="lp-nebula-img"
				loading="eager"
			/>
		</div>

		<!-- Stars canvas -->
		<canvas bind:this={canvasEl} class="lp-starfield" aria-hidden="true"></canvas>

		<!-- Orbital arc hero image -->
		<div class="lp-orbital-wrap" style={parallax(0.06)}>
			<img src="/landing-hero.jpg" alt="IDX stock constellation orbital" class="lp-orbital-img" />
			<!-- Cyan glow ring overlay -->
			<div class="lp-orbital-glow" aria-hidden="true"></div>
		</div>

		<!-- Wordmark + copy -->
		<div class="lp-hero-content" class:lp-visible={heroVisible}>
			<!-- Logo wordmark -->
			<div class="lp-wordmark">
				<svg
					width="36"
					height="36"
					viewBox="0 0 36 36"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
					class="lp-logo-icon"
					aria-hidden="true"
				>
					<circle cx="18" cy="18" r="16" stroke="#30B4C9" stroke-width="1.5" />
					<circle cx="18" cy="18" r="9" stroke="#30B4C9" stroke-width="1" opacity="0.5" />
					<circle cx="18" cy="18" r="3" fill="#30B4C9" />
					<circle cx="29" cy="10" r="1.5" fill="#30B4C9" opacity="0.8" />
					<circle cx="7" cy="26" r="1" fill="#30B4C9" opacity="0.5" />
					<line x1="18" y1="2" x2="29" y2="10" stroke="#30B4C9" stroke-width="0.75" opacity="0.4" />
					<line x1="18" y1="34" x2="7" y2="26" stroke="#30B4C9" stroke-width="0.75" opacity="0.4" />
				</svg>
				<span class="lp-wordmark-text">Trading Lab</span>
			</div>

			<!-- Slogan -->
			<h1 class="lp-hero-headline">
				Everyone built a<br />stock screener, but<br /><em>no one ever backtested it.</em>
			</h1>

			<!-- Sub-copy -->
			<p class="lp-hero-sub">
				Define your strategy. Test it against real IDX history.<br />Discover what actually works.
			</p>

			<!-- Stat strip -->
			<div class="lp-stat-strip">
				<div class="lp-stat">
					<span class="lp-stat-num">50+</span>
					<span class="lp-stat-label">IDX Indicators</span>
				</div>
				<div class="lp-stat-divider" aria-hidden="true"></div>
				<div class="lp-stat">
					<span class="lp-stat-num">4 Yrs</span>
					<span class="lp-stat-label">Historical Data</span>
				</div>
				<div class="lp-stat-divider" aria-hidden="true"></div>
				<div class="lp-stat">
					<span class="lp-stat-num">6</span>
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
						Run asynchronous simulations against real IDX daily data. Configure capital, fees,
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
						<img src="/landing-ai.jpg" alt="" class="lp-ai-thumb" />
					</div>
					<div class="lp-ai-badge">✦ AI-Powered</div>
					<h3 class="lp-feature-title">AI Strategy Intelligence</h3>
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
				<h2 class="lp-section-headline">Built for serious analysis.</h2>
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
			</div>
		</div>
	</section>

	<!-- ─── HOW IT WORKS ─────────────────────────────────── -->
	<section class="lp-section lp-how">
		<div class="lp-section-inner">
			<div class="lp-section-header">
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
						<h3 class="lp-step-title">Run the simulation</h3>
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

	<!-- ─── COMMUNITY PROOF ──────────────────────────────── -->
	<section id="social-section" class="lp-section lp-social" class:lp-visible={socialVisible}>
		<div class="lp-section-inner">
			<div class="lp-section-header" class:lp-visible={socialVisible}>
				<p class="lp-eyebrow">Community</p>
				<h2 class="lp-section-headline">The best strategies rise to the top.</h2>
				<p class="lp-section-sub">
					Explore top-performing public backtests ranked by net return or community stars. Star the
					ones that inspire you.
				</p>
			</div>

			<!-- Leaderboard mockup cards -->
			<div class="lp-leaderboard" class:lp-visible={socialVisible}>
				<div class="lp-lb-card lp-lb-delay-0">
					<div class="lp-lb-rank">#1</div>
					<div class="lp-lb-body">
						<div class="lp-lb-title">Value Momentum Strategy</div>
						<div class="lp-lb-meta">by Alfin · 2024 · 1 Year</div>
					</div>
					<div class="lp-lb-sparkline" aria-hidden="true">
						<svg viewBox="0 0 80 32" class="lp-sparkline-svg">
							<polyline
								points="0,28 10,22 20,24 30,16 40,18 50,10 60,8 70,4 80,2"
								stroke="#30B4C9"
								stroke-width="1.5"
								fill="none"
								stroke-linecap="round"
							/>
						</svg>
					</div>
					<div class="lp-lb-return lp-positive">+11.95%</div>
					<div class="lp-lb-stars">⭐ 24</div>
				</div>

				<div class="lp-lb-card lp-lb-delay-1">
					<div class="lp-lb-rank">#2</div>
					<div class="lp-lb-body">
						<div class="lp-lb-title">Growth Quality Filter</div>
						<div class="lp-lb-meta">by Budi · 2023 · 6 Months</div>
					</div>
					<div class="lp-lb-sparkline" aria-hidden="true">
						<svg viewBox="0 0 80 32" class="lp-sparkline-svg">
							<polyline
								points="0,26 10,20 20,22 30,18 40,12 50,14 60,9 70,6 80,3"
								stroke="#30B4C9"
								stroke-width="1.5"
								fill="none"
								stroke-linecap="round"
							/>
						</svg>
					</div>
					<div class="lp-lb-return lp-positive">+9.42%</div>
					<div class="lp-lb-stars">⭐ 17</div>
				</div>

				<div class="lp-lb-card lp-lb-delay-2">
					<div class="lp-lb-rank">#3</div>
					<div class="lp-lb-body">
						<div class="lp-lb-title">Dividend Compounder</div>
						<div class="lp-lb-meta">by Siti · 2024 · 1 Year</div>
					</div>
					<div class="lp-lb-sparkline" aria-hidden="true">
						<svg viewBox="0 0 80 32" class="lp-sparkline-svg">
							<polyline
								points="0,30 10,26 20,22 30,20 40,18 50,15 60,12 70,9 80,5"
								stroke="#30B4C9"
								stroke-width="1.5"
								fill="none"
								stroke-linecap="round"
							/>
						</svg>
					</div>
					<div class="lp-lb-return lp-positive">+7.81%</div>
					<div class="lp-lb-stars">⭐ 11</div>
				</div>
			</div>

			<p class="lp-community-note">
				Illustrative. Based on the kinds of strategies traders build on Trading Lab.
			</p>
		</div>
	</section>

	<!-- ─── CTA FOOTER ───────────────────────────────────── -->
	<section class="lp-section lp-cta-section">
		<div class="lp-cta-cosmos" aria-hidden="true">
			<div class="lp-cta-ring lp-ring-1"></div>
			<div class="lp-cta-ring lp-ring-2"></div>
			<div class="lp-cta-ring lp-ring-3"></div>
			<div class="lp-cta-pulsar"></div>
		</div>
		<div class="lp-cta-content">
			<h2 class="lp-cta-headline">
				Your screener is a hypothesis.<br /><em>Backtest it.</em>
			</h2>
			<p class="lp-cta-sub">
				Join Indonesian traders who stopped guessing and started measuring.
			</p>
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
				<svg
					width="20"
					height="20"
					viewBox="0 0 36 36"
					fill="none"
					class="lp-footer-icon"
					aria-hidden="true"
				>
					<circle cx="18" cy="18" r="16" stroke="#30B4C9" stroke-width="1.5" />
					<circle cx="18" cy="18" r="3" fill="#30B4C9" />
				</svg>
				<span class="lp-footer-name">Trading Lab</span>
			</div>
			<p class="lp-footer-copy">
				For educational and research purposes only. Not financial advice.
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

	.lp-nebula-layer {
		position: absolute;
		inset: -10% 0;
		will-change: transform;
		pointer-events: none;
	}

	.lp-nebula-img {
		width: 100%;
		height: 110%;
		object-fit: cover;
		opacity: 0.7;
	}

	.lp-starfield {
		position: absolute;
		inset: 0;
		pointer-events: none;
		z-index: 1;
	}

	.lp-orbital-wrap {
		position: absolute;
		right: -5%;
		top: 50%;
		transform: translateY(-50%);
		width: min(60vw, 680px);
		aspect-ratio: 16/9;
		will-change: transform;
		pointer-events: none;
		z-index: 1;
	}

	.lp-orbital-img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		border-radius: 50%;
		opacity: 0.55;
		mask-image: radial-gradient(ellipse 80% 80% at center, black 40%, transparent 100%);
		-webkit-mask-image: radial-gradient(ellipse 80% 80% at center, black 40%, transparent 100%);
	}

	.lp-orbital-glow {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: radial-gradient(ellipse 60% 60% at center, var(--lp-cyan-glow) 0%, transparent 70%);
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
		gap: 0.6rem;
	}

	.lp-wordmark-text {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1.25rem;
		font-weight: 700;
		letter-spacing: 0.04em;
		text-transform: uppercase;
		color: var(--lp-cyan);
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
		background: radial-gradient(ellipse 80% 50% at 50% 0%, rgba(48, 180, 201, 0.04) 0%, transparent 70%);
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
		transform: translateY(-2px);
	}

	.lp-fc-ai {
		border-color: rgba(245, 200, 66, 0.2);
	}

	.lp-fc-ai:hover {
		border-color: rgba(245, 200, 66, 0.45);
		box-shadow: 0 0 32px rgba(245, 200, 66, 0.1);
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
		width: 44px;
		height: 44px;
		border-radius: 10px;
		overflow: hidden;
		border: 1px solid rgba(245, 200, 66, 0.25);
		padding: 0;
	}

	.lp-ai-thumb {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.lp-ai-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.3rem;
		padding: 0.2rem 0.6rem;
		border-radius: 999px;
		background: var(--lp-gold-dim);
		border: 1px solid rgba(245, 200, 66, 0.3);
		font-size: 0.65rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		color: var(--lp-gold);
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
	}

	.lp-screen-ai {
		grid-column: 2 / 3;
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

	/* ─── COMMUNITY ─────────────────────────────────────────── */
	.lp-social {
		background: linear-gradient(180deg, var(--lp-void) 0%, var(--lp-section-bg) 100%);
	}

	.lp-leaderboard {
		display: flex;
		flex-direction: column;
		gap: 0.85rem;
		opacity: 0;
		transform: translateY(20px);
		transition:
			opacity 0.6s ease 0.2s,
			transform 0.6s ease 0.2s;
	}

	.lp-leaderboard.lp-visible {
		opacity: 1;
		transform: translateY(0);
	}

	.lp-lb-card {
		display: flex;
		align-items: center;
		gap: 1.25rem;
		background: var(--lp-card);
		border: 1px solid var(--lp-card-border);
		border-radius: 12px;
		padding: 1rem 1.25rem;
		backdrop-filter: blur(6px);
		-webkit-backdrop-filter: blur(6px);
		transition:
			border-color 0.2s ease,
			transform 0.2s ease;
	}

	.lp-lb-card:hover {
		border-color: rgba(48, 180, 201, 0.35);
		transform: translateX(4px);
	}

	.lp-lb-delay-0 {
		animation-delay: 0ms;
	}
	.lp-lb-delay-1 {
		animation-delay: 80ms;
	}
	.lp-lb-delay-2 {
		animation-delay: 160ms;
	}

	.lp-lb-rank {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1.4rem;
		font-weight: 700;
		color: var(--lp-cyan);
		min-width: 2.5rem;
		font-variant-numeric: tabular-nums;
	}

	.lp-lb-body {
		flex: 1;
		min-width: 0;
	}

	.lp-lb-title {
		font-size: 0.92rem;
		font-weight: 600;
		color: var(--lp-fg);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.lp-lb-meta {
		font-size: 0.75rem;
		color: var(--lp-fg-dim);
		margin-top: 0.1rem;
	}

	.lp-lb-sparkline {
		width: 80px;
		height: 32px;
		flex-shrink: 0;
	}

	.lp-sparkline-svg {
		width: 100%;
		height: 100%;
	}

	.lp-lb-return {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1.2rem;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		min-width: 6rem;
		text-align: right;
	}

	.lp-positive {
		color: var(--lp-positive);
	}

	.lp-lb-stars {
		font-size: 0.8rem;
		color: var(--lp-fg-dim);
		min-width: 3.5rem;
		text-align: right;
	}

	.lp-community-note {
		font-size: 0.72rem;
		color: var(--lp-fg-dim);
		text-align: center;
		margin-top: 1.5rem;
		font-style: italic;
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

	.lp-cta-pulsar {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--lp-cyan);
		box-shadow: 0 0 20px var(--lp-cyan-glow);
		animation: pulsarBeat 2.5s ease-in-out infinite;
		position: absolute;
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
		gap: 0.5rem;
	}

	.lp-footer-name {
		font-family: 'Barlow Condensed', sans-serif;
		font-size: 1rem;
		font-weight: 700;
		letter-spacing: 0.04em;
		text-transform: uppercase;
		color: var(--lp-fg-muted);
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

		.lp-orbital-wrap {
			display: none;
		}

		.lp-hero-headline {
			font-size: clamp(2.2rem, 8vw, 3rem);
		}

		.lp-screens-stack {
			grid-template-columns: 1fr;
		}

		.lp-screen-main,
		.lp-screen-secondary,
		.lp-screen-ai {
			grid-column: 1 / -1;
		}

		.lp-step {
			flex-direction: column;
			gap: 0.75rem;
		}

		.lp-step-num {
			font-size: 2rem;
		}

		.lp-lb-card {
			flex-wrap: wrap;
			gap: 0.75rem;
		}

		.lp-lb-sparkline {
			display: none;
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
		.lp-cta-pulsar,
		.lp-scroll-dot {
			animation: none;
		}

		.lp-hero-content,
		.lp-section-header,
		.lp-features-grid,
		.lp-screens-stack,
		.lp-leaderboard {
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
</style>
