import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import EmptyState from '$lib/components/EmptyState.svelte';

describe('EmptyState', () => {
	it('renders the default message', () => {
		render(EmptyState, { props: { message: 'No items found.' } });
		expect(screen.getByText('No items found.')).toBeInTheDocument();
	});

	it('renders a custom message', () => {
		render(EmptyState, { props: { message: 'Nothing here yet.' } });
		expect(screen.getByText('Nothing here yet.')).toBeInTheDocument();
	});

	it('renders a default SVG icon when no icon slot provided', () => {
		const { container } = render(EmptyState, { props: { message: 'Empty' } });
		const svg = container.querySelector('svg');
		expect(svg).toBeInTheDocument();
	});
});
