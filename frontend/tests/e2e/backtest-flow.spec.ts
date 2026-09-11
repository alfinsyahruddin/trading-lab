import { test, expect } from '@playwright/test';
import {
	authenticateUser,
	setupDefaultApiMocks,
	mockBacktest,
	mockStrategy,
	jsonEnvelope
} from './helpers/mock-api';

interface CapturedBacktestPayload {
	name?: string;
	initial_cash?: number;
	year?: number;
	is_public?: boolean;
	backtest_duration_months?: number;
}

test.describe('Backtest Flow', () => {
	test.beforeEach(async ({ page }) => {
		await setupDefaultApiMocks(page);
		await authenticateUser(page);
	});

	test('navigates to backtests list and displays backtest cards', async ({ page }) => {
		await page.goto('/dashboard/backtests');

		await expect(page.getByRole('heading', { name: 'Backtests' })).toBeVisible();
		await expect(page.getByText(mockBacktest.name)).toBeVisible();
	});

	test('navigates to new backtest form and populates strategy default name', async ({ page }) => {
		await page.goto('/dashboard/backtests/new');

		await expect(page.getByRole('heading', { name: /run backtest/i })).toBeVisible();

		// Strategy should be selected and name auto-populated with year
		const nameInput = page.locator('#field-backtest-name');
		await expect(nameInput).toBeVisible();
		await expect(nameInput).toHaveValue(new RegExp(`${mockStrategy.name}`));
	});

	test('validates cash input constraints on new backtest form', async ({ page }) => {
		await page.goto('/dashboard/backtests/new');

		const form = page.locator('form');
		await form.waitFor();
		await form.evaluate((f: HTMLFormElement) => (f.noValidate = true));

		const cashInput = page.locator('#field-initial-cash');
		// Set invalid cash below minimum (Rp 1.000.000)
		await cashInput.fill('100');

		const runBtn = page.getByRole('button', { name: /run backtest/i });
		await runBtn.click();

		await expect(
			page.getByText(/initial cash must be between rp 1\.000\.000 and rp 100\.000\.000\.000/i)
		).toBeVisible();
	});

	test('configures parameters and successfully submits backtest simulation', async ({ page }) => {
		let capturedPayload: CapturedBacktestPayload | null = null;

		await page.route('**/api/backtests', async (route) => {
			const req = route.request();
			if (req.method() === 'POST') {
				capturedPayload = JSON.parse(req.postData() || '{}');
				return route.fulfill({
					status: 200,
					contentType: 'application/json',
					body: JSON.stringify(
						jsonEnvelope({
							...mockBacktest,
							id: 'bt-new-simulation',
							name: capturedPayload?.name || 'Simulation 2024',
							status: 'PENDING'
						})
					)
				});
			}
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope([mockBacktest]))
			});
		});

		await page.goto('/dashboard/backtests/new');

		// Fill in parameters
		await page.locator('#field-backtest-name').fill('Custom Momentum Test 2024');
		await page.locator('#field-initial-cash').fill('50000000');
		await page.locator('#field-max-holding-stocks').fill('5');
		await page.locator('#field-max-stocks').fill('15');

		// Select Year 2024 via SegmentedControl
		const yearBtn = page.getByRole('button', { name: '2024' });
		await yearBtn.click();

		// Toggle Visibility to Public
		const publicBtn = page.getByRole('button', { name: 'Public' });
		await publicBtn.click();

		// Submit form
		const runBtn = page.getByRole('button', { name: /run backtest/i });
		await runBtn.click();

		// Should navigate to the backtests list view
		await expect(page).toHaveURL(/\/dashboard\/backtests$/);

		// Payload verified
		const payload = capturedPayload as unknown as CapturedBacktestPayload;
		expect(payload).not.toBeNull();
		expect(payload.name).toBe('Custom Momentum Test 2024');
		expect(payload.initial_cash).toBe(50000000);
		expect(payload.year).toBe(2024);
		expect(payload.is_public).toBe(true);
	});
});
