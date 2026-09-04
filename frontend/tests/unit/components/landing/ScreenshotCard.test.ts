import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ScreenshotCard from '../../../../src/lib/components/landing/ScreenshotCard.svelte';

describe('ScreenshotCard', () => {
	it('renders light and dark images with tag and description', () => {
		const { container } = render(ScreenshotCard, {
			lightSrc: '/backtest-light.png',
			darkSrc: '/backtest-dark.png',
			alt: 'Backtest screenshot',
			tag: 'Backtest Results',
			description: 'Full equity curve and trade logs.'
		});

		const images = container.querySelectorAll('img');
		expect(images).toHaveLength(2);
		expect(images[0]).toHaveAttribute('src', '/backtest-light.png');
		expect(images[1]).toHaveAttribute('src', '/backtest-dark.png');

		expect(screen.getByText('Backtest Results')).toBeInTheDocument();
		expect(screen.getByText('Full equity curve and trade logs.')).toBeInTheDocument();
	});
});
