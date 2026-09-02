<script lang="ts">
	import { onMount } from 'svelte';
	import { getToken } from '$lib/helpers/session';

	let isLoggedIn = $state(false);

	onMount(() => {
		isLoggedIn = !!getToken();
	});
</script>

<div
	class="relative flex min-h-screen flex-col items-center justify-center px-4 py-8 sm:px-6"
	style="background: linear-gradient(135deg, var(--bg) 0%, var(--bg-card) 100%);"
>
	<!-- Background decoration -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
		<div
			class="absolute -top-40 left-1/2 size-96 -translate-x-1/2 rounded-full opacity-10 sm:size-150"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<!-- Hero content -->
	<div class="relative z-10 flex w-full max-w-lg flex-col items-center text-center">
		<!-- Wordmark logo -->
		<div class="mb-6 flex flex-col items-center sm:mb-8">
			<img
				src="/logo-dark.svg"
				alt="Trading Lab"
				class="logo-dark-theme h-8 sm:h-10"
				style="max-width: 240px;"
			/>
			<img
				src="/logo-light.svg"
				alt="Trading Lab"
				class="logo-light-theme h-8 sm:h-10"
				style="max-width: 240px;"
			/>
		</div>

		<!-- Tagline / Slogan -->
		<p
			class="font-500 mb-8 max-w-md px-2 text-base leading-relaxed sm:mb-10 sm:text-lg"
			style="color: var(--fg-muted)"
		>
			Everyone built a stock screener, but no one ever backtested it!
		</p>

		<!-- CTA Buttons -->
		<div class="flex w-full max-w-xs flex-col gap-3 sm:max-w-none sm:flex-row sm:justify-center">
			{#if isLoggedIn}
				<a
					href="/dashboard"
					class="btn-interactive font-700 inline-flex w-full items-center justify-center rounded-xl px-8 py-3.5 text-sm text-white shadow-sm hover:opacity-95 hover:shadow-md active:scale-95 sm:w-auto"
					style="background-color: var(--accent);"
				>
					Go to Dashboard
				</a>
			{:else}
				<a
					href="/login"
					class="btn-interactive font-700 inline-flex w-full items-center justify-center rounded-xl px-8 py-3.5 text-sm text-white shadow-sm hover:opacity-95 hover:shadow-md active:scale-95 sm:w-auto"
					style="background-color: var(--accent);"
				>
					Sign In
				</a>
				<a
					href="/register"
					class="btn-interactive font-700 inline-flex w-full items-center justify-center rounded-xl border px-8 py-3.5 text-sm shadow-sm hover:bg-(--bg-card) hover:shadow-md active:scale-95 sm:w-auto"
					style="border-color: var(--border-strong); color: var(--fg);"
				>
					Create Account
				</a>
			{/if}
		</div>
	</div>
</div>
