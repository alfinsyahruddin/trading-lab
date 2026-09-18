import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import VariablePickerModal from '$lib/components/strategy/VariablePickerModal.svelte';

describe('VariablePickerModal', () => {
	it('does not render dialog content when open is false', () => {
		const { container } = render(VariablePickerModal, { props: { open: false } });
		expect(container.querySelector('[role="dialog"]')).toBeNull();
	});

	it('renders dialog with variable categories when open is true', () => {
		render(VariablePickerModal, {
			props: { open: true, selectedVariable: 'price' }
		});
		expect(screen.getByRole('dialog')).toBeInTheDocument();
		expect(screen.getByText('Select Variable')).toBeInTheDocument();
		expect(screen.getByText('Price & Market')).toBeInTheDocument();
	});

	it('filters variables when searching', async () => {
		const user = userEvent.setup();
		render(VariablePickerModal, {
			props: { open: true, selectedVariable: 'price' }
		});

		const input = screen.getByPlaceholderText(/Search by code or description/i);
		await user.type(input, 'operating_pnl');

		expect(screen.getAllByText('operating_pnl').length).toBeGreaterThanOrEqual(1);
		expect(screen.getByText(/Operating profit\/loss/i)).toBeInTheDocument();
		expect(screen.queryByText('Price & Market')).toBeNull();
	});

	it('disables Save button when no selection change is made', () => {
		render(VariablePickerModal, {
			props: { open: true, selectedVariable: 'price' }
		});

		const saveBtn = screen.getByRole('button', { name: /save/i });
		expect(saveBtn).toBeDisabled();
	});

	it('enables Save button when a different variable is selected', async () => {
		const user = userEvent.setup();
		const onselect = vi.fn();
		render(VariablePickerModal, {
			props: { open: true, selectedVariable: 'price', onselect }
		});

		const volumeBtn = screen.getByRole('button', { name: /volume/i });
		await user.click(volumeBtn);

		const saveBtn = screen.getByRole('button', { name: /save/i });
		expect(saveBtn).not.toBeDisabled();

		await user.click(saveBtn);
		expect(onselect).toHaveBeenCalledWith('volume');
	});

	it('allows selecting market_cap and passes market_cap to onselect', async () => {
		const user = userEvent.setup();
		const onselect = vi.fn();
		render(VariablePickerModal, {
			props: { open: true, selectedVariable: 'price', onselect }
		});

		const marketCapBtn = screen.getByRole('button', { name: /market cap/i });
		expect(marketCapBtn).toBeInTheDocument();
		await user.click(marketCapBtn);

		const saveBtn = screen.getByRole('button', { name: /save/i });
		await user.click(saveBtn);
		expect(onselect).toHaveBeenCalledWith('market_cap');
	});
});
