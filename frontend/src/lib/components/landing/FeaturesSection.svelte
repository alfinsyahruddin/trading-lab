<script lang="ts">
	import { reveal } from '$lib/helpers/reveal';
	import SectionHeader from './SectionHeader.svelte';
	import FeatureCard from './FeatureCard.svelte';
	import { LANDING_FEATURES } from '$lib/constants/landing';
</script>

{#snippet strategyBuilderIcon()}
	<svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
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
{/snippet}

{#snippet backtestEngineIcon()}
	<svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
		<polyline
			points="3,20 8,12 13,16 18,8 25,10"
			stroke="#30B4C9"
			stroke-width="1.75"
			stroke-linecap="round"
			stroke-linejoin="round"
			fill="none"
		/>
		<path d="M3,20 L8,12 L13,16 L18,8 L25,10 L25,20 Z" fill="#30B4C9" fill-opacity="0.08" />
		<circle cx="8" cy="12" r="1.5" fill="#30B4C9" />
		<circle cx="13" cy="16" r="1.5" fill="#30B4C9" opacity="0.7" />
		<circle cx="18" cy="8" r="1.5" fill="#30B4C9" />
	</svg>
{/snippet}

{#snippet aiIcon()}
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
{/snippet}

{#snippet analyticsIcon()}
	<svg width="28" height="28" viewBox="0 0 28 28" fill="none" xmlns="http://www.w3.org/2000/svg">
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
{/snippet}

<section
	id="features-section"
	use:reveal
	class="relative bg-(--bg) px-4 py-16 sm:px-6 sm:py-24 lg:py-28"
>
	<div class="mx-auto max-w-6xl">
		<SectionHeader
			tag="The Platform"
			title="Your strategy deserves evidence."
			description="Stop guessing. Build a multi-condition filter, fire it against years of IDX data, and see the numbers."
		/>

		<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
			<FeatureCard
				title={LANDING_FEATURES[0].title}
				description={LANDING_FEATURES[0].description}
				bullets={LANDING_FEATURES[0].bullets}
				delay={LANDING_FEATURES[0].delay}
				icon={strategyBuilderIcon}
			/>

			<FeatureCard
				title={LANDING_FEATURES[1].title}
				description={LANDING_FEATURES[1].description}
				bullets={LANDING_FEATURES[1].bullets}
				delay={LANDING_FEATURES[1].delay}
				icon={backtestEngineIcon}
			/>

			<FeatureCard
				title={LANDING_FEATURES[2].title}
				description={LANDING_FEATURES[2].description}
				bullets={LANDING_FEATURES[2].bullets}
				delay={LANDING_FEATURES[2].delay}
				isAi={true}
				icon={aiIcon}
			/>

			<FeatureCard
				title={LANDING_FEATURES[3].title}
				description={LANDING_FEATURES[3].description}
				bullets={LANDING_FEATURES[3].bullets}
				delay={LANDING_FEATURES[3].delay}
				icon={analyticsIcon}
			/>
		</div>
	</div>
</section>
