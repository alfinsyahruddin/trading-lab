<script lang="ts">
	// error.svelte — custom error page
	import { page } from '$app/stores';
</script>

<div
	class="flex min-h-screen flex-col items-center justify-center px-4 py-8 text-center sm:px-6"
	style="background-color: var(--bg);"
>
	<h1 class="mb-2 text-5xl sm:text-6xl font-800" style="color: var(--accent)">
		{$page.status}
	</h1>
	<p class="mb-8 max-w-md text-base sm:text-lg font-500" style="color: var(--fg-muted)">
		{$page.error?.message || 'Something went wrong.'}
	</p>
	<a
		href="/"
		class="btn-interactive inline-flex w-full max-w-xs sm:w-auto items-center justify-center rounded-xl px-6 py-3 text-sm font-700 text-white shadow-sm hover:shadow-md hover:opacity-95 active:scale-95"
		style="background-color: var(--accent);"
	>
		Go Home
	</a>
</div>
