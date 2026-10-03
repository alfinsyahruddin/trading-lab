<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import Modal from './Modal.svelte';
	import TextField from './TextField.svelte';
	import { changePassword, ApiError } from '#lib/api.js';
	import { clearSession } from '#lib/helpers/session.js';
	import { toast } from '#lib/helpers/toast.svelte.js';

	let {
		open = $bindable(false)
	}: {
		open?: boolean;
	} = $props();

	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');

	let currentPasswordError = $state('');
	let newPasswordError = $state('');
	let confirmPasswordError = $state('');
	let generalError = $state('');
	let loading = $state(false);

	$effect(() => {
		if (open) {
			currentPassword = '';
			newPassword = '';
			confirmPassword = '';
			currentPasswordError = '';
			newPasswordError = '';
			confirmPasswordError = '';
			generalError = '';
		}
	});

	function validate(): boolean {
		let valid = true;
		currentPasswordError = '';
		newPasswordError = '';
		confirmPasswordError = '';
		generalError = '';

		if (!currentPassword) {
			currentPasswordError = 'Current password is required';
			valid = false;
		}

		if (!newPassword) {
			newPasswordError = 'New password is required';
			valid = false;
		} else if (newPassword.length < 8) {
			newPasswordError = 'New password must be at least 8 characters';
			valid = false;
		}

		if (!confirmPassword) {
			confirmPasswordError = 'Please confirm your new password';
			valid = false;
		} else if (confirmPassword !== newPassword) {
			confirmPasswordError = 'Passwords do not match';
			valid = false;
		}

		return valid;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!validate() || loading) return;

		loading = true;
		try {
			await changePassword({
				current_password: currentPassword,
				new_password: newPassword
			});
			toast.success('Password changed successfully. Please log in again.');
			open = false;
			clearSession();
			goto('/login');
		} catch (err) {
			if (err instanceof ApiError) {
				generalError = err.message;
				toast.error(err.message);
			} else {
				generalError = 'Failed to change password. Please try again.';
				toast.error(generalError);
			}
		} finally {
			loading = false;
		}
	}
</script>

<Modal bind:open title="Change Password">
	{#snippet children()}
		<form id="change-password-form" onsubmit={handleSubmit} class="flex flex-col gap-4">
			{#if generalError}
				<div
					class="rounded-lg p-3 text-sm"
					style="background-color: rgba(239, 68, 68, 0.1); color: var(--danger); border: 1px solid rgba(239, 68, 68, 0.2);"
				>
					{generalError}
				</div>
			{/if}

			<TextField
				label="Current Password"
				type="password"
				placeholder="••••••••"
				bind:value={currentPassword}
				error={currentPasswordError}
				required
				disabled={loading}
			/>

			<TextField
				label="New Password"
				type="password"
				placeholder="At least 8 characters"
				bind:value={newPassword}
				error={newPasswordError}
				required
				disabled={loading}
			/>

			<TextField
				label="Confirm New Password"
				type="password"
				placeholder="Repeat new password"
				bind:value={confirmPassword}
				error={confirmPasswordError}
				required
				disabled={loading}
			/>
		</form>
	{/snippet}

	{#snippet footer()}
		<button
			type="button"
			onclick={() => (open = false)}
			disabled={loading}
			class="btn-interactive font-500 w-full rounded-lg border px-4 py-2 text-sm transition-colors duration-150 hover:bg-(--bg-card-hover) disabled:opacity-50 sm:w-auto"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="change-password-form"
			disabled={loading}
			class="btn-interactive font-600 inline-flex w-full items-center justify-center rounded-lg px-4 py-2 text-sm text-white transition-opacity duration-150 sm:w-auto"
			style="background-color: var(--accent); opacity: {loading ? '0.7' : '1'};"
		>
			{#if loading}
				<span class="flex items-center gap-2">
					<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
					Updating...
				</span>
			{:else}
				Change Password
			{/if}
		</button>
	{/snippet}
</Modal>
