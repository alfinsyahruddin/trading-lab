import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import EditProfileModal from '$lib/components/EditProfileModal.svelte';
import type { UserResponse } from '$lib/types';

const mockUser: UserResponse = {
	id: '123e4567-e89b-12d3-a456-426614174000',
	name: 'John Doe',
	email: 'john@example.com',
	role: 'MEMBER',
	created_at: '2026-08-30T00:00:00Z',
	updated_at: '2026-08-30T00:00:00Z'
};

describe('EditProfileModal', () => {
	it('does not render when open is false', () => {
		render(EditProfileModal, { props: { open: false, user: mockUser } });
		expect(screen.queryByText('Edit Profile')).not.toBeInTheDocument();
	});

	it('renders modal title and prefilled fields when open is true', () => {
		render(EditProfileModal, { props: { open: true, user: mockUser } });
		expect(screen.getByText('Edit Profile')).toBeInTheDocument();
		expect(screen.getByDisplayValue('John Doe')).toBeInTheDocument();
		expect(screen.getByDisplayValue('john@example.com')).toBeInTheDocument();
		expect(screen.getByText('Save Changes')).toBeInTheDocument();
	});
});
