<script lang="ts">
	import { reveal } from '$lib/helpers/reveal';
	import SectionHeader from './SectionHeader.svelte';
	import ScreenshotCard from './ScreenshotCard.svelte';
	import { LANDING_SCREENSHOTS } from '$lib/constants/landing';
</script>

<section
	id="screenshot-section"
	use:reveal
	class="border-t border-(--border) bg-(--bg-card)/40 px-4 py-16 sm:px-6 sm:py-24 lg:py-28"
>
	<div class="mx-auto max-w-6xl">
		<SectionHeader
			tag="Inside the Platform"
			title="Built for serious traders."
			description="Every screen is designed to surface what matters: data, clarity, and confidence."
		/>

		<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
			{#each LANDING_SCREENSHOTS as item (item.id)}
				<ScreenshotCard
					lightSrc={item.lightSrc}
					darkSrc={item.darkSrc}
					alt={item.alt}
					tag={item.tag}
					description={item.description}
					delay={item.delay}
					containerClass={item.containerClass}
					imageBorderClass={item.imageBorderClass}
					tagClass={item.tagClass}
				/>
			{/each}
		</div>
	</div>
</section>
