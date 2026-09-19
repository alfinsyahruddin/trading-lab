import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import HeroSection from '../../../../src/lib/components/landing/HeroSection.svelte';

describe('HeroSection', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('renders headline, stat labels, and CTAs', () => {
		render(HeroSection, { isLoggedIn: false });

		expect(screen.getByText(/Everyone built a/i)).toBeInTheDocument();
		expect(screen.getByText(/IDX Indicators/i)).toBeInTheDocument();
		expect(screen.getByText(/Historical Data/i)).toBeInTheDocument();
		expect(screen.getByText(/Performance Metrics/i)).toBeInTheDocument();
	});

	it('animates stats numbers from 0 to target with ease-out', async () => {
		render(HeroSection, { isLoggedIn: false, statsDuration: 1000 });

		// Initially at 0
		expect(screen.getAllByText('0+')).toHaveLength(2);
		expect(screen.getByText('0 Years')).toBeInTheDocument();

		// Advance animation past duration
		await vi.advanceTimersByTimeAsync(1200);

		expect(screen.getByText('100+')).toBeInTheDocument();
		expect(screen.getByText('5 Years')).toBeInTheDocument();
		expect(screen.getByText('8+')).toBeInTheDocument();
	});

	it('displays target numbers immediately if prefers-reduced-motion is true', () => {
		const originalMatchMedia = window.matchMedia;
		window.matchMedia = vi.fn().mockImplementation((query: string) => ({
			matches: query.includes('prefers-reduced-motion'),
			media: query,
			onchange: null,
			addListener: vi.fn(),
			removeListener: vi.fn(),
			addEventListener: vi.fn(),
			removeEventListener: vi.fn(),
			dispatchEvent: vi.fn()
		}));

		render(HeroSection, { isLoggedIn: false });

		expect(screen.getByText('100+')).toBeInTheDocument();
		expect(screen.getByText('5 Years')).toBeInTheDocument();
		expect(screen.getByText('8+')).toBeInTheDocument();

		window.matchMedia = originalMatchMedia;
	});
});
