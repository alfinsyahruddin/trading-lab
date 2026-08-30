import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ChangePasswordModal from '$lib/components/ChangePasswordModal.svelte';

describe('ChangePasswordModal', () => {
	it('does not render when open is false', () => {
		render(ChangePasswordModal, { props: { open: false } });
		expect(screen.queryByRole('heading', { name: 'Change Password' })).not.toBeInTheDocument();
	});

	it('renders modal title and password fields when open is true', () => {
		render(ChangePasswordModal, { props: { open: true } });
		expect(screen.getByRole('heading', { name: 'Change Password' })).toBeInTheDocument();
		expect(screen.getByLabelText(/Current Password/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/^New Password/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/Confirm New Password/i)).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Change Password' })).toBeInTheDocument();
	});
});
