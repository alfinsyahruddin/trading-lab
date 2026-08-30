import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import RoleBadge from '$lib/components/RoleBadge.svelte';

describe('RoleBadge', () => {
	it('renders ADMIN role', () => {
		render(RoleBadge, { props: { role: 'ADMIN' } });
		expect(screen.getByText('ADMIN')).toBeInTheDocument();
	});

	it('renders MEMBER role', () => {
		render(RoleBadge, { props: { role: 'MEMBER' } });
		expect(screen.getByText('MEMBER')).toBeInTheDocument();
	});

	it('has accent color for ADMIN', () => {
		render(RoleBadge, { props: { role: 'ADMIN' } });
		const badge = screen.getByText('ADMIN');
		const style = badge.getAttribute('style') || '';
		expect(style).toContain('var(--accent)');
	});

	it('has muted color for MEMBER', () => {
		render(RoleBadge, { props: { role: 'MEMBER' } });
		const badge = screen.getByText('MEMBER');
		const style = badge.getAttribute('style') || '';
		expect(style).toContain('var(--fg-muted)');
	});

	it('supports compact sm size', () => {
		render(RoleBadge, { props: { role: 'ADMIN', size: 'sm' } });
		const badge = screen.getByText('ADMIN');
		expect(badge.className).toContain('text-[10px]');
	});
});
