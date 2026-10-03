import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { createRawSnippet } from 'svelte';
import Modal from '#lib/components/Modal.svelte';

describe('Modal', () => {
	const dummyChildren = createRawSnippet(() => ({
		render: () => '<p>Modal body test content</p>'
	}));

	const dummyFooter = createRawSnippet(() => ({
		render: () => '<button type="button">Custom Footer Button</button>'
	}));

	it('does not render dialog when open is false', () => {
		render(Modal, {
			props: {
				open: false,
				title: 'Test Modal',
				children: dummyChildren
			}
		});

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('renders dialog, title, body children, and footer when open is true', () => {
		render(Modal, {
			props: {
				open: true,
				title: 'Test Modal',
				children: dummyChildren,
				footer: dummyFooter
			}
		});

		const dialog = screen.getByRole('dialog');
		expect(dialog).toBeInTheDocument();
		expect(dialog).toHaveAttribute('aria-modal', 'true');
		expect(dialog).toHaveAttribute('aria-labelledby', 'modal-title');
		expect(screen.getByText('Test Modal')).toBeInTheDocument();
		expect(screen.getByText('Modal body test content')).toBeInTheDocument();
		expect(screen.getByText('Custom Footer Button')).toBeInTheDocument();
	});

	it('closes when close button (x) is clicked', async () => {
		const user = userEvent.setup();
		render(Modal, {
			props: {
				open: true,
				title: 'Closable Modal',
				children: dummyChildren
			}
		});

		const closeBtn = screen.getByRole('button', { name: /close modal/i });
		await user.click(closeBtn);

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('closes when Escape key is pressed', async () => {
		render(Modal, {
			props: {
				open: true,
				title: 'Escape Modal',
				children: dummyChildren
			}
		});

		expect(screen.getByRole('dialog')).toBeInTheDocument();

		await fireEvent.keyDown(window, { key: 'Escape' });

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('closes when backdrop outside dialog is clicked', async () => {
		const { container } = render(Modal, {
			props: {
				open: true,
				title: 'Backdrop Modal',
				children: dummyChildren
			}
		});

		const backdrop = container.firstElementChild as HTMLElement;
		expect(backdrop).not.toBeNull();

		await fireEvent.click(backdrop);

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});
});
