import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import IosSwitch from '#lib/components/IosSwitch.svelte';

describe('IosSwitch', () => {
	it('renders switch with role="switch" and unchecked state', () => {
		render(IosSwitch, { props: { checked: false, label: 'AI Enabled' } });
		const sw = screen.getByRole('switch', { name: 'AI Enabled' });
		expect(sw).toBeInTheDocument();
		expect(sw).toHaveAttribute('aria-checked', 'false');
	});

	it('renders checked state', () => {
		render(IosSwitch, { props: { checked: true, label: 'AI Enabled' } });
		const sw = screen.getByRole('switch', { name: 'AI Enabled' });
		expect(sw).toHaveAttribute('aria-checked', 'true');
	});

	it('triggers onchange when clicked', async () => {
		const onchange = vi.fn();
		render(IosSwitch, { props: { checked: false, label: 'AI Enabled', onchange } });
		const sw = screen.getByRole('switch', { name: 'AI Enabled' });
		await fireEvent.click(sw);
		expect(onchange).toHaveBeenCalledWith(true);
	});

	it('toggles with Space or Enter key', async () => {
		const onchange = vi.fn();
		render(IosSwitch, { props: { checked: false, label: 'AI Enabled', onchange } });
		const sw = screen.getByRole('switch', { name: 'AI Enabled' });
		await fireEvent.keyDown(sw, { key: ' ' });
		expect(onchange).toHaveBeenCalledWith(true);
	});

	it('does not toggle when disabled', async () => {
		const onchange = vi.fn();
		render(IosSwitch, { props: { checked: false, disabled: true, label: 'AI Enabled', onchange } });
		const sw = screen.getByRole('switch', { name: 'AI Enabled' });
		expect(sw).toBeDisabled();
		await fireEvent.click(sw);
		expect(onchange).not.toHaveBeenCalled();
	});
});
