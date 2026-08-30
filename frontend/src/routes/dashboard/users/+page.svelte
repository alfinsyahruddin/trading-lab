<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import TextField from '$lib/components/TextField.svelte';
	import SelectField from '$lib/components/SelectField.svelte';
	import RoleBadge from '$lib/components/RoleBadge.svelte';
	import { listUsers, createUser, updateUser, deleteUser } from '$lib/api';
	import { ApiError } from '$lib/api';
	import { getToken, getUser } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast';
	import type { UserResponse, UserRole } from '$lib/types';

	// ── State ──────────────────────────────────────────────────────────────
	let users = $state<UserResponse[]>([]);
	let loading = $state(true);
	let currentUser = $state<UserResponse | null>(null);

	// Create modal
	let createOpen = $state(false);
	let createLoading = $state(false);
	let createError = $state('');
	let createName = $state('');
	let createEmail = $state('');
	let createPassword = $state('');
	let createRole = $state<UserRole>('MEMBER');

	// Edit modal
	let editOpen = $state(false);
	let editLoading = $state(false);
	let editError = $state('');
	let editTarget = $state<UserResponse | null>(null);
	let editName = $state('');
	let editEmail = $state('');
	let editPassword = $state('');
	let editRole = $state<UserRole>('MEMBER');

	// Delete modal
	let deleteOpen = $state(false);
	let deleteLoading = $state(false);
	let deleteTarget = $state<UserResponse | null>(null);

	// ── Table columns ──────────────────────────────────────────────────────
	const columns = [
		{ key: 'name', label: 'Name' },
		{ key: 'email', label: 'Email' },
		{ key: 'role', label: 'Role' },
		{
			key: 'created_at',
			label: 'Created',
			render: (row: UserResponse) =>
				new Date(row.created_at).toLocaleDateString('en-US', {
					year: 'numeric',
					month: 'short',
					day: 'numeric'
				})
		}
	];

	const roleOptions = [
		{ label: 'Member', value: 'MEMBER' },
		{ label: 'Admin', value: 'ADMIN' }
	];

	// ── Lifecycle ──────────────────────────────────────────────────────────
	onMount(async () => {
		currentUser = getUser();
		// Client-side role guard
		if (currentUser?.role !== 'ADMIN') {
			goto('/dashboard');
			return;
		}
		await loadUsers();
	});

	async function loadUsers() {
		loading = true;
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}
			users = await listUsers(token);
		} catch (err) {
			if (err instanceof ApiError) {
				toast.error(err.message);
			} else {
				toast.error('Failed to load users.');
			}
		} finally {
			loading = false;
		}
	}

	// ── Create ─────────────────────────────────────────────────────────────
	function openCreate() {
		createName = '';
		createEmail = '';
		createPassword = '';
		createRole = 'MEMBER';
		createError = '';
		createOpen = true;
	}

	async function handleCreate(e: Event) {
		e.preventDefault();
		createError = '';
		if (createPassword.length < 8) {
			createError = 'Password must be at least 8 characters.';
			return;
		}
		createLoading = true;
		try {
			const token = getToken()!;
			const newUser = await createUser(token, {
				name: createName.trim(),
				email: createEmail.trim().toLowerCase(),
				password: createPassword,
				role: createRole
			});
			users = [...users, newUser];
			createOpen = false;
			toast.success(`User "${newUser.name}" created.`);
		} catch (err) {
			createError = err instanceof ApiError ? err.message : 'Failed to create user.';
		} finally {
			createLoading = false;
		}
	}

	// ── Edit ───────────────────────────────────────────────────────────────
	function openEdit(user: UserResponse) {
		editTarget = user;
		editName = user.name;
		editEmail = user.email;
		editPassword = '';
		editRole = user.role;
		editError = '';
		editOpen = true;
	}

	async function handleEdit(e: Event) {
		e.preventDefault();
		if (!editTarget) return;
		editError = '';
		if (editPassword && editPassword.length < 8) {
			editError = 'New password must be at least 8 characters.';
			return;
		}
		editLoading = true;
		try {
			const token = getToken()!;
			const payload: { name?: string; email?: string; password?: string; role?: UserRole } = {};
			if (editName.trim() !== editTarget.name) payload.name = editName.trim();
			if (editEmail.trim().toLowerCase() !== editTarget.email)
				payload.email = editEmail.trim().toLowerCase();
			if (editPassword) payload.password = editPassword;
			if (editRole !== editTarget.role) payload.role = editRole;

			const updated = await updateUser(token, editTarget.id, payload);
			users = users.map((u) => (u.id === updated.id ? updated : u));
			editOpen = false;
			toast.success(`User "${updated.name}" updated.`);
		} catch (err) {
			editError = err instanceof ApiError ? err.message : 'Failed to update user.';
		} finally {
			editLoading = false;
		}
	}

	// ── Delete ─────────────────────────────────────────────────────────────
	function openDelete(user: UserResponse) {
		deleteTarget = user;
		deleteOpen = true;
	}

	async function handleDelete() {
		if (!deleteTarget) return;
		deleteLoading = true;
		try {
			const token = getToken()!;
			await deleteUser(token, deleteTarget.id);
			const name = deleteTarget.name;
			users = users.filter((u) => u.id !== deleteTarget!.id);
			deleteOpen = false;
			deleteTarget = null;
			toast.success(`User "${name}" deleted.`);
		} catch (err) {
			const msg = err instanceof ApiError ? err.message : 'Failed to delete user.';
			toast.error(msg);
			deleteOpen = false;
		} finally {
			deleteLoading = false;
		}
	}
</script>

<!-- Page header -->
<div class="mb-6 flex items-center justify-between">
	<div>
		<h1 class="text-2xl font-700" style="color: var(--fg)">Users</h1>
		<p class="mt-0.5 text-sm" style="color: var(--fg-muted)">
			Manage platform users and their roles.
		</p>
	</div>
	<button
		onclick={openCreate}
		class="flex items-center gap-2 rounded-xl px-4 py-2.5 text-sm font-600 text-white transition-opacity duration-150 hover:opacity-90"
		style="background-color: var(--accent);"
	>
		<svg
			width="16"
			height="16"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2.5"
		>
			<path d="M12 5v14M5 12h14" />
		</svg>
		New User
	</button>
</div>

<!-- User count -->
{#if !loading}
	<p class="mb-4 text-xs font-500 uppercase tracking-wider" style="color: var(--fg-muted)">
		{users.length}
		{users.length === 1 ? 'user' : 'users'}
	</p>
{/if}

<!-- Data table -->
<DataTable
	{columns}
	rows={users}
	{loading}
	emptyMessage="No users found. Create one to get started."
>
	{#snippet actions(row: UserResponse)}
		<div class="flex items-center justify-end gap-1">
			<!-- Edit button -->
			<button
				onclick={() => openEdit(row)}
				class="flex h-8 w-8 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-(--bg-card-hover)"
				style="color: var(--fg-muted);"
				title="Edit user"
				aria-label="Edit {row.name}"
			>
				<svg
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7" />
					<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z" />
				</svg>
			</button>
			<!-- Delete button (don't allow deleting self) -->
			{#if row.id !== currentUser?.id}
				<button
					onclick={() => openDelete(row)}
					class="flex h-8 w-8 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950"
					style="color: var(--fg-muted);"
					title="Delete user"
					aria-label="Delete {row.name}"
				>
					<svg
						width="14"
						height="14"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polyline points="3 6 5 6 21 6" />
						<path d="M19 6l-1 14H6L5 6" />
						<path d="M10 11v6M14 11v6" />
						<path d="M9 6V4h6v2" />
					</svg>
				</button>
			{/if}
		</div>
	{/snippet}
</DataTable>

<!-- ── Create Modal ─────────────────────────────────────────────────────── -->
<Modal bind:open={createOpen} title="New User">
	{#snippet children()}
		<form id="create-form" onsubmit={handleCreate} class="flex flex-col gap-4">
			<TextField
				label="Full Name"
				type="text"
				placeholder="John Doe"
				bind:value={createName}
				required
			/>
			<TextField
				label="Email"
				type="email"
				placeholder="john@example.com"
				bind:value={createEmail}
				required
			/>
			<TextField
				label="Password"
				type="password"
				placeholder="Min. 8 characters"
				bind:value={createPassword}
				required
			/>
			<SelectField label="Role" bind:value={createRole} options={roleOptions} />

			{#if createError}
				<div
					class="rounded-lg border px-4 py-3 text-sm"
					style="background-color: rgba(239,68,68,0.08); border-color: rgba(239,68,68,0.3); color: var(--danger);"
				>
					{createError}
				</div>
			{/if}
		</form>
	{/snippet}
	{#snippet footer()}
		<button
			onclick={() => (createOpen = false)}
			class="rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="create-form"
			disabled={createLoading}
			class="rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
			style="background-color: var(--accent); opacity: {createLoading ? '0.7' : '1'};"
		>
			{createLoading ? 'Creating…' : 'Create User'}
		</button>
	{/snippet}
</Modal>

<!-- ── Edit Modal ───────────────────────────────────────────────────────── -->
<Modal bind:open={editOpen} title="Edit User">
	{#snippet children()}
		<form id="edit-form" onsubmit={handleEdit} class="flex flex-col gap-4">
			<TextField label="Full Name" type="text" bind:value={editName} required />
			<TextField label="Email" type="email" bind:value={editEmail} required />
			<TextField
				label="New Password"
				type="password"
				placeholder="Leave blank to keep current"
				bind:value={editPassword}
			/>
			<SelectField label="Role" bind:value={editRole} options={roleOptions} />

			{#if editError}
				<div
					class="rounded-lg border px-4 py-3 text-sm"
					style="background-color: rgba(239,68,68,0.08); border-color: rgba(239,68,68,0.3); color: var(--danger);"
				>
					{editError}
				</div>
			{/if}
		</form>
	{/snippet}
	{#snippet footer()}
		<button
			onclick={() => (editOpen = false)}
			class="rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="edit-form"
			disabled={editLoading}
			class="rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
			style="background-color: var(--accent); opacity: {editLoading ? '0.7' : '1'};"
		>
			{editLoading ? 'Saving…' : 'Save Changes'}
		</button>
	{/snippet}
</Modal>

<!-- ── Delete Confirm Modal ─────────────────────────────────────────────── -->
<ConfirmModal
	bind:open={deleteOpen}
	title="Delete User"
	message={`Are you sure you want to delete "${deleteTarget?.name}"? This action cannot be undone.`}
	confirmLabel="Delete"
	loading={deleteLoading}
	onconfirm={handleDelete}
/>
