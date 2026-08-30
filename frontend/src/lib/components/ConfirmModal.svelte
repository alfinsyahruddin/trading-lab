<script lang="ts">
	import Modal from './Modal.svelte';

	let {
		open = $bindable(false),
		title = 'Are you sure?',
		message = '',
		confirmLabel = 'Confirm',
		cancelLabel = 'Cancel',
		loading = false,
		onconfirm,
		oncancel
	}: {
		open?: boolean;
		title?: string;
		message?: string;
		confirmLabel?: string;
		cancelLabel?: string;
		loading?: boolean;
		onconfirm?: () => void;
		oncancel?: () => void;
	} = $props();

	function handleCancel() {
		open = false;
		oncancel?.();
	}

	function handleConfirm() {
		onconfirm?.();
	}
</script>

<Modal bind:open {title}>
	{#snippet children()}
		<p class="text-sm" style="color: var(--fg-muted)">{message}</p>
	{/snippet}
	{#snippet footer()}
		<button
			onclick={handleCancel}
			class="rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			{cancelLabel}
		</button>
		<button
			onclick={handleConfirm}
			disabled={loading}
			class="rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
			style="background-color: var(--danger); opacity: {loading ? '0.7' : '1'};"
		>
			{#if loading}
				<span class="flex items-center gap-2">
					<svg class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
						></path>
					</svg>
					Loading...
				</span>
			{:else}
				{confirmLabel}
			{/if}
		</button>
	{/snippet}
</Modal>
