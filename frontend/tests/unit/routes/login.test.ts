import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import LoginPage from '../../../src/routes/login/+page.svelte';
import * as session from '#lib/helpers/session.js';
import * as api from '#lib/api.js';
import { goto } from '$app/navigation';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('#lib/api.js', async (importOriginal) => {
	const actual = await importOriginal<typeof import('#lib/api.js')>();
	return {
		...actual,
		login: vi.fn()
	};
});

describe('LoginPage', () => {
	beforeEach(() => {
		localStorage.clear();
		vi.clearAllMocks();
		import.meta.env.PUBLIC_IS_COMING_SOON = 'false';
	});

	it('renders login form and Remember me checkbox', () => {
		render(LoginPage);
		expect(screen.getByLabelText(/^Email/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/^Password/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/Remember me/i)).toBeInTheDocument();
		// Quick login section should be hidden when list is empty
		expect(screen.queryByText('Quick Login')).not.toBeInTheDocument();
	});

	it('shows quick login list when remembered accounts exist', () => {
		session.saveRememberedAccount({
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER'
		});

		render(LoginPage);
		expect(screen.getByText('Quick Login')).toBeInTheDocument();
		expect(screen.getByText('joe@example.com')).toBeInTheDocument();
		expect(screen.getByText('MEMBER')).toBeInTheDocument();
		expect(
			screen.getByLabelText(/Remove joe@example.com from saved accounts/i)
		).toBeInTheDocument();
	});

	it('auto-logins when clicking a saved account with password', async () => {
		vi.mocked(api.login).mockResolvedValueOnce({
			user: {
				id: 'u-1',
				name: 'Trader Joe',
				email: 'joe@example.com',
				role: 'MEMBER',
				created_at: '2026-08-30T00:00:00Z',
				updated_at: '2026-08-30T00:00:00Z'
			},
			tokens: {
				access_token: 'access-123',
				refresh_token: 'refresh-123'
			}
		});

		session.saveRememberedAccount({
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER',
			password: 'securepassword123'
		});

		render(LoginPage);
		const accountItem = screen.getByText('joe@example.com');
		await fireEvent.click(accountItem);

		expect(api.login).toHaveBeenCalledWith('joe@example.com', 'securepassword123');
		expect(session.getToken()).toBe('access-123');
		expect(goto).toHaveBeenCalledWith('/dashboard');
	});

	it('populates email input when clicking a saved account without password', async () => {
		session.saveRememberedAccount({
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER'
		});

		render(LoginPage);
		const accountItem = screen.getByText('joe@example.com');
		await fireEvent.click(accountItem);

		const emailInput = screen.getByLabelText(/^Email/i) as HTMLInputElement;
		expect(emailInput.value).toBe('joe@example.com');
		expect(api.login).not.toHaveBeenCalled();
	});

	it('removes account from quick login and hides section when last account is removed', async () => {
		session.saveRememberedAccount({
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER'
		});

		render(LoginPage);
		const removeBtn = screen.getByLabelText(/Remove joe@example.com from saved accounts/i);
		await fireEvent.click(removeBtn);

		expect(screen.queryByText('joe@example.com')).not.toBeInTheDocument();
		expect(screen.queryByText('Quick Login')).not.toBeInTheDocument();
		expect(session.getRememberedAccounts()).toEqual([]);
	});

	it('shows coming soon modal and prevents login when PUBLIC_IS_COMING_SOON is true on form submit', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';

		render(LoginPage);
		const emailInput = screen.getByLabelText(/^Email/i);
		const passwordInput = screen.getByLabelText(/^Password/i);
		const submitBtn = screen.getByRole('button', { name: /sign in/i });

		await fireEvent.input(emailInput, { target: { value: 'tokyo@mail.com' } });
		await fireEvent.input(passwordInput, { target: { value: 'password123' } });
		await fireEvent.click(submitBtn);

		expect(api.login).not.toHaveBeenCalled();
		expect(screen.getByRole('dialog')).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /we're launching soon!/i })).toBeInTheDocument();
		expect(screen.getByText(/stay tuned for public release!/i)).toBeInTheDocument();
	});

	it('shows coming soon modal on quick login when PUBLIC_IS_COMING_SOON is true', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';

		session.saveRememberedAccount({
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER',
			password: 'securepassword123'
		});

		render(LoginPage);
		const accountItem = screen.getByText('joe@example.com');
		await fireEvent.click(accountItem);

		expect(api.login).not.toHaveBeenCalled();
		expect(screen.getByRole('dialog')).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /we're launching soon!/i })).toBeInTheDocument();
	});
});
