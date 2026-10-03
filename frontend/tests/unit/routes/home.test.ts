import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import HomePage from '../../../src/routes/+page.svelte';
import * as session from '#lib/helpers/session.js';

describe('HomePage (+page.svelte)', () => {
	beforeEach(() => {
		localStorage.clear();
		session.clearSession();
		vi.clearAllMocks();
	});

	it('renders slogan, Sign In and Start Backtesting buttons when logged out', () => {
		render(HomePage);

		expect(
			screen.getByRole('heading', {
				level: 1,
				name: /everyone built a/i
			})
		).toBeInTheDocument();

		const signInLinks = screen.getAllByRole('link', { name: /Sign In/i });
		const startLinks = screen.getAllByRole('link', {
			name: /Start Backtesting|Create Free Account|Get Started/i
		});

		expect(signInLinks.length).toBeGreaterThanOrEqual(1);
		expect(signInLinks[0]).toHaveAttribute('href', '/login');
		expect(startLinks.length).toBeGreaterThanOrEqual(1);
		expect(startLinks[0]).toHaveAttribute('href', '/register');
		expect(screen.queryByRole('link', { name: /Dashboard/i })).not.toBeInTheDocument();
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
		expect(
			screen.queryByRole('link', { name: /Start Backtesting|Create Free Account|Get Started/i })
		).not.toBeInTheDocument();
	});
});
