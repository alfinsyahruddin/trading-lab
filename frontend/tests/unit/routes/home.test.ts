import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import HomePage from '../../../src/routes/+page.svelte';
import * as session from '$lib/helpers/session';

describe('HomePage (+page.svelte)', () => {
	beforeEach(() => {
		localStorage.clear();
		vi.clearAllMocks();
	});

	it('renders slogan, Sign In and Create Account buttons when logged out', () => {
		render(HomePage);

		expect(
			screen.getByText('Everyone built a stock screener, but no one ever backtested it!')
		).toBeInTheDocument();

		const signInLink = screen.getByRole('link', { name: /Sign In/i });
		const createAccountLink = screen.getByRole('link', { name: /Create Account/i });

		expect(signInLink).toBeInTheDocument();
		expect(signInLink).toHaveAttribute('href', '/login');
		expect(createAccountLink).toBeInTheDocument();
		expect(createAccountLink).toHaveAttribute('href', '/register');
		expect(screen.queryByRole('link', { name: /Go to Dashboard/i })).not.toBeInTheDocument();
	});

	it('renders Go to Dashboard button when logged in without redirecting', () => {
		session.persistSession('token-123', 'refresh-123', {
			id: 'u-1',
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER',
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		});

		render(HomePage);

		const dashboardLink = screen.getByRole('link', { name: /Go to Dashboard/i });
		expect(dashboardLink).toBeInTheDocument();
		expect(dashboardLink).toHaveAttribute('href', '/dashboard');
		expect(screen.queryByRole('link', { name: /Sign In/i })).not.toBeInTheDocument();
		expect(screen.queryByRole('link', { name: /Create Account/i })).not.toBeInTheDocument();
	});
});
