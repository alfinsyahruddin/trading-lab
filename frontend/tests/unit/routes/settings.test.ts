import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import SettingsPage from '../../../src/routes/dashboard/settings/+page.svelte';
import * as api from '$lib/api';
import * as session from '$lib/helpers/session';
import { toast } from '$lib/helpers/toast.svelte';
import type { UserResponse, AppSettings } from '$lib/types';

const mockGoto = vi.fn();
vi.mock('$app/navigation', () => ({
	goto: (...args: unknown[]) => mockGoto(...args)
}));

vi.mock('$lib/api', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/api')>();
	return {
		...actual,
		getSettings: vi.fn(),
		updateSettings: vi.fn()
	};
});

describe('SettingsPage', () => {
	const adminUser: UserResponse = {
		id: 'u-admin',
		name: 'Admin Boss',
		email: 'admin@tradinglab.id',
		role: 'ADMIN',
		created_at: '2026-01-01T00:00:00Z',
		updated_at: '2026-01-01T00:00:00Z'
	};

	const memberUser: UserResponse = {
		id: 'u-member',
		name: 'Trader Member',
		email: 'member@tradinglab.id',
		role: 'MEMBER',
		created_at: '2026-01-01T00:00:00Z',
		updated_at: '2026-01-01T00:00:00Z'
	};

	const defaultSettings: AppSettings = {
		ai_enabled: false
	};

	beforeEach(() => {
		vi.clearAllMocks();
		vi.spyOn(session, 'getToken').mockReturnValue('admin-token');
		vi.spyOn(session, 'getUser').mockReturnValue(adminUser);
		vi.mocked(api.getSettings).mockResolvedValue(defaultSettings);
	});

	it('redirects non-admin users to /dashboard and displays error toast', () => {
		vi.spyOn(session, 'getUser').mockReturnValue(memberUser);
		const toastErrorSpy = vi.spyOn(toast, 'error');

		render(SettingsPage);

		expect(toastErrorSpy).toHaveBeenCalledWith('Access restricted to Administrators only.');
		expect(mockGoto).toHaveBeenCalledWith('/dashboard');
		expect(api.getSettings).not.toHaveBeenCalled();
	});

	it('loads and renders settings with initial state', async () => {
		render(SettingsPage);

		expect(screen.getByText('System Settings')).toBeInTheDocument();
		expect(await screen.findByRole('switch', { name: 'AI Enabled' })).toBeInTheDocument();
		expect(screen.getByText('Disabled')).toBeInTheDocument();
		expect(screen.getByLabelText('LLM Model')).toBeDisabled();
		expect(screen.getByLabelText('Model Name')).toBeDisabled();
	});

	it('renders Active badge when ai_enabled is initially true', async () => {
		vi.mocked(api.getSettings).mockResolvedValue({
			ai_enabled: true
		});

		render(SettingsPage);

		expect(await screen.findByRole('switch', { name: 'AI Enabled' })).toBeInTheDocument();
		expect(screen.getByText('Active')).toBeInTheDocument();
	});

	it('toggles AI switch on and calls updateSettings API', async () => {
		vi.mocked(api.updateSettings).mockResolvedValue({
			ai_enabled: true
		});
		const toastSuccessSpy = vi.spyOn(toast, 'success');

		render(SettingsPage);

		const toggleButton = await screen.findByRole('switch', { name: 'AI Enabled' });
		expect(toggleButton).toHaveAttribute('aria-checked', 'false');

		await fireEvent.click(toggleButton);

		await waitFor(() => {
			expect(api.updateSettings).toHaveBeenCalledWith('admin-token', { ai_enabled: true });
			expect(toastSuccessSpy).toHaveBeenCalledWith('AI capabilities enabled.');
			expect(toggleButton).toHaveAttribute('aria-checked', 'true');
			expect(screen.getByText('Active')).toBeInTheDocument();
		});
	});

	it('reverts switch state if updateSettings API fails', async () => {
		vi.mocked(api.updateSettings).mockRejectedValue(new Error('Network error'));
		const toastErrorSpy = vi.spyOn(toast, 'error');

		render(SettingsPage);

		const toggleButton = await screen.findByRole('switch', { name: 'AI Enabled' });
		expect(toggleButton).toHaveAttribute('aria-checked', 'false');

		await fireEvent.click(toggleButton);

		await waitFor(() => {
			expect(api.updateSettings).toHaveBeenCalled();
			expect(toastErrorSpy).toHaveBeenCalledWith('Failed to update settings');
			expect(toggleButton).toHaveAttribute('aria-checked', 'false');
		});
	});
});
