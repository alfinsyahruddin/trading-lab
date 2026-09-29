import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ScreenshotCard from '../../../../src/lib/components/landing/ScreenshotCard.svelte';

describe('ScreenshotCard', () => {
	it('renders light and dark images with tag and description', () => {
		const { container } = render(ScreenshotCard, {
			lightSrc: '/backtest-light.webp',
			darkSrc: '/backtest-dark.webp',
			alt: 'Backtest screenshot',
			tag: 'Backtest Results',
			description: 'Full equity curve and trade logs.',
			width: 2314,
			height: 1792
		});

		const images = container.querySelectorAll('img');
		expect(images).toHaveLength(2);
		expect(images[0]).toHaveAttribute('src', '/backtest-light.webp');
		expect(images[0]).toHaveAttribute('width', '2314');
		expect(images[0]).toHaveAttribute('height', '1792');
		expect(images[0]).toHaveAttribute('decoding', 'async');
		expect(images[1]).toHaveAttribute('src', '/backtest-dark.webp');
		expect(images[1]).toHaveAttribute('width', '2314');
		expect(images[1]).toHaveAttribute('height', '1792');
		expect(images[1]).toHaveAttribute('decoding', 'async');

		expect(screen.getByText('Backtest Results')).toBeInTheDocument();
		expect(screen.getByText('Full equity curve and trade logs.')).toBeInTheDocument();
	});
});
