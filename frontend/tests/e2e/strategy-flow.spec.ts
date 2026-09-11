import { test, expect } from '@playwright/test';
import { authenticateUser, setupDefaultApiMocks, mockStrategy } from './helpers/mock-api';

test.describe('Strategy Flow', () => {
	test.beforeEach(async ({ page }) => {
		await setupDefaultApiMocks(page);
		await authenticateUser(page);
	});

	test('navigates to strategies list and displays existing strategy card', async ({ page }) => {
		await page.goto('/dashboard/strategies');

		await expect(page.getByRole('heading', { name: 'Trading Strategy' })).toBeVisible();
		await expect(page.getByText(mockStrategy.name)).toBeVisible();
		await expect(page.getByText(mockStrategy.description)).toBeVisible();
	});

	test('opens strategy creation form and navigates to /dashboard/strategies/new', async ({
		page
	}) => {
		await page.goto('/dashboard/strategies');

		const createStrategyBtn = page.getByRole('link', { name: /create strategy/i });
		await createStrategyBtn.click();

		await expect(page).toHaveURL(/\/dashboard\/strategies\/new/);
		await expect(page.getByRole('heading', { name: /create trading strategy/i })).toBeVisible();
	});

	test('interacts with WhereConditionsBuilder, connector buttons, and VariablePickerModal', async ({
		page
	}) => {
		await page.goto('/dashboard/strategies/new');

		// Verify initial condition group
		await expect(page.getByText(/group 1/i)).toBeVisible();

		// Add a second condition into Group 1
		const addConditionBtn = page.getByRole('button', { name: /add condition/i });
		await addConditionBtn.click();

		// Connector toggle (AND / OR) between the two conditions should now appear
		const andBtn = page.getByRole('button', { name: /^and$/i }).first();
		const orBtn = page.getByRole('button', { name: /^or$/i }).first();
		await expect(andBtn).toBeVisible();
		await expect(orBtn).toBeVisible();

		// Toggle connector to OR
		await orBtn.click();

		// Open VariablePickerModal for the first condition
		const variablePickerBtn = page.locator('button').filter({ hasText: 'price' }).first();
		await variablePickerBtn.click();

		// Verify Variable Picker Modal opens
		await expect(page.getByRole('dialog')).toBeVisible();
		await expect(page.getByRole('heading', { name: /select variable/i })).toBeVisible();

		// Search for variable
		const searchInput = page.getByPlaceholder(/search by code or description/i);
		await searchInput.fill('volume');

		// Select Volume card
		const volumeCard = page.getByRole('button').filter({ hasText: 'volume' }).first();
		await expect(volumeCard).toBeVisible();
		await volumeCard.click();

		// Confirm selection via Save button
		const saveBtn = page.getByRole('button', { name: /^save$/i });
		await expect(saveBtn).toBeEnabled();
		await saveBtn.click();

		// Modal should close
		await expect(page.getByRole('dialog')).not.toBeVisible();
	});

	test('fills strategy parameters and successfully creates strategy', async ({ page }) => {
		await page.goto('/dashboard/strategies/new');

		// Fill in form inputs
		await page.locator('#field-strategy-name').fill('Breakout Alpha Pro');
		await page.locator('#strat-desc').fill('High momentum strategy tested via Playwright');
		await page.locator('#tp-input').fill('15');
		await page.locator('#sl-input').fill('5');
		await page.locator('#holding-input').fill('45');

		// Verify dynamic risk reward display (15 / 5 = 1 : 3.0)
		await expect(page.getByText(/1 : 3/)).toBeVisible();

		// Submit form
		const submitBtn = page.getByRole('button', { name: /create strategy/i });
		await submitBtn.click();

		// Successfully redirects to strategies page
		await expect(page).toHaveURL(/\/dashboard\/strategies/);
	});
});
