import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SectionHeader from '../../../../src/lib/components/landing/SectionHeader.svelte';

describe('SectionHeader', () => {
	it('renders tag, title and description', () => {
		render(SectionHeader, {
			tag: 'The Platform',
			title: 'Your strategy deserves evidence.',
			description: 'Stop guessing and test.'
		});

		expect(screen.getByText('The Platform')).toBeInTheDocument();
		expect(
			screen.getByRole('heading', { level: 2, name: /your strategy deserves evidence/i })
		).toBeInTheDocument();
		expect(screen.getByText('Stop guessing and test.')).toBeInTheDocument();
	});

	it('renders without description if omitted', () => {
		render(SectionHeader, {
			tag: 'The Workflow',
			title: 'From hypothesis to evidence'
		});

		expect(screen.getByText('The Workflow')).toBeInTheDocument();
		expect(
			screen.getByRole('heading', { level: 2, name: /from hypothesis to evidence/i })
		).toBeInTheDocument();
	});
});
