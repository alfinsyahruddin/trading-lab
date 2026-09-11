import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import StatusBadge from '$lib/components/backtest/StatusBadge.svelte';

describe('StatusBadge', () => {
	it('renders PENDING status correctly', () => {
		const { container } = render(StatusBadge, { props: { status: 'PENDING' } });
		expect(screen.getByText('PENDING')).toBeInTheDocument();
		const badge = container.querySelector('span');
		expect(badge?.className).not.toContain('animate-pulse');
		expect(badge?.getAttribute('style')).toContain('var(--fg-muted)');
	});

	it('renders PROCESSING status with pulse animation and bounce indicator', () => {
		const { container } = render(StatusBadge, { props: { status: 'PROCESSING' } });
		expect(screen.getByText('PROCESSING')).toBeInTheDocument();
		const badge = container.querySelector('span');
		expect(badge?.className).toContain('animate-pulse');
		expect(badge?.getAttribute('style')).toContain('var(--warning)');
		const dot = badge?.querySelector('.animate-bounce');
		expect(dot).not.toBeNull();
	});

	it('renders DONE status correctly', () => {
		const { container } = render(StatusBadge, { props: { status: 'DONE' } });
		expect(screen.getByText('DONE')).toBeInTheDocument();
		const badge = container.querySelector('span');
		expect(badge?.className).not.toContain('animate-pulse');
		expect(badge?.getAttribute('style')).toContain('var(--success)');
	});

	it('renders FAILED status correctly', () => {
		const { container } = render(StatusBadge, { props: { status: 'FAILED' } });
		expect(screen.getByText('FAILED')).toBeInTheDocument();
		const badge = container.querySelector('span');
		expect(badge?.className).not.toContain('animate-pulse');
		expect(badge?.getAttribute('style')).toContain('var(--danger)');
	});
});
