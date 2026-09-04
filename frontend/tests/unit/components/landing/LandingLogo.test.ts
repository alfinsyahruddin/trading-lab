import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import LandingLogo from '../../../../src/lib/components/landing/LandingLogo.svelte';

describe('LandingLogo', () => {
	it('renders dark and light theme logos with default md size', () => {
		const { container } = render(LandingLogo);

		const logos = screen.getAllByAltText('Trading Lab');
		expect(logos).toHaveLength(2);

		const darkLogo = container.querySelector('.logo-dark-theme');
		const lightLogo = container.querySelector('.logo-light-theme');

		expect(darkLogo).toBeInTheDocument();
		expect(lightLogo).toBeInTheDocument();
		expect(darkLogo).toHaveAttribute('src', '/logo-dark.svg');
		expect(lightLogo).toHaveAttribute('src', '/logo-light.svg');
		expect(darkLogo).toHaveClass('h-7');
		expect(darkLogo).toHaveStyle('max-width: 150px');
	});

	it('renders sm size when size="sm" is passed', () => {
		const { container } = render(LandingLogo, { size: 'sm' });

		const darkLogo = container.querySelector('.logo-dark-theme');
		expect(darkLogo).toHaveClass('h-6');
		expect(darkLogo).toHaveStyle('max-width: 130px');
	});

	it('supports custom alt text', () => {
		render(LandingLogo, { alt: 'Custom Alt' });
		expect(screen.getAllByAltText('Custom Alt')).toHaveLength(2);
	});
});
