<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="flex min-h-[50vh] flex-col items-center justify-center px-2 py-4 sm:min-h-[60vh]">
	<div
		class="flex w-full max-w-lg flex-col items-center rounded-2xl border p-6 text-center sm:p-12 md:p-16"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<!-- Decorative icon -->
		<div
			class="animate-float mb-5 flex size-16 items-center justify-center rounded-2xl transition-transform duration-300 hover:scale-110 sm:mb-6 sm:size-20"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:flame" class="size-8 sm:size-10" />
		</div>

		<h1 class="font-800 mb-2 text-2xl tracking-tight sm:mb-3 sm:text-3xl" style="color: var(--fg);">
			COMING SOON
		</h1>
		<p class="font-400 text-sm leading-relaxed sm:text-base" style="color: var(--fg-muted);">
			We're building something exciting.<br />Stay tuned for the dashboard.
		</p>

		<!-- Accent line decoration -->
		<div
			class="mt-6 h-1 w-12 rounded-full sm:mt-8 sm:w-16"
			style="background-color: var(--accent);"
		></div>
	</div>
</div>
