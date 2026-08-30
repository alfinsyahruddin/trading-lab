<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import TextField from '$lib/components/TextField.svelte';
	import SelectField from '$lib/components/SelectField.svelte';
	import RoleBadge from '$lib/components/RoleBadge.svelte';
	import { listUsers, createUser, updateUser, deleteUser } from '$lib/api';
	import { ApiError } from '$lib/api';
	import { getToken, getUser } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
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

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		if (isNaN(d.getTime())) return dateStr;
		const weekday = d.toLocaleDateString('en-US', { weekday: 'short' });
		const day = d.getDate();
		const month = d.toLocaleDateString('en-US', { month: 'short' });
		const year = d.getFullYear();
		const hours = String(d.getHours()).padStart(2, '0');
		const minutes = String(d.getMinutes()).padStart(2, '0');
		return `${weekday}, ${day} ${month} ${year} • ${hours}.${minutes}`;
	}

	// ── Table columns ──────────────────────────────────────────────────────
	const columns = [
		{ key: 'name', label: 'Name' },
		{ key: 'email', label: 'Email' },
		{ key: 'role', label: 'Role' },
		{
			key: 'created_at',
			label: 'Created',
			render: (row: UserResponse) => formatDate(row.created_at)
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
<div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
	<div>
		<h1 class="text-2xl font-700" style="color: var(--fg)">Users</h1>
		<p class="mt-0.5 text-sm" style="color: var(--fg-muted)">
			Manage platform users and their roles.
		</p>
	</div>
	<button
		onclick={openCreate}
		class="btn-interactive inline-flex items-center justify-center gap-2 rounded-xl px-4 py-2.5 text-sm font-600 text-white shadow-sm hover:shadow-md hover:opacity-95 active:scale-95 w-full sm:w-auto"
		style="background-color: var(--accent);"
	>
		<Icon icon="lucide:plus" width="16" height="16" />
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
	{#snippet cell(row: UserResponse, col)}
		{#if col.key === 'role'}
			<RoleBadge role={row.role} size="sm" />
		{:else if col.key === 'name'}
			<span class="font-600">{row.name}</span>
		{:else if col.key === 'email'}
			<span class="text-xs sm:text-sm">{row.email}</span>
		{:else if col.render}
			{col.render(row)}
		{:else}
			{(row as unknown as Record<string, unknown>)[col.key as string] ?? '—'}
		{/if}
	{/snippet}
	{#snippet actions(row: UserResponse)}
		<div class="flex items-center justify-end gap-1">
			<!-- Edit button -->
			<button
				onclick={() => openEdit(row)}
				class="btn-interactive flex h-8 w-8 items-center justify-center rounded-lg transition-all duration-150 hover:scale-110 hover:bg-(--bg-card-hover) active:scale-90"
				style="color: var(--fg-muted);"
				title="Edit user"
				aria-label="Edit {row.name}"
			>
				<Icon icon="lucide:pencil" width="14" height="14" />
			</button>
			<!-- Delete button (don't allow deleting self) -->
			{#if row.id !== currentUser?.id}
				<button
					onclick={() => openDelete(row)}
					class="btn-interactive flex h-8 w-8 items-center justify-center rounded-lg transition-all duration-150 hover:scale-110 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950 active:scale-90"
					style="color: var(--fg-muted);"
					title="Delete user"
					aria-label="Delete {row.name}"
				>
					<Icon icon="lucide:trash-2" width="14" height="14" />
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
			class="btn-interactive w-full sm:w-auto rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="create-form"
			disabled={createLoading}
			class="btn-interactive w-full sm:w-auto inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
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
			class="btn-interactive w-full sm:w-auto rounded-lg border px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			form="edit-form"
			disabled={editLoading}
			class="btn-interactive w-full sm:w-auto inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-600 text-white transition-opacity duration-150"
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
