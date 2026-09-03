import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import RegisterPage from '../../../src/routes/register/+page.svelte';
import * as api from '$lib/api';
import { goto } from '$app/navigation';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		register: vi.fn()
	};
});

describe('RegisterPage', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		import.meta.env.PUBLIC_IS_COMING_SOON = 'false';
	});

	it('renders registration form fields', () => {
		render(RegisterPage);
		expect(screen.getByLabelText(/^Full Name/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/^Email/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/^Password/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/^Confirm Password/i)).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /create account/i })).toBeInTheDocument();
	});

	it('validates form inputs before submission', async () => {
		const { container } = render(RegisterPage);
		const form = container.querySelector('form')!;
		await fireEvent.submit(form);

		expect(api.register).not.toHaveBeenCalled();
		expect(screen.getByText(/name is required/i)).toBeInTheDocument();
	});

	it('submits registration when inputs are valid and PUBLIC_IS_COMING_SOON is false', async () => {
		vi.mocked(api.register).mockResolvedValueOnce({
			id: 'u-1',
			name: 'Trader Joe',
			email: 'joe@example.com',
			role: 'MEMBER',
			created_at: '2026-08-30T00:00:00Z',
			updated_at: '2026-08-30T00:00:00Z'
		});

		render(RegisterPage);
		await fireEvent.input(screen.getByLabelText(/^Full Name/i), {
			target: { value: 'Trader Joe' }
		});
		await fireEvent.input(screen.getByLabelText(/^Email/i), {
			target: { value: 'joe@example.com' }
		});
		await fireEvent.input(screen.getByLabelText(/^Password/i), {
			target: { value: 'securepassword123' }
		});
		await fireEvent.input(screen.getByLabelText(/^Confirm Password/i), {
			target: { value: 'securepassword123' }
		});

		await fireEvent.click(screen.getByRole('button', { name: /create account/i }));

		expect(api.register).toHaveBeenCalledWith('Trader Joe', 'joe@example.com', 'securepassword123');
		expect(goto).toHaveBeenCalledWith('/login');
	});

	it('shows coming soon modal and blocks API call when PUBLIC_IS_COMING_SOON is true', async () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';

		render(RegisterPage);
		await fireEvent.input(screen.getByLabelText(/^Full Name/i), {
			target: { value: 'Trader Joe' }
		});
		await fireEvent.input(screen.getByLabelText(/^Email/i), {
			target: { value: 'joe@example.com' }
		});
		await fireEvent.input(screen.getByLabelText(/^Password/i), {
			target: { value: 'securepassword123' }
		});
		await fireEvent.input(screen.getByLabelText(/^Confirm Password/i), {
			target: { value: 'securepassword123' }
		});

		await fireEvent.click(screen.getByRole('button', { name: /create account/i }));

		expect(api.register).not.toHaveBeenCalled();
		expect(screen.getByRole('dialog')).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /we're launching soon!/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /close modal/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /got it/i })).toBeInTheDocument();
		expect(screen.getByText(/stay tuned for public release!/i)).toBeInTheDocument();
	});
});
