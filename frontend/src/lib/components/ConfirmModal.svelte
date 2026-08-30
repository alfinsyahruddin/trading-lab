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
			class="btn-interactive w-full sm:w-auto rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			{cancelLabel}
		</button>
		<button
			onclick={handleConfirm}
			disabled={loading}
			class="btn-interactive w-full sm:w-auto inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
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
