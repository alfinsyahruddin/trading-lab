import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import UsersPage from '../../../src/routes/dashboard/users/+page.svelte';
import * as api from '$lib/api';
import * as session from '$lib/helpers/session';
import { toast } from '$lib/helpers/toast.svelte';
import type { UserResponse } from '$lib/types';

const mockGoto = vi.fn();
vi.mock('$app/navigation', () => ({
	goto: (...args: unknown[]) => mockGoto(...args)
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		listUsers: vi.fn(),
		createUser: vi.fn(),
		updateUser: vi.fn(),
		deleteUser: vi.fn()
	};
});

describe('UsersPage (Admin User Management)', () => {
	const adminUser: UserResponse = {
		id: 'u-admin',
		name: 'Admin Boss',
		email: 'admin@tradinglab.id',
		role: 'ADMIN',
		created_at: '2026-01-01T00:00:00Z',
		updated_at: '2026-01-01T00:00:00Z'
	};

	const memberUser: UserResponse = {
		id: 'u-member-1',
		name: 'Trader John',
		email: 'john@tradinglab.id',
		role: 'MEMBER',
		created_at: '2026-02-01T00:00:00Z',
		updated_at: '2026-02-01T00:00:00Z'
	};

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('admin-token');
		vi.spyOn(session, 'getUser').mockReturnValue(adminUser);
		vi.mocked(api.listUsers).mockResolvedValue([adminUser, memberUser]);
	});

	it('redirects non-admin users to /dashboard', () => {
		vi.spyOn(session, 'getUser').mockReturnValue(memberUser);

		render(UsersPage);

		expect(mockGoto).toHaveBeenCalledWith('/dashboard');
		expect(api.listUsers).not.toHaveBeenCalled();
	});

	it('loads and renders users in DataTable with appropriate badges', async () => {
		render(UsersPage);

		expect(await screen.findByText('Admin Boss')).toBeInTheDocument();
		expect(screen.getByText('admin@tradinglab.id')).toBeInTheDocument();
		expect(screen.getByText('Trader John')).toBeInTheDocument();
		expect(screen.getByText('john@tradinglab.id')).toBeInTheDocument();
		expect(screen.getByText('2 users')).toBeInTheDocument();
	});

	it('prevents self-deletion: does not render delete button for current user', async () => {
		render(UsersPage);

		expect(await screen.findByText('Admin Boss')).toBeInTheDocument();

		// Should have delete button for Trader John but not for Admin Boss
		expect(screen.getByLabelText('Delete Trader John')).toBeInTheDocument();
		expect(screen.queryByLabelText('Delete Admin Boss')).not.toBeInTheDocument();
	});

	it('creates a new user when New User modal is filled and submitted', async () => {
		const newUser: UserResponse = {
			id: 'u-new',
			name: 'Tokyo Drift',
			email: 'tokyo@tradinglab.id',
			role: 'MEMBER',
			created_at: '2026-03-01T00:00:00Z',
			updated_at: '2026-03-01T00:00:00Z'
		};
		vi.mocked(api.createUser).mockResolvedValue(newUser);
		const toastSuccessSpy = vi.spyOn(toast, 'success');

		render(UsersPage);

		expect(await screen.findByText('Admin Boss')).toBeInTheDocument();

		const openCreateBtn = screen.getByRole('button', { name: /new user/i });
		await fireEvent.click(openCreateBtn);

		expect(screen.getByRole('heading', { name: 'New User' })).toBeInTheDocument();

		const nameInput = screen.getByPlaceholderText('Tokyo');
		const emailInput = screen.getByPlaceholderText('tokyo@mail.com');
		const passInput = screen.getByPlaceholderText('Min. 8 characters');

		await fireEvent.input(nameInput, { target: { value: 'Tokyo Drift' } });
		await fireEvent.input(emailInput, { target: { value: 'tokyo@tradinglab.id' } });
		await fireEvent.input(passInput, { target: { value: 'secretpass123' } });

		const submitBtn = screen.getByRole('button', { name: 'Create User' });
		await fireEvent.click(submitBtn);

		await waitFor(() => {
			expect(api.createUser).toHaveBeenCalledWith('admin-token', {
				name: 'Tokyo Drift',
				email: 'tokyo@tradinglab.id',
				password: 'secretpass123',
				role: 'MEMBER'
			});
			expect(toastSuccessSpy).toHaveBeenCalledWith('User "Tokyo Drift" created.');
			expect(screen.getByText('Tokyo Drift')).toBeInTheDocument();
		});
	});

	it('edits a user when edit modal is opened and submitted', async () => {
		const updatedUser: UserResponse = {
			...memberUser,
			name: 'Trader John Updated'
		};
		vi.mocked(api.updateUser).mockResolvedValue(updatedUser);
		const toastSuccessSpy = vi.spyOn(toast, 'success');

		render(UsersPage);

		expect(await screen.findByText('Trader John')).toBeInTheDocument();

		const editBtn = screen.getByLabelText('Edit Trader John');
		await fireEvent.click(editBtn);

		expect(screen.getByRole('heading', { name: 'Edit User' })).toBeInTheDocument();

		const nameInput = screen.getByDisplayValue('Trader John');
		await fireEvent.input(nameInput, { target: { value: 'Trader John Updated' } });

		const saveBtn = screen.getByRole('button', { name: 'Save Changes' });
		await fireEvent.click(saveBtn);

		await waitFor(() => {
			expect(api.updateUser).toHaveBeenCalledWith('admin-token', 'u-member-1', {
				name: 'Trader John Updated'
			});
			expect(toastSuccessSpy).toHaveBeenCalledWith('User "Trader John Updated" updated.');
			expect(screen.getByText('Trader John Updated')).toBeInTheDocument();
		});
	});

	it('deletes a user when delete confirm modal is confirmed', async () => {
		vi.mocked(api.deleteUser).mockResolvedValue('Deleted successfully');
		const toastSuccessSpy = vi.spyOn(toast, 'success');

		render(UsersPage);

		expect(await screen.findByText('Trader John')).toBeInTheDocument();

		const deleteBtn = screen.getByLabelText('Delete Trader John');
		await fireEvent.click(deleteBtn);

		expect(screen.getByRole('heading', { name: 'Delete User' })).toBeInTheDocument();

		const confirmBtn = screen.getByRole('button', { name: 'Delete' });
		await fireEvent.click(confirmBtn);

		await waitFor(() => {
			expect(api.deleteUser).toHaveBeenCalledWith('admin-token', 'u-member-1');
			expect(toastSuccessSpy).toHaveBeenCalledWith('User "Trader John" deleted.');
			expect(screen.queryByText('Trader John')).not.toBeInTheDocument();
		});
	});
});
