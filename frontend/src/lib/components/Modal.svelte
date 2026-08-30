<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		open = $bindable(false),
		title = '',
		children,
		footer
	}: {
		open?: boolean;
		title?: string;
		children: Snippet;
		footer?: Snippet;
	} = $props();

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
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		style="background-color: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
	>
		<!-- Dialog -->
		<div
			class="relative w-full max-w-md rounded-xl shadow-2xl"
			style="background-color: var(--bg-card); border: 1px solid var(--border-strong);"
			role="dialog"
			aria-modal="true"
			aria-labelledby={title ? 'modal-title' : undefined}
		>
			<!-- Header -->
			{#if title}
				<div
					class="flex items-center justify-between border-b px-6 py-4"
					style="border-color: var(--border);"
				>
					<h2 id="modal-title" class="text-base font-700" style="color: var(--fg)">
						{title}
					</h2>
					<button
						onclick={close}
						class="rounded-lg p-1.5 transition-colors duration-150 hover:bg-(--bg-card-hover)"
						style="color: var(--fg-muted);"
						aria-label="Close modal"
					>
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.5"
						>
							<path d="M18 6L6 18M6 6l12 12" />
						</svg>
					</button>
				</div>
			{/if}

			<!-- Body -->
			<div class="px-6 py-5">
				{@render children()}
			</div>

			<!-- Footer -->
			{#if footer}
				<div class="flex justify-end gap-3 border-t px-6 py-4" style="border-color: var(--border);">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}
