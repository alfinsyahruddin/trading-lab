import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import LandingFooter from '../../../../src/lib/components/landing/LandingFooter.svelte';

describe('LandingFooter', () => {
	it('renders logo, copyright, and unauthenticated links when isLoggedIn is false', () => {
		render(LandingFooter, { isLoggedIn: false });

		const signInLink = screen.getByRole('link', { name: /sign in/i });
		const getStartedLink = screen.getByRole('link', { name: /get started/i });

		expect(signInLink).toHaveAttribute('href', '/login');
		expect(getStartedLink).toHaveAttribute('href', '/register');
		expect(screen.queryByRole('link', { name: /dashboard/i })).not.toBeInTheDocument();
	});

	it('renders dashboard link when isLoggedIn is true', () => {
		render(LandingFooter, { isLoggedIn: true });

		const dashboardLink = screen.getByRole('link', { name: /dashboard/i });
		expect(dashboardLink).toHaveAttribute('href', '/dashboard');
		expect(screen.queryByRole('link', { name: /sign in/i })).not.toBeInTheDocument();
		expect(screen.queryByRole('link', { name: /get started/i })).not.toBeInTheDocument();
	});

	it('triggers oncomingsoon and prevents navigation when PUBLIC_IS_COMING_SOON is true', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';
		const oncomingsoon = vi.fn();
		render(LandingFooter, { isLoggedIn: false, oncomingsoon });

		const signInLink = screen.getByRole('link', { name: /sign in/i });
		const getStartedLink = screen.getByRole('link', { name: /get started/i });

		await fireEvent.click(signInLink);
		expect(oncomingsoon).toHaveBeenCalledTimes(1);

		await fireEvent.click(getStartedLink);
		expect(oncomingsoon).toHaveBeenCalledTimes(2);
	});

	it('does not trigger oncomingsoon when PUBLIC_IS_COMING_SOON is false', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'false';
		const oncomingsoon = vi.fn();
		render(LandingFooter, { isLoggedIn: false, oncomingsoon });

		const getStartedLink = screen.getByRole('link', { name: /get started/i });
		await fireEvent.click(getStartedLink);
		expect(oncomingsoon).not.toHaveBeenCalled();
	});
});
