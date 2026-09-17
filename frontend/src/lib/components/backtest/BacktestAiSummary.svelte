<script lang="ts" module>
	export interface TextToken {
		text: string;
		bold?: boolean;
		italic?: boolean;
		tone?: 'positive' | 'negative' | 'neutral';
	}

	/**
	 * Parses markdown formatting (**bold**, *italic*) and colorizes signed percentages:
	 * - Positive (+31%, +31.2%) -> positive tone (green)
	 * - Negative (-20%, -20.5%) -> negative tone (red)
	 * - Neutral (0%, 0.0%, +0%, -0%) -> neutral tone (muted)
	 */
	export function parseSummaryTokens(input: string): TextToken[] {
		if (!input) return [];

		// Step 1: Split into formatting chunks (**bold**, *italic*, plain)
		const formatRegex = /(\*\*[^*]+\*\*|\*[^*]+\*)/g;
		const parts = input.split(formatRegex);

		const initialChunks: { text: string; bold: boolean; italic: boolean }[] = [];

		for (const part of parts) {
			if (!part) continue;
			if (part.startsWith('**') && part.endsWith('**') && part.length >= 4) {
				initialChunks.push({ text: part.slice(2, -2), bold: true, italic: false });
			} else if (part.startsWith('*') && part.endsWith('*') && part.length >= 2) {
				initialChunks.push({ text: part.slice(1, -1), bold: false, italic: true });
			} else {
				initialChunks.push({ text: part, bold: false, italic: false });
			}
		}

		// Step 2: Identify percentages (+XX%, -XX%, 0%)
		const pctRegex =
			/((?<![A-Za-z0-9_])(?:[+-]?0(?:\.0+)?%|\+(?!0(?:\.0+)?%)\d+(?:\.\d+)?%|-(?!0(?:\.0+)?%)\d+(?:\.\d+)?%))/g;

		const tokens: TextToken[] = [];

		for (const chunk of initialChunks) {
			const subParts = chunk.text.split(pctRegex);
			for (const sub of subParts) {
				if (!sub) continue;

				if (/^[+-]?0(?:\.0+)?%$/.test(sub)) {
					tokens.push({
						text: sub,
						bold: chunk.bold,
						italic: chunk.italic,
						tone: 'neutral'
					});
				} else if (/^\+(?!0(?:\.0+)?%)\d+(?:\.\d+)?%$/.test(sub)) {
					tokens.push({
						text: sub,
						bold: chunk.bold,
						italic: chunk.italic,
						tone: 'positive'
					});
				} else if (/^-(?!0(?:\.0+)?%)\d+(?:\.\d+)?%$/.test(sub)) {
					tokens.push({
						text: sub,
						bold: chunk.bold,
						italic: chunk.italic,
						tone: 'negative'
					});
				} else {
					tokens.push({
						text: sub,
						bold: chunk.bold,
						italic: chunk.italic
					});
				}
			}
		}

		return tokens;
	}
</script>

<script lang="ts">
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';
	import { cubicInOut } from 'svelte/easing';

	let {
		summary = []
	}: {
		summary?: string[];
	} = $props();

	let isCollapsed = $state(false);

	const DIMENSIONS = [
		{
			title: 'Profitability & Return',
			shortTitle: 'Return',
			icon: 'lucide:trending-up',
			badgeClass: 'text-emerald-500 dark:text-emerald-400 bg-emerald-500/10 border-emerald-500/20',
			iconColor: 'var(--success)'
		},
		{
			title: 'Risk & Volatility Profile',
			shortTitle: 'Risk',
			icon: 'lucide:shield-alert',
			badgeClass: 'text-amber-500 dark:text-amber-400 bg-amber-500/10 border-amber-500/20',
			iconColor: '#f59e0b'
		},
		{
			title: 'Holding & Execution',
			shortTitle: 'Execution',
			icon: 'lucide:clock',
			badgeClass: 'text-sky-500 dark:text-sky-400 bg-sky-500/10 border-sky-500/20',
			iconColor: '#38bdf8'
		},
		{
			title: 'Next Steps',
			shortTitle: 'Next Steps',
			icon: 'lucide:lightbulb',
			badgeClass: 'text-[#DE98FE] bg-[#DE98FE]/15 border-[#DE98FE]/30',
			iconColor: '#DE98FE'
		}
	];

	// Extract standard grid cards and spotlight recommendation
	const gridPoints = $derived.by(() => {
		if (!summary || summary.length === 0) return [];
		if (summary.length <= 3) {
			return summary.slice(0, Math.max(0, summary.length - 1));
		}
		return summary.slice(0, 3);
	});

	const recommendationPoint = $derived.by(() => {
		if (!summary || summary.length === 0) return null;
		return summary[summary.length - 1];
	});

	const isSinglePoint = $derived(summary && summary.length === 1);
</script>

{#if summary && summary.length > 0}
	<div class="ai-summary-wrapper relative mb-6 rounded-2xl p-[1.5px]">
		<!-- Subtle ambient base border -->
		<div class="absolute inset-0 rounded-2xl" style="border: 1px solid var(--border);"></div>

		<!-- Partial moving border beam -->
		<div class="ai-border-beam pointer-events-none"></div>

		<!-- Inner container -->
		<div
			class="ai-summary-inner relative z-10 flex flex-col rounded-2xl p-5 sm:p-6"
			style="background-color: var(--bg-card);"
		>
			<!-- Header: AI Gradient Icon + Title + Action Buttons -->
			<div
				class="mb-4 flex flex-wrap items-center justify-between gap-3 border-b pb-3.5"
				style="border-color: var(--border-subtle, var(--border));"
			>
				<div class="flex items-center gap-2.5">
					<svg
						width="20"
						height="20"
						viewBox="0 0 607 607"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
						class="shrink-0"
					>
						<defs>
							<linearGradient id="aiIconGrad" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#DE98FE" />
								<stop offset="100%" stop-color="#38C1FB" />
							</linearGradient>
						</defs>
						<path
							d="M279.22 488.635C295.407 525.561 303.5 565.016 303.5 607C303.5 565.016 311.34 525.561 327.021 488.635C343.208 451.709 364.959 419.589 392.274 392.274C419.589 364.959 451.709 343.461 488.635 327.78C525.561 311.593 565.016 303.5 607 303.5C565.016 303.5 525.561 295.66 488.635 279.979C452.696 264.483 420.004 242.345 392.274 214.726C364.655 186.996 342.517 154.304 327.021 118.365C311.34 81.4392 303.5 41.9842 303.5 0C303.5 41.9842 295.407 81.4392 279.22 118.365C263.539 155.291 242.041 187.411 214.726 214.726C186.996 242.345 154.304 264.483 118.365 279.979C81.4392 295.66 41.9842 303.5 0 303.5C41.9842 303.5 81.4392 311.593 118.365 327.78C155.291 343.461 187.411 364.959 214.726 392.274C242.041 419.589 263.539 451.709 279.22 488.635Z"
							fill="url(#aiIconGrad)"
						/>
					</svg>

					<div class="flex items-center gap-2">
						<h3
							class="font-700 text-sm tracking-tight sm:text-base"
							style="
								background: linear-gradient(135deg, #DE98FE 0%, #38C1FB 100%);
								-webkit-background-clip: text;
								-webkit-text-fill-color: transparent;
								background-clip: text;
							"
						>
							AI Insights
						</h3>
						<span
							class="font-600 rounded-full border px-2 py-0.5 text-[10px] tracking-wide uppercase"
							style="border-color: var(--border); color: var(--fg-muted);"
						>
							Backtest Review
						</span>
					</div>
				</div>

				<!-- Collapse action button (chevron only) -->
				<div class="flex items-center">
					<button
						type="button"
						onclick={() => (isCollapsed = !isCollapsed)}
						class="btn-interactive font-600 inline-flex size-7 items-center justify-center rounded-lg border transition-all hover:bg-(--bg-card-hover) active:scale-95"
						style="border-color: var(--border); color: var(--fg-muted);"
						title={isCollapsed ? 'Expand insights' : 'Collapse insights'}
						aria-label={isCollapsed ? 'Expand insights' : 'Collapse insights'}
					>
						<span
							class="inline-flex transition-transform duration-300 ease-in-out {isCollapsed
								? 'rotate-180'
								: ''}"
						>
							<Icon icon="lucide:chevron-up" width="14" height="14" />
						</span>
					</button>
				</div>
			</div>

			<!-- Collapsible Bento Grid Layout with smooth slide transition -->
			{#if !isCollapsed && !isSinglePoint && gridPoints.length > 0}
				<div transition:slide={{ duration: 250, easing: cubicInOut }} class="overflow-hidden pb-3">
					<!-- 3-Column Responsive Dimension Grid -->
					<div class="grid grid-cols-1 gap-3 md:grid-cols-3">
						{#each gridPoints as point, i (i)}
							{@const config = DIMENSIONS[i] || {
								title: `Dimension ${i + 1}`,
								shortTitle: `D${i + 1}`,
								icon: 'lucide:sparkles',
								badgeClass: 'text-cyan-500 bg-cyan-500/10 border-cyan-500/20',
								iconColor: 'var(--accent)'
							}}
							<div
								class="group relative flex flex-col justify-between rounded-xl border p-4 transition-all duration-200 hover:shadow-xs"
								style="background-color: transparent; border-color: var(--border);"
							>
								<!-- Card Top Header -->
								<div class="mb-2 flex items-center gap-2">
									<div
										class="flex size-6 shrink-0 items-center justify-center rounded-md border text-xs {config.badgeClass}"
									>
										<Icon
											icon={config.icon}
											width="13"
											height="13"
											style="color: {config.iconColor};"
										/>
									</div>
									<span class="font-700 text-xs tracking-tight" style="color: {config.iconColor};">
										{config.title}
									</span>
								</div>

								<!-- Card Body Content with Colorized Percentages & Smaller Text Size -->
								<p
									class="font-400 text-[11px] leading-relaxed sm:text-xs"
									style="color: var(--fg);"
								>
									{#each parseSummaryTokens(point) as token, j (j)}
										{#if token.tone === 'positive'}
											<span
												class="font-700 py-0.2 inline-block rounded px-1.5 text-[11px] sm:text-xs"
												class:italic={token.italic}
												style="color: var(--success); background-color: rgba(34, 197, 94, 0.15);"
											>
												{token.text}
											</span>
										{:else if token.tone === 'negative'}
											<span
												class="font-700 py-0.2 inline-block rounded px-1.5 text-[11px] sm:text-xs"
												class:italic={token.italic}
												style="color: var(--danger); background-color: rgba(239, 68, 68, 0.15);"
											>
												{token.text}
											</span>
										{:else if token.tone === 'neutral'}
											<span
												class="font-700 py-0.2 inline-block rounded px-1.5 text-[11px] sm:text-xs"
												class:italic={token.italic}
												style="color: var(--fg-muted); background-color: rgba(148, 163, 184, 0.15);"
											>
												{token.text}
											</span>
										{:else if token.bold}
											<strong
												class="font-700"
												class:italic={token.italic}
												style="color: var(--fg);"
											>
												{token.text}
											</strong>
										{:else if token.italic}
											<em class="italic" style="color: var(--fg);">
												{token.text}
											</em>
										{:else}
											<span>{token.text}</span>
										{/if}
									{/each}
								</p>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Spotlight Next Steps Card: Full Gradient Border + Gradient Background -->
			{#if recommendationPoint}
				<div class="next-steps-card relative rounded-xl p-[0.75px]">
					<div class="next-steps-inner relative flex flex-col rounded-[11.25px] p-4 sm:p-5">
						<div class="mb-2 flex items-center justify-between gap-2">
							<div class="flex items-center gap-2">
								<Icon
									icon="lucide:lightbulb"
									width="16"
									height="16"
									class="shrink-0"
									style="color: #DE98FE;"
								/>
								<span
									class="font-700 text-xs tracking-tight sm:text-sm"
									style="
										background: linear-gradient(135deg, #DE98FE 0%, #38C1FB 100%);
										-webkit-background-clip: text;
										-webkit-text-fill-color: transparent;
										background-clip: text;
									"
								>
									Next Steps
								</span>
							</div>
						</div>

						<!-- Recommendation Content with Formatted Tokens -->
						<p class="font-400 text-xs leading-relaxed sm:text-sm" style="color: var(--fg);">
							{#each parseSummaryTokens(recommendationPoint) as token, j (j)}
								{#if token.tone === 'positive'}
									<span
										class="font-700 inline-block rounded px-1.5 py-0.5 text-xs sm:text-sm"
										class:italic={token.italic}
										style="color: var(--success); background-color: rgba(34, 197, 94, 0.15);"
									>
										{token.text}
									</span>
								{:else if token.tone === 'negative'}
									<span
										class="font-700 inline-block rounded px-1.5 py-0.5 text-xs sm:text-sm"
										class:italic={token.italic}
										style="color: var(--danger); background-color: rgba(239, 68, 68, 0.15);"
									>
										{token.text}
									</span>
								{:else if token.tone === 'neutral'}
									<span
										class="font-700 inline-block rounded px-1.5 py-0.5 text-xs sm:text-sm"
										class:italic={token.italic}
										style="color: var(--fg-muted); background-color: rgba(148, 163, 184, 0.15);"
									>
										{token.text}
									</span>
								{:else if token.bold}
									<strong class="font-700" class:italic={token.italic} style="color: var(--fg);">
										{token.text}
									</strong>
								{:else if token.italic}
									<em class="italic" style="color: var(--fg);">
										{token.text}
									</em>
								{:else}
									<span>{token.text}</span>
								{/if}
							{/each}
						</p>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.ai-summary-wrapper {
		position: relative;
		overflow: hidden;
	}

	.ai-border-beam {
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		width: 300%;
		aspect-ratio: 1 / 1;
		background: conic-gradient(
			from 0deg,
			transparent 0deg,
			transparent 210deg,
			rgba(251, 175, 51, 0) 220deg,
			#fbaf33 255deg,
			#fe426b 285deg,
			#de98fe 315deg,
			#38c1fb 340deg,
			rgba(56, 193, 251, 0) 360deg
		);
		animation: spinBeam 6s linear infinite;
		opacity: 0.85;
		filter: blur(1.5px);
		pointer-events: none;
	}

	.ai-summary-inner {
		background:
			radial-gradient(circle at 0% 0%, rgba(251, 175, 51, 0.04) 0%, transparent 45%),
			radial-gradient(circle at 100% 0%, rgba(254, 66, 107, 0.04) 0%, transparent 45%),
			radial-gradient(circle at 100% 100%, rgba(222, 152, 254, 0.03) 0%, transparent 45%),
			radial-gradient(circle at 0% 100%, rgba(56, 193, 251, 0.03) 0%, transparent 45%),
			var(--bg-card);
		box-shadow: 0 4px 20px -4px rgba(0, 0, 0, 0.08);
	}

	.next-steps-card {
		/* background: linear-gradient(135deg, #fbaf33 0%, #fe426b 32%, #de98fe 68%, #38c1fb 100%); */
		background: linear-gradient(135deg, #fbaf3375 0%, #fe426b75 32%, #de98fe75 68%, #38c1fb75 100%);
		box-shadow:
			0 4px 20px -4px rgba(222, 152, 254, 0.1),
			0 2px 10px -2px rgba(56, 193, 251, 0.1);
	}

	.next-steps-inner {
		background:
			radial-gradient(ellipse at 0% 0%, rgba(251, 175, 51, 0.08) 0%, transparent 45%),
			radial-gradient(ellipse at 35% 0%, rgba(254, 66, 107, 0.06) 0%, transparent 45%),
			radial-gradient(ellipse at 75% 100%, rgba(222, 152, 254, 0.07) 0%, transparent 50%),
			radial-gradient(ellipse at 100% 100%, rgba(56, 193, 251, 0.08) 0%, transparent 50%),
			linear-gradient(135deg, rgba(222, 152, 254, 0.04) 0%, rgba(56, 193, 251, 0.04) 100%),
			var(--bg-card);
	}

	@keyframes spinBeam {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
