import { test, expect } from '@playwright/test';
import { jsonEnvelope, mockUser, setupDefaultApiMocks } from './helpers/mock-api';

test.describe('Landing & Authentication Flow', () => {
	test.beforeEach(async ({ page }) => {
		await setupDefaultApiMocks(page);
	});

	test('renders hero headline, branding, and navigation CTA buttons', async ({ page }) => {
		await page.goto('/');

		// Verify hero text and slogan
		await expect(page.locator('h1')).toContainText('Everyone built a');
		await expect(page.locator('h1')).toContainText('backtested it');

		// Verify CTA buttons on landing
		const joinBtn = page.getByRole('link', { name: /join trading lab/i }).first();
		const signInBtn = page.getByRole('link', { name: /sign in/i }).first();

		await expect(joinBtn).toBeVisible();
		await expect(signInBtn).toBeVisible();
	});

	test('navigates from landing to login and register pages', async ({ page }) => {
		await page.goto('/');

		const signInBtn = page.getByRole('link', { name: /sign in/i }).first();
		await signInBtn.click();
		await expect(page).toHaveURL(/\/login/);
		await expect(page.getByRole('heading', { name: /sign in/i })).toBeVisible();

		// Go back to landing and navigate to register
		await page.goto('/');
		const joinBtn = page.getByRole('link', { name: /join trading lab/i }).first();
		await joinBtn.click();
		await expect(page).toHaveURL(/\/register/);
		await expect(page.getByRole('heading', { name: /create account/i })).toBeVisible();
	});

	test('toggles theme between dark and light modes and persists preference', async ({ page }) => {
		await page.goto('/');

		const themeToggle = page.getByRole('button', { name: /toggle theme/i }).first();
		await expect(themeToggle).toBeVisible();

		// Check initial theme state on html element
		const initialIsDark = await page.evaluate(() =>
			document.documentElement.classList.contains('dark')
		);

		// Click toggle
		await themeToggle.click();

		// Verify html class flipped
		const flippedIsDark = await page.evaluate(() =>
			document.documentElement.classList.contains('dark')
		);
		expect(flippedIsDark).toBe(!initialIsDark);

		// Verify localStorage saved the preference
		const storedTheme = await page.evaluate(() => localStorage.getItem('trading_lab_theme'));
		expect(storedTheme).toBe(flippedIsDark ? 'dark' : 'light');
	});

	test('validates required fields on the login form', async ({ page }) => {
		await page.goto('/login');

		// Wait for form to appear and disable native browser validation so app-level validation rules trigger
		const form = page.locator('form');
		await form.waitFor();
		await form.evaluate((f: HTMLFormElement) => (f.noValidate = true));

		// Submit empty form
		const submitBtn = page.getByRole('button', { name: /sign in/i });
		await submitBtn.click();

		await expect(page.getByText('A valid email is required.')).toBeVisible();

		// Fill email only
		await page.locator('#field-email').fill('test@tradinglab.id');
		await submitBtn.click();

		await expect(page.getByText('Password is required.')).toBeVisible();
	});

	test('validates password constraints on register form', async ({ page }) => {
		await page.goto('/register');

		await page.locator('#field-full-name').fill('New Trader');
		await page.locator('#field-email').fill('newtrader@tradinglab.id');
		await page.locator('#field-password').fill('short');
		await page.locator('#field-confirm-password').fill('short');
		await page.getByPlaceholder(/enter code above/i).fill('TEST');

		const submitBtn = page.getByRole('button', { name: /create account/i });
		await submitBtn.click();

		await expect(page.getByText('Password must be at least 8 characters.')).toBeVisible();
	});

	test('authenticates user on successful login and redirects to /dashboard', async ({ page }) => {
		// Mock login POST endpoint
		await page.route('**/api/users/login', async (route) => {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(
					jsonEnvelope({
						tokens: {
							access_token: 'valid-jwt-token',
							refresh_token: 'valid-refresh-token'
						},
						user: mockUser
					})
				)
			});
		});

		await page.goto('/login');

		await page.locator('#field-email').fill('admin@tradinglab.id');
		await page.locator('#field-password').fill('password123');

		const submitBtn = page.getByRole('button', { name: /sign in/i });
		await submitBtn.click();

		await expect(page).toHaveURL(/\/dashboard/);
		await expect(page.getByText('Welcome back, Admin Tester!')).toBeVisible();
	});
});
