import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import LandingCtaButtons from '../../../../src/lib/components/landing/LandingCtaButtons.svelte';

describe('LandingCtaButtons', () => {
	it('renders unauthenticated buttons when isLoggedIn is false', () => {
		render(LandingCtaButtons, { isLoggedIn: false });

		const registerBtn = screen.getByRole('link', { name: /start backtesting/i });
		const signInBtn = screen.getByRole('link', { name: /sign in/i });

		expect(registerBtn).toHaveAttribute('href', '/register');
		expect(signInBtn).toHaveAttribute('href', '/login');
		expect(screen.queryByRole('link', { name: /dashboard/i })).not.toBeInTheDocument();
	});

	it('renders authenticated dashboard button when isLoggedIn is true', () => {
		render(LandingCtaButtons, { isLoggedIn: true });

		const dashboardBtn = screen.getByRole('link', { name: /go to dashboard/i });
		expect(dashboardBtn).toHaveAttribute('href', '/dashboard');
		expect(screen.queryByRole('link', { name: /sign in/i })).not.toBeInTheDocument();
		expect(screen.queryByRole('link', { name: /register/i })).not.toBeInTheDocument();
	});

	it('renders custom text and large size variant', () => {
		render(LandingCtaButtons, {
			isLoggedIn: false,
			primaryUnauthText: 'Create Free Account',
			secondaryText: 'Login Now',
			size: 'lg'
		});

		const primary = screen.getByRole('link', { name: /create free account/i });
		const secondary = screen.getByRole('link', { name: /login now/i });

		expect(primary).toBeInTheDocument();
		expect(secondary).toBeInTheDocument();
		expect(primary).toHaveClass('sm:text-base');
	});
});
