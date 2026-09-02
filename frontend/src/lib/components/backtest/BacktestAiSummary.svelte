<script lang="ts">
	let {
		summary = []
	}: {
		summary?: string[];
	} = $props();
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
			<!-- Header: AI Gradient Icon + Title -->
			<div class="mb-4 flex items-center gap-2.5">
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

				<h3
					class="font-700 text-sm tracking-tight sm:text-base"
					style="
						background: linear-gradient(135deg, #DE98FE 0%, #38C1FB 100%);
						-webkit-background-clip: text;
						-webkit-text-fill-color: transparent;
						background-clip: text;
					"
				>
					AI Summary
				</h3>
			</div>

			<!-- Clean Frameless Keypoints List -->
			<div class="flex flex-col gap-3">
				{#each summary as point, i (i)}
					<div class="flex items-start gap-2.5">
						<span class="font-700 text-xs select-none sm:text-sm" style="color: var(--fg-muted);">
							{i + 1}.
						</span>
						<p class="font-400 text-xs leading-relaxed sm:text-sm" style="color: var(--fg);">
							{point}
						</p>
					</div>
				{/each}
			</div>
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
		inset: -150%;
		background: conic-gradient(
			from 0deg at 50% 50%,
			transparent 0deg,
			transparent 220deg,
			#fbaf33 260deg,
			#fe426b 295deg,
			#de98fe 330deg,
			#38c1fb 360deg
		);
		animation: spinBeam 6s linear infinite;
		opacity: 0.85;
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

	@keyframes spinBeam {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
