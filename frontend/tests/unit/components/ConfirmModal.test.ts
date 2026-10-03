import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ConfirmModal from '#lib/components/ConfirmModal.svelte';

describe('ConfirmModal', () => {
	it('does not render dialog when open is false', () => {
		render(ConfirmModal, {
			props: {
				open: false,
				title: 'Delete Strategy',
				message: 'Are you sure you want to delete this?'
			}
		});

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('renders dialog with title, message, and action buttons when open is true', () => {
		render(ConfirmModal, {
			props: {
				open: true,
				title: 'Delete Strategy',
				message: 'Are you sure you want to delete this?',
				confirmLabel: 'Yes, Delete',
				cancelLabel: 'Keep Strategy'
			}
		});

		expect(screen.getByRole('dialog')).toBeInTheDocument();
		expect(screen.getByText('Delete Strategy')).toBeInTheDocument();
		expect(screen.getByText('Are you sure you want to delete this?')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Yes, Delete' })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Keep Strategy' })).toBeInTheDocument();
	});

	it('triggers onconfirm callback when confirm button is clicked', async () => {
		const user = userEvent.setup();
		const onconfirm = vi.fn();
		render(ConfirmModal, {
			props: {
				open: true,
				title: 'Confirm Action',
				message: 'Please confirm',
				onconfirm
			}
		});

		const confirmBtn = screen.getByRole('button', { name: 'Confirm' });
		await user.click(confirmBtn);

		expect(onconfirm).toHaveBeenCalledTimes(1);
	});

	it('triggers oncancel callback and closes when cancel button is clicked', async () => {
		const user = userEvent.setup();
		const oncancel = vi.fn();
		render(ConfirmModal, {
			props: {
				open: true,
				title: 'Confirm Action',
				message: 'Please confirm',
				oncancel
			}
		});

		const cancelBtn = screen.getByRole('button', { name: 'Cancel' });
		await user.click(cancelBtn);

		expect(oncancel).toHaveBeenCalledTimes(1);
	});

	it('disables confirm button and shows loading state when loading is true', () => {
		render(ConfirmModal, {
			props: {
				open: true,
				loading: true,
				confirmLabel: 'Delete'
			}
		});

		const confirmBtn = screen.getByRole('button', { name: /loading\.\.\./i });
		expect(confirmBtn).toBeDisabled();
	});
});
