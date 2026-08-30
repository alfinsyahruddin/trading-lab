<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from '@iconify/svelte';

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
		class="animate-fade-in fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-4"
		style="background-color: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
	>
		<!-- Dialog -->
		<div
			class="animate-modal relative flex max-h-[calc(100dvh-2rem)] w-full max-w-md flex-col overflow-hidden rounded-2xl shadow-2xl"
			style="background-color: var(--bg-card); border: 1px solid var(--border-strong);"
			role="dialog"
			aria-modal="true"
			aria-labelledby={title ? 'modal-title' : undefined}
		>
			<!-- Header -->
			{#if title}
				<div
					class="flex shrink-0 items-center justify-between border-b px-4 py-3.5 sm:px-6 sm:py-4"
					style="border-color: var(--border);"
				>
					<h2 id="modal-title" class="text-base font-700" style="color: var(--fg)">
						{title}
					</h2>
					<button
						onclick={close}
						class="btn-interactive rounded-lg p-1.5 transition-all duration-200 hover:rotate-90 hover:bg-(--bg-card-hover)"
						style="color: var(--fg-muted);"
						aria-label="Close modal"
					>
						<Icon icon="lucide:x" width="16" height="16" />
					</button>
				</div>
			{/if}

			<!-- Body -->
			<div class="flex-1 overflow-y-auto px-4 py-4 sm:px-6 sm:py-5">
				{@render children()}
			</div>

			<!-- Footer -->
			{#if footer}
				<div
					class="flex shrink-0 flex-col-reverse gap-2 border-t px-4 py-3.5 sm:flex-row sm:justify-end sm:gap-3 sm:px-6 sm:py-4"
					style="border-color: var(--border);"
				>
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}
