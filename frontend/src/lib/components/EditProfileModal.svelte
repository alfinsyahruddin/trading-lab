<script lang="ts">
	import Icon from '@iconify/svelte';
	import Modal from './Modal.svelte';
	import TextField from './TextField.svelte';
	import { updateProfile, ApiError } from '$lib/api';
	import { updateUserSession } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { UserResponse } from '$lib/types';

	let {
		open = $bindable(false),
		user = null,
		onsuccess
	}: {
		open?: boolean;
		user?: UserResponse | null;
		onsuccess?: (updatedUser: UserResponse) => void;
	} = $props();

	let name = $state('');
	let email = $state('');
	let nameError = $state('');
	let emailError = $state('');
	let generalError = $state('');
	let loading = $state(false);

	$effect(() => {
		if (open && user) {
			name = user.name;
			email = user.email;
			nameError = '';
			emailError = '';
			generalError = '';
		}
	});

	function validate(): boolean {
		let valid = true;
		nameError = '';
		emailError = '';
		generalError = '';

		if (!name.trim()) {
			nameError = 'Name is required';
			valid = false;
		}

		if (!email.trim()) {
			emailError = 'Email is required';
			valid = false;
		} else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email.trim())) {
			emailError = 'Please enter a valid email address';
			valid = false;
		}

		return valid;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!validate() || loading) return;

		loading = true;
		try {
			const updated = await updateProfile({
				name: name.trim(),
				email: email.trim().toLowerCase()
			});
			updateUserSession(updated);
			toast.success('Profile updated successfully.');
			onsuccess?.(updated);
			open = false;
		} catch (err) {
			if (err instanceof ApiError) {
				generalError = err.message;
				toast.error(err.message);
			} else {
				generalError = 'Failed to update profile. Please try again.';
				toast.error(generalError);
			}
		} finally {
			loading = false;
		}
	}
</script>

<Modal bind:open title="Edit Profile">
	{#snippet children()}
		<form id="edit-profile-form" onsubmit={handleSubmit} class="flex flex-col gap-4">
			{#if generalError}
				<div
					class="rounded-lg p-3 text-sm"
					style="background-color: rgba(239, 68, 68, 0.1); color: var(--danger); border: 1px solid rgba(239, 68, 68, 0.2);"
				>
					{generalError}
				</div>
			{/if}

			<TextField
				label="Full Name"
				placeholder="John Doe"
				bind:value={name}
				error={nameError}
				required
				disabled={loading}
			/>

			<TextField
				label="Email Address"
				type="email"
				placeholder="name@example.com"
				bind:value={email}
				error={emailError}
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
			class="btn-interactive w-full sm:w-auto rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) disabled:opacity-50"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="edit-profile-form"
			disabled={loading}
			class="btn-interactive w-full sm:w-auto inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
			style="background-color: var(--accent); opacity: {loading ? '0.7' : '1'};"
		>
			{#if loading}
				<span class="flex items-center gap-2">
					<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
					Saving...
				</span>
			{:else}
				Save Changes
			{/if}
		</button>
	{/snippet}
</Modal>
