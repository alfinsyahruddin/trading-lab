import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import ComingSoonModal from '$lib/components/ComingSoonModal.svelte';

describe('ComingSoonModal', () => {
	it('does not render dialog when open is false', () => {
		render(ComingSoonModal, {
			props: {
				open: false
			}
		});

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('renders dialog with launching soon message, open source notice, and github links when open is true', () => {
		render(ComingSoonModal, {
			props: {
				open: true
			}
		});

		const dialog = screen.getByRole('dialog');
		expect(dialog).toBeInTheDocument();
		expect(dialog).toHaveAttribute('aria-modal', 'true');
		expect(dialog).toHaveAttribute('aria-label', 'Coming Soon');

		expect(screen.getByRole('heading', { name: /we're launching soon!/i })).toBeInTheDocument();
		expect(screen.getByText(/trading lab is currently in private preview/i)).toBeInTheDocument();
		expect(screen.getByText(/stay tuned for public release!/i)).toBeInTheDocument();

		// Open source notice & GitHub repository
		expect(screen.getAllByText(/open source/i).length).toBeGreaterThanOrEqual(1);
		expect(
			screen.getByText(/this app is open source\. you can run it locally from:/i)
		).toBeInTheDocument();

		const repoLinks = screen.getAllByRole('link', {
			name: /github/i
		});
		expect(repoLinks.length).toBeGreaterThanOrEqual(1);

		const repoUrlLink = screen.getByRole('link', {
			name: /alfinsyahruddin\/trading-lab/i
		});
		expect(repoUrlLink).toHaveAttribute('href', 'https://github.com/alfinsyahruddin/trading-lab');
		expect(repoUrlLink).toHaveAttribute('target', '_blank');
		expect(repoUrlLink).toHaveAttribute('rel', 'noopener noreferrer');

		const viewOnGitHubBtn = screen.getByRole('link', {
			name: /view on github/i
		});
		expect(viewOnGitHubBtn).toHaveAttribute(
			'href',
			'https://github.com/alfinsyahruddin/trading-lab'
		);
		expect(viewOnGitHubBtn).toHaveAttribute('target', '_blank');
		expect(viewOnGitHubBtn).toHaveAttribute('rel', 'noopener noreferrer');
	});

	it('closes when Got it button is clicked', async () => {
		const user = userEvent.setup();
		render(ComingSoonModal, {
			props: {
				open: true
			}
		});

		const gotItBtn = screen.getByRole('button', { name: /got it/i });
		await user.click(gotItBtn);

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('closes when close button (x) is clicked', async () => {
		const user = userEvent.setup();
		render(ComingSoonModal, {
			props: {
				open: true
			}
		});

		const closeBtn = screen.getByRole('button', { name: /close modal/i });
		await user.click(closeBtn);

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('closes when Escape key is pressed', async () => {
		render(ComingSoonModal, {
			props: {
				open: true
			}
		});

		expect(screen.getByRole('dialog')).toBeInTheDocument();

		await fireEvent.keyDown(window, { key: 'Escape' });

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('closes when clicking backdrop outside modal', async () => {
		const { container } = render(ComingSoonModal, {
			props: {
				open: true
			}
		});

		const backdrop = container.firstElementChild as HTMLElement;
		expect(backdrop).not.toBeNull();

		await fireEvent.click(backdrop);

		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});
});
