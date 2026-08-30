<script lang="ts">
	// index.svelte — hero landing page
</script>

<div
	class="relative flex min-h-screen flex-col items-center justify-center px-4 py-8 sm:px-6"
	style="background: linear-gradient(135deg, var(--bg) 0%, var(--bg-card) 100%);"
>
	<!-- Background decoration -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
		<div
			class="absolute -top-40 left-1/2 h-96 w-96 sm:h-150 sm:w-150 -translate-x-1/2 rounded-full opacity-10"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<!-- Hero content -->
	<div class="relative z-10 flex w-full max-w-lg flex-col items-center text-center">
		<!-- Wordmark logo -->
		<div class="mb-6 sm:mb-8 flex flex-col items-center">
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

		<!-- Tagline -->
		<p
			class="mb-8 sm:mb-10 max-w-md px-2 text-base sm:text-lg font-400 leading-relaxed"
			style="color: var(--fg-muted)"
		>
			Your intelligent trading companion. Analyse, manage, and grow — all in one place.
		</p>

		<!-- CTA Buttons -->
		<div class="flex w-full max-w-xs sm:max-w-none flex-col gap-3 sm:flex-row sm:justify-center">
			<a
				href="/login"
				class="btn-interactive inline-flex w-full sm:w-auto items-center justify-center rounded-xl px-8 py-3.5 text-sm font-700 text-white shadow-sm hover:shadow-md hover:opacity-95 active:scale-95"
				style="background-color: var(--accent);"
			>
				Sign In
			</a>
			<a
				href="/register"
				class="btn-interactive inline-flex w-full sm:w-auto items-center justify-center rounded-xl border px-8 py-3.5 text-sm font-700 shadow-sm hover:shadow-md hover:bg-(--bg-card) active:scale-95"
				style="border-color: var(--border-strong); color: var(--fg);"
			>
				Create Account
			</a>
		</div>
	</div>
</div>
