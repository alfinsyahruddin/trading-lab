import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import FeatureCard from '../../../../src/lib/components/landing/FeatureCard.svelte';

describe('FeatureCard', () => {
	it('renders standard card with title, description, and bullets', () => {
		render(FeatureCard, {
			title: 'Visual Strategy Builder',
			description: 'Compose multi-group screening rules.',
			bullets: ['P/E, P/B, ROE', 'Price, volume signals']
		});

		expect(
			screen.getByRole('heading', { level: 3, name: 'Visual Strategy Builder' })
		).toBeInTheDocument();
		expect(screen.getByText('Compose multi-group screening rules.')).toBeInTheDocument();
		expect(screen.getByText('P/E, P/B, ROE')).toBeInTheDocument();
		expect(screen.getByText('Price, volume signals')).toBeInTheDocument();
	});

	it('renders AI gradient variant when isAi is true', () => {
		const { container } = render(FeatureCard, {
			title: 'AI-Powered',
			description: 'AI reviews your strategies.',
			bullets: ['Risk-reward hints'],
			isAi: true
		});

		expect(screen.getByRole('heading', { level: 3, name: 'AI-Powered' })).toBeInTheDocument();
		const heading = screen.getByRole('heading', { level: 3, name: 'AI-Powered' });
		expect(heading).toHaveClass('bg-clip-text');

		const article = container.querySelector('article');
		expect(article).toHaveClass('p-px');
	});
});
