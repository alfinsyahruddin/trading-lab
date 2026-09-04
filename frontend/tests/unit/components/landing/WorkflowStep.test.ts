import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import WorkflowStep from '../../../../src/lib/components/landing/WorkflowStep.svelte';

describe('WorkflowStep', () => {
	it('renders step number, title, and description', () => {
		render(WorkflowStep, {
			step: '01',
			title: 'Build your strategy',
			description: 'Use the visual rule builder.'
		});

		expect(screen.getByText('01')).toBeInTheDocument();
		expect(
			screen.getByRole('heading', { level: 3, name: 'Build your strategy' })
		).toBeInTheDocument();
		expect(screen.getByText('Use the visual rule builder.')).toBeInTheDocument();
	});
});
