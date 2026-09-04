<script lang="ts">
	import { reveal } from '$lib/helpers/reveal';

	interface Props {
		step: string;
		title: string;
		description: string;
		delay?: number;
	}

	let { step, title, description, delay = 0 }: Props = $props();
</script>

<li use:reveal={{ delay }} class="flex items-start gap-4 sm:gap-8">
	<div
		class="min-w-10 font-mono text-3xl font-extrabold text-(--accent)/35 sm:min-w-14 sm:text-5xl"
		aria-hidden="true"
	>
		{step}
	</div>
	<div class="flex flex-col gap-1 sm:gap-1.5">
		<h3 class="text-lg font-bold tracking-tight text-(--fg) sm:text-2xl">
			{title}
		</h3>
		<p class="text-sm leading-relaxed text-(--fg-muted) sm:text-base">
			{description}
		</p>
	</div>
</li>
