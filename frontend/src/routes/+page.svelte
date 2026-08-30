<script lang="ts">
	// index.svelte — hero landing page
</script>

<div
	class="relative flex min-h-screen flex-col items-center justify-center px-6"
	style="background: linear-gradient(135deg, var(--bg) 0%, var(--bg-card) 100%);"
>
	<!-- Background decoration -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
		<div
			class="absolute -top-40 left-1/2 h-150 w-150 -translate-x-1/2 rounded-full opacity-10"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<!-- Hero content -->
	<div class="relative z-10 flex flex-col items-center text-center">
		<!-- Icon + wordmark -->
		<div class="mb-8 flex flex-col items-center">
			<img src="/favicon.svg" alt="Trading Lab icon" class="mb-4 h-20 w-20 opacity-90" />
			<img
				src="/logo-dark.svg"
				alt="Trading Lab"
				class="hidden h-8 dark:block"
				style="max-width: 200px;"
			/>
			<img
				src="/logo-light.svg"
				alt="Trading Lab"
				class="block h-8 dark:hidden"
				style="max-width: 200px;"
			/>
		</div>

		<!-- Tagline -->
		<p class="mb-10 max-w-md text-lg font-400 leading-relaxed" style="color: var(--fg-muted)">
			Your intelligent trading companion. Analyse, manage, and grow — all in one place.
		</p>

		<!-- CTA Buttons -->
		<div class="flex flex-col gap-3 sm:flex-row">
			<a
				href="/login"
				class="inline-flex items-center justify-center rounded-xl px-8 py-3.5 text-sm font-700 text-white transition-opacity duration-150 hover:opacity-90"
				style="background-color: var(--accent);"
			>
				Sign In
			</a>
			<a
				href="/register"
				class="inline-flex items-center justify-center rounded-xl border px-8 py-3.5 text-sm font-700 transition-colors duration-150 hover:bg-(--bg-card)"
				style="border-color: var(--border-strong); color: var(--fg);"
			>
				Create Account
			</a>
		</div>
	</div>
</div>
