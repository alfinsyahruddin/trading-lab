<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		targetReturn?: number;
		targetSharpe?: number;
		targetWinRate?: number;
		duration?: number;
	}

	let {
		targetReturn = 24.8,
		targetSharpe = 1.82,
		targetWinRate = 75,
		duration = 2200
	}: Props = $props();

	let returnVal = $state(0);
	let sharpeVal = $state(0);
	let winRateVal = $state(0);

	onMount(() => {
		if (typeof window === 'undefined') return;

		if (window.matchMedia?.('(prefers-reduced-motion: reduce)')?.matches) {
			returnVal = targetReturn;
			sharpeVal = targetSharpe;
			winRateVal = targetWinRate;
			return;
		}

		let rafId: number | null = null;
		let startTime: number | null = null;

		function step(timestamp: number) {
			if (!startTime) startTime = timestamp;
			const elapsed = timestamp - startTime;
			const progress = Math.min(elapsed / duration, 1);

			// Easing deceleration to match chart drawing curve (cubic-bezier(0.16, 1, 0.3, 1))
			const ease = 1 - Math.pow(1 - progress, 3);

			returnVal = targetReturn * ease;
			sharpeVal = targetSharpe * ease;
			winRateVal = targetWinRate * ease;

			if (progress < 1) {
				rafId = requestAnimationFrame(step);
			} else {
				returnVal = targetReturn;
				sharpeVal = targetSharpe;
				winRateVal = targetWinRate;
			}
		}

		rafId = requestAnimationFrame(step);

		return () => {
			if (rafId !== null) cancelAnimationFrame(rafId);
		};
	});
</script>

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
					<animate attributeName="opacity" from="0.75" to="0" dur="2.4s" repeatCount="indefinite" />
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
		<span>RETURN <b class="font-semibold text-(--accent)">+{returnVal.toFixed(1)}%</b></span>
		<span>SHARPE RATIO <b class="font-semibold text-(--accent)">{sharpeVal.toFixed(2)}</b></span>
		<span>WIN RATE <b class="font-semibold text-(--accent)">{Math.round(winRateVal)}%</b></span>
	</div>
</div>
