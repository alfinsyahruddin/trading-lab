import { test, expect } from '@playwright/test';
import { authenticateUser, setupDefaultApiMocks, mockUser } from './helpers/mock-api';

test.describe('Dashboard Navigation & Profile Dropdown', () => {
	test.beforeEach(async ({ page }) => {
		await setupDefaultApiMocks(page);
		await authenticateUser(page);
	});

	test('navigates seamlessly across primary dashboard navigation tabs', async ({ page }) => {
		await page.goto('/dashboard');

		// 1. Overview / Dashboard Home
		await expect(page.getByRole('heading', { name: /community/i })).toBeVisible();

		// 2. Navigate to Trading Strategy
		const strategiesLink = page.getByRole('link', { name: /trading strategy/i }).first();
		await strategiesLink.click();
		await expect(page).toHaveURL(/\/dashboard\/strategies/);
		await expect(page.getByRole('heading', { name: 'Trading Strategy' })).toBeVisible();

		// 3. Navigate to Backtest
		const backtestLink = page.getByRole('link', { name: /backtest/i }).first();
		await backtestLink.click();
		await expect(page).toHaveURL(/\/dashboard\/backtests/);
		await expect(page.getByRole('heading', { name: 'Backtests' })).toBeVisible();

		// 4. Navigate to Settings
		const settingsLink = page.getByRole('link', { name: /settings/i }).first();
		await settingsLink.click();
		await expect(page).toHaveURL(/\/dashboard\/settings/);
		await expect(page.getByRole('heading', { name: /system settings/i })).toBeVisible();
	});

	test('opens user menu dropdown and triggers Edit Profile modal', async ({ page }) => {
		await page.goto('/dashboard');

		// Open user dropdown menu in TopBar
		const userMenuBtn = page.getByRole('button', { name: `User menu for ${mockUser.name}` });
		await userMenuBtn.click();

		// Verify menu content (email and role badge)
		await expect(page.getByText(mockUser.email)).toBeVisible();
		await expect(page.getByText('ADMIN').first()).toBeVisible();

		// Click Edit Profile option
		const editProfileBtn = page.getByRole('menuitem', { name: /edit profile/i });
		await editProfileBtn.click();

		// Verify Edit Profile modal opens
		const modalDialog = page.getByRole('dialog');
		await expect(modalDialog).toBeVisible();
		await expect(page.getByRole('heading', { name: 'Edit Profile' })).toBeVisible();

		// Close modal
		const cancelBtn = page.getByRole('button', { name: /cancel/i });
		await cancelBtn.click();
		await expect(modalDialog).not.toBeVisible();
	});

	test('opens user menu dropdown and triggers Change Password modal', async ({ page }) => {
		await page.goto('/dashboard');

		const userMenuBtn = page.getByRole('button', { name: `User menu for ${mockUser.name}` });
		await userMenuBtn.click();

		const changePasswordBtn = page.getByRole('menuitem', { name: /change password/i });
		await changePasswordBtn.click();

		const modalDialog = page.getByRole('dialog');
		await expect(modalDialog).toBeVisible();
		await expect(page.getByRole('heading', { name: 'Change Password' })).toBeVisible();

		const cancelBtn = page.getByRole('button', { name: /cancel/i });
		await cancelBtn.click();
		await expect(modalDialog).not.toBeVisible();
	});

	test('logs out successfully from user dropdown menu', async ({ page }) => {
		await page.goto('/dashboard');

		const userMenuBtn = page.getByRole('button', { name: `User menu for ${mockUser.name}` });
		await userMenuBtn.click();

		const logoutBtn = page.getByRole('menuitem', { name: /logout/i });
		await logoutBtn.click();

		// Should redirect to login page and clear tokens
		await expect(page).toHaveURL(/\/login/);
		const storedToken = await page.evaluate(() => localStorage.getItem('trading_lab_token'));
		expect(storedToken).toBeNull();
	});
});
