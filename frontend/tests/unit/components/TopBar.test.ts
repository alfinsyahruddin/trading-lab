import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import TopBar from '$lib/components/TopBar.svelte';
import * as session from '$lib/helpers/session';
import type { UserResponse } from '$lib/types';

vi.mock('$app/stores', () => ({
	page: {
		subscribe: (fn: (val: { url: URL }) => void) => {
			fn({ url: new URL('http://localhost/dashboard') });
			return () => {};
		}
	}
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

const mockUser: UserResponse = {
	id: '123e4567-e89b-12d3-a456-426614174000',
	name: 'Alice Trader',
	email: 'alice@example.com',
	role: 'MEMBER',
	created_at: '2026-08-30T00:00:00Z',
	updated_at: '2026-08-30T00:00:00Z'
};

describe('TopBar', () => {
	beforeEach(() => {
		session.persistSession('token', 'refresh', mockUser);
	});

	it('renders user name and avatar button, with no standalone topbar logout button', () => {
		render(TopBar);
		expect(screen.getByText('Alice Trader')).toBeInTheDocument();
		const avatarBtn = screen.getByRole('button', { name: /user menu/i });
		expect(avatarBtn).toBeInTheDocument();
		// Standalone logout button is not open initially
		expect(screen.queryByRole('menuitem', { name: /logout/i })).not.toBeInTheDocument();
	});

	it('shows popover menu with Edit Profile, Change Password, and Logout on avatar click', async () => {
		render(TopBar);
		const avatarBtn = screen.getByRole('button', { name: /user menu/i });
		await fireEvent.click(avatarBtn);

		const editProfileBtn = screen.getByRole('menuitem', { name: /edit profile/i });
		const changePasswordBtn = screen.getByRole('menuitem', { name: /change password/i });
		const logoutBtn = screen.getByRole('menuitem', { name: /logout/i });

		expect(editProfileBtn).toBeInTheDocument();
		expect(changePasswordBtn).toBeInTheDocument();
		expect(logoutBtn).toBeInTheDocument();

		// Logout button has danger red styling
		expect(logoutBtn.getAttribute('style')).toContain('var(--danger)');
	});

	it('opens Edit Profile modal when Edit Profile menuitem is clicked', async () => {
		render(TopBar);
		const avatarBtn = screen.getByRole('button', { name: /user menu/i });
		await fireEvent.click(avatarBtn);

		const editProfileBtn = screen.getByRole('menuitem', { name: /edit profile/i });
		await fireEvent.click(editProfileBtn);

		expect(screen.getByRole('heading', { name: 'Edit Profile' })).toBeInTheDocument();
	});

	it('opens Change Password modal when Change Password menuitem is clicked', async () => {
		render(TopBar);
		const avatarBtn = screen.getByRole('button', { name: /user menu/i });
		await fireEvent.click(avatarBtn);

		const changePasswordBtn = screen.getByRole('menuitem', { name: /change password/i });
		await fireEvent.click(changePasswordBtn);

		expect(screen.getByRole('heading', { name: 'Change Password' })).toBeInTheDocument();
	});

	it('renders desktop navigation links with active state for current page', () => {
		render(TopBar);
		const dashboardLink = screen.getByRole('link', { name: /dashboard/i });
		const strategiesLink = screen.getByRole('link', { name: /trading strategy/i });
		const backtestsLink = screen.getByRole('link', { name: /backtest/i });

		expect(dashboardLink).toBeInTheDocument();
		expect(strategiesLink).toBeInTheDocument();
		expect(backtestsLink).toBeInTheDocument();

		expect(dashboardLink).toHaveAttribute('aria-current', 'page');
		expect(strategiesLink).not.toHaveAttribute('aria-current');
	});

	it('renders Users and Settings navigation links for ADMIN role', () => {
		const adminUser: UserResponse = {
			...mockUser,
			role: 'ADMIN'
		};
		session.persistSession('token', 'refresh', adminUser);
		render(TopBar);

		const usersLink = screen.getByRole('link', { name: /users/i });
		const settingsLink = screen.getByRole('link', { name: /settings/i });

		expect(usersLink).toBeInTheDocument();
		expect(settingsLink).toBeInTheDocument();
	});
});
