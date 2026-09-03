<script lang="ts">
	import Icon from '@iconify/svelte';

	let { open = $bindable(false) }: { open?: boolean } = $props();

	function close() {
		open = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') close();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="animate-fade-in fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-4"
		style="background-color: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
	>
		<!-- Dialog Container -->
		<div
			class="animate-modal relative flex max-h-[calc(100dvh-2rem)] w-full max-w-md flex-col overflow-hidden rounded-2xl p-6 shadow-2xl sm:p-8"
			style="background-color: var(--bg-card); border: 1px solid var(--border-strong);"
			role="dialog"
			aria-modal="true"
			aria-label="Coming Soon"
		>
			<!-- Close (X) button in top-right -->
			<button
				type="button"
				onclick={close}
				class="btn-interactive absolute top-3.5 right-3.5 z-10 rounded-lg p-1.5 transition-all duration-200 hover:rotate-90 hover:bg-(--bg-card-hover) sm:top-4 sm:right-4"
				style="color: var(--fg-muted);"
				aria-label="Close modal"
			>
				<Icon icon="lucide:x" width="18" height="18" />
			</button>

			<!-- Content -->
			<div class="flex flex-col items-center gap-4 text-center">
				<!-- Glowing Icon Box -->
				<div
					class="flex size-16 items-center justify-center rounded-2xl border border-(--accent)/30 bg-(--accent-soft) text-(--accent) shadow-lg"
				>
					<Icon icon="lucide:rocket" class="size-8" />
				</div>

				<!-- Title & Subtitle -->
				<div class="flex flex-col gap-2">
					<h3 class="text-xl font-bold tracking-tight text-(--fg)">We're Launching Soon!</h3>
					<p class="max-w-sm text-sm leading-relaxed text-(--fg-muted)">
						Trading Lab is currently in private preview. We are fine-tuning the platform for the
						best backtesting experience.
					</p>
				</div>

				<!-- Status Badge -->
				<div
					class="flex items-center gap-2 rounded-full border border-(--accent)/30 bg-(--accent-soft) px-3.5 py-1.5 font-mono text-xs font-semibold text-(--accent)"
				>
					<span class="relative flex size-2">
						<span
							class="absolute inline-flex size-full animate-ping rounded-full bg-(--accent) opacity-75"
						></span>
						<span class="relative inline-flex size-2 rounded-full bg-(--accent)"></span>
					</span>
					<span>Stay tuned for public release!</span>
				</div>

				<!-- Centered Got It Button (no divider) -->
				<div class="mt-2 flex w-full justify-center">
					<button
						type="button"
						onclick={close}
						class="btn-interactive w-full rounded-xl bg-(--accent) px-8 py-2.5 text-sm font-semibold text-white shadow-sm transition-all hover:bg-(--accent-hover) active:scale-97 sm:w-auto"
					>
						Got it
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
