import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import LandingPage from '../../../src/routes/+page.svelte';
import * as session from '$lib/helpers/session';

// Mock matchMedia
Object.defineProperty(globalThis, 'matchMedia', {
	value: (query: string) => ({
		matches: query.includes('dark'),
		addListener: () => {},
		removeListener: () => {}
	})
});

// Mock IntersectionObserver
const mockObserve = vi.fn();
const mockUnobserve = vi.fn();
const mockDisconnect = vi.fn();

class MockIntersectionObserver {
	observe = mockObserve;
	unobserve = mockUnobserve;
	disconnect = mockDisconnect;
}
Object.defineProperty(globalThis, 'IntersectionObserver', {
	value: MockIntersectionObserver,
	configurable: true
});

describe('Landing Page (+page.svelte)', () => {
	beforeEach(() => {
		session.clearSession();
		mockObserve.mockClear();
		mockUnobserve.mockClear();
		mockDisconnect.mockClear();
	});

	it('renders theme toggle switch in top header row alongside logo', () => {
		render(LandingPage);
		const themeToggleBtn = screen.getByRole('button', { name: /toggle theme/i });
		expect(themeToggleBtn).toBeInTheDocument();

		const logos = screen.getAllByAltText('Trading Lab');
		expect(logos.length).toBeGreaterThanOrEqual(2); // header and footer logos
	});

	it('renders unauthenticated CTAs (Start Backtesting and Sign In) when user is not logged in', () => {
		render(LandingPage);
		const startBacktestingBtns = screen.getAllByRole('link', {
			name: /start backtesting|join trading lab|create free account/i
		});
		expect(startBacktestingBtns.length).toBeGreaterThanOrEqual(2);

		const signInLinks = screen.getAllByRole('link', { name: /sign in/i });
		expect(signInLinks.length).toBeGreaterThanOrEqual(2);
	});

	it('renders authenticated CTAs (Dashboard links) when user is logged in', () => {
		session.persistSession('token', 'refresh', {
			id: 'user-1',
			name: 'Test User',
			email: 'test@example.com',
			role: 'MEMBER',
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		});

		render(LandingPage);
		const dashboardLinks = screen.getAllByRole('link', { name: /dashboard/i });
		expect(dashboardLinks.length).toBeGreaterThanOrEqual(2);
	});

	it('renders platform feature highlights and workflow steps', () => {
		render(LandingPage);
		expect(screen.getByRole('heading', { name: /visual strategy builder/i })).toBeInTheDocument();
		expect(
			screen.getByRole('heading', { name: /(?:reliable|historical) backtest engine/i })
		).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /ai-powered/i })).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /deep analytics/i })).toBeInTheDocument();

		expect(screen.getByRole('heading', { name: /build your strategy/i })).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /run the backtest/i })).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /analyze and refine/i })).toBeInTheDocument();
	});

	it('renders screenshot images with light and dark mode variants', () => {
		const { container } = render(LandingPage);
		const images = Array.from(container.querySelectorAll('img')).map((img) =>
			img.getAttribute('src')
		);

		expect(images).toContain('/backtest-light.png');
		expect(images).toContain('/backtest-dark.png');
		expect(images).toContain('/dashboard-light.png');
		expect(images).toContain('/dashboard-dark.png');
		expect(images).toContain('/ai-suggestions-light.png');
		expect(images).toContain('/ai-suggestions-dark.png');
		expect(images).toContain('/ai-insights-light.png');
		expect(images).toContain('/ai-insights-dark.png');
	});

	it('renders hero chart widget with animated paths and pulse dots at end of chart data', () => {
		const { container } = render(LandingPage);
		const chartLine = container.querySelector('.chart-line');
		expect(chartLine).toBeInTheDocument();
		expect(chartLine).toHaveAttribute('pathLength', '1000');

		const chartArea = container.querySelector('.chart-area');
		expect(chartArea).toBeInTheDocument();

		const pulseDotGroup = container.querySelector('.chart-pulse-dot');
		expect(pulseDotGroup).toBeInTheDocument();
		expect(pulseDotGroup).toHaveAttribute('transform', 'translate(580, 42)');

		const pulseCircles = pulseDotGroup?.querySelectorAll('circle');
		expect(pulseCircles && pulseCircles.length).toBeGreaterThanOrEqual(4);
	});

	it('attaches scroll appear intersection observers to all landing page sections', () => {
		const { container } = render(LandingPage);

		expect(container.querySelector('#features-section')).toBeInTheDocument();
		expect(container.querySelector('#screenshot-section')).toBeInTheDocument();
		expect(container.querySelector('#how-section')).toBeInTheDocument();
		expect(container.querySelector('#cta-section')).toBeInTheDocument();
		expect(container.querySelector('#footer-section')).toBeInTheDocument();

		expect(mockObserve).toHaveBeenCalled();
		const observedElements = mockObserve.mock.calls.map((call) => (call[0] as HTMLElement)?.id);
		expect(observedElements).toContain('features-section');
		expect(observedElements).toContain('screenshot-section');
		expect(observedElements).toContain('how-section');
		expect(observedElements).toContain('cta-section');
		expect(observedElements).toContain('footer-section');
	});

	it('shows coming soon modal when user clicks register or login CTA if PUBLIC_IS_COMING_SOON is true', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';
		render(LandingPage);

		expect(screen.queryByText(/we're launching soon!/i)).not.toBeInTheDocument();

		const registerBtn = screen.getAllByRole('link', {
			name: /start backtesting|join trading lab|create free account/i
		})[0];
		await fireEvent.click(registerBtn);

		expect(screen.getByText(/we're launching soon!/i)).toBeInTheDocument();

		// Close modal
		const gotItBtn = screen.getByRole('button', { name: /got it/i });
		await fireEvent.click(gotItBtn);

		expect(screen.queryByText(/we're launching soon!/i)).not.toBeInTheDocument();

		// Test sign in CTA
		const signInBtn = screen.getAllByRole('link', { name: /sign in/i })[0];
		await fireEvent.click(signInBtn);
		expect(screen.getByText(/we're launching soon!/i)).toBeInTheDocument();
	});

	it('shows coming soon modal when user clicks footer auth links if PUBLIC_IS_COMING_SOON is true', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';
		render(LandingPage);

		const footerGetStarted = screen.getByRole('link', { name: /get started/i });
		await fireEvent.click(footerGetStarted);

		expect(screen.getByText(/we're launching soon!/i)).toBeInTheDocument();
	});

	it('does not show coming soon modal on click if PUBLIC_IS_COMING_SOON is false', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'false';
		render(LandingPage);

		const registerBtn = screen.getAllByRole('link', {
			name: /start backtesting|join trading lab|create free account/i
		})[0];
		await fireEvent.click(registerBtn);

		expect(screen.queryByText(/we're launching soon!/i)).not.toBeInTheDocument();
	});
});
