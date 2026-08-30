<script lang="ts">
	import Icon from '@iconify/svelte';
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
			class="btn-interactive font-500 w-full rounded-lg border px-4 py-2 text-sm transition-colors duration-150 hover:bg-(--bg-card-hover) sm:w-auto"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			{cancelLabel}
		</button>
		<button
			onclick={handleConfirm}
			disabled={loading}
			class="btn-interactive font-600 inline-flex w-full items-center justify-center rounded-lg px-4 py-2 text-sm text-white transition-opacity duration-150 sm:w-auto"
			style="background-color: var(--danger); opacity: {loading ? '0.7' : '1'};"
		>
			{#if loading}
				<span class="flex items-center gap-2">
					<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
					Loading...
				</span>
			{:else}
				{confirmLabel}
			{/if}
		</button>
	{/snippet}
</Modal>
