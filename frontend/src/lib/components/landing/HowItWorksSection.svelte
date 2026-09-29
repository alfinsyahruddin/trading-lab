<script lang="ts">
	import { reveal } from '$lib/helpers/reveal';
	import SectionHeader from './SectionHeader.svelte';
	import WorkflowStep from './WorkflowStep.svelte';
	import { LANDING_WORKFLOW_STEPS } from '$lib/constants/landing';
</script>

{#snippet workflowTitleSnippet()}
	From hypothesis to <span class="relative inline-block whitespace-nowrap">
		<span class="relative z-10">evidence</span>
		<img
			src="/paint-underline.webp"
			alt=""
			aria-hidden="true"
			width="800"
			height="134"
			loading="lazy"
			decoding="async"
			class="pointer-events-none absolute -bottom-1.5 left-0 z-0 h-2.5 w-[104%] max-w-none object-fill select-none sm:-bottom-2.5 sm:h-3.5 lg:-bottom-3.5 lg:h-4.5"
		/>
	</span>.
{/snippet}

<section
	id="how-section"
	use:reveal
	class="border-t border-(--border) bg-(--bg) px-4 py-16 sm:px-6 sm:py-24 lg:py-28"
>
	<div class="mx-auto max-w-4xl">
		<SectionHeader tag="The Workflow" titleSnippet={workflowTitleSnippet} />

		<ol class="flex flex-col gap-8 sm:gap-10" aria-label="How Trading Lab works">
			{#each LANDING_WORKFLOW_STEPS as step (step.step)}
				<WorkflowStep
					step={step.step}
					title={step.title}
					description={step.description}
					delay={step.delay}
				/>
			{/each}
		</ol>
	</div>
</section>
