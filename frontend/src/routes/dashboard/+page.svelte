<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="flex min-h-[50vh] sm:min-h-[60vh] flex-col items-center justify-center px-2 py-4">
	<div
		class="flex w-full max-w-lg flex-col items-center rounded-2xl border p-6 sm:p-12 md:p-16 text-center"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<!-- Decorative icon -->
		<div
			class="animate-float mb-5 sm:mb-6 flex h-16 w-16 sm:h-20 sm:w-20 items-center justify-center rounded-2xl transition-transform duration-300 hover:scale-110"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:flame" class="h-8 w-8 sm:h-10 sm:w-10" />
		</div>

		<h1 class="mb-2 sm:mb-3 text-2xl sm:text-3xl font-800 tracking-tight" style="color: var(--fg);">
			COMING SOON
		</h1>
		<p class="text-sm sm:text-base font-400 leading-relaxed" style="color: var(--fg-muted);">
			We're building something exciting.<br />Stay tuned for the dashboard.
		</p>

		<!-- Accent line decoration -->
		<div
			class="mt-6 sm:mt-8 h-1 w-12 sm:w-16 rounded-full"
			style="background-color: var(--accent);"
		></div>
	</div>
</div>
