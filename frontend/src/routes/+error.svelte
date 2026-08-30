<script lang="ts">
	// error.svelte — custom error page
	import { page } from '$app/stores';
</script>

<div
	class="flex min-h-screen flex-col items-center justify-center px-6 text-center"
	style="background-color: var(--bg);"
>
	<h1 class="mb-2 text-6xl font-800" style="color: var(--accent)">
		{$page.status}
	</h1>
	<p class="mb-8 text-lg font-500" style="color: var(--fg-muted)">
		{$page.error?.message || 'Something went wrong.'}
	</p>
	<a
		href="/"
		class="rounded-xl px-6 py-3 text-sm font-700 text-white transition-opacity hover:opacity-90"
		style="background-color: var(--accent);"
	>
		Go Home
	</a>
</div>
