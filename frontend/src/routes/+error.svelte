<script lang="ts">
	// error.svelte — custom error page
	import { page } from '$app/stores';
</script>

<div
	class="flex min-h-screen flex-col items-center justify-center px-4 py-8 text-center sm:px-6"
	style="background-color: var(--bg);"
>
	<h1 class="font-800 mb-2 text-5xl sm:text-6xl" style="color: var(--accent)">
		{$page.status}
	</h1>
	<p class="font-500 mb-8 max-w-md text-base sm:text-lg" style="color: var(--fg-muted)">
		{$page.error?.message || 'Something went wrong.'}
	</p>
	<a
		href="/"
		class="btn-interactive font-700 inline-flex w-full max-w-xs items-center justify-center rounded-xl px-6 py-3 text-sm text-white shadow-sm hover:opacity-95 hover:shadow-md active:scale-95 sm:w-auto"
		style="background-color: var(--accent);"
	>
		Go Home
	</a>
</div>
