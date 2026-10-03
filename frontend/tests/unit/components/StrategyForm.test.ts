import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import StrategyForm from '#lib/components/strategy/StrategyForm.svelte';

describe('StrategyForm', () => {
	it('submits valid strategy with default TP and SL values', async () => {
		const user = userEvent.setup();
		const onsubmit = vi.fn();
		const oncancel = vi.fn();

		render(StrategyForm, {
			props: {
				onsubmit,
				oncancel
			}
		});

		const nameInput = screen.getByLabelText(/strategy name/i);
		await user.type(nameInput, 'Breakout Alpha');

		const saveBtn = screen.getByRole('button', { name: /save strategy/i });
		await user.click(saveBtn);

		expect(onsubmit).toHaveBeenCalledTimes(1);
		expect(onsubmit).toHaveBeenCalledWith(
			expect.objectContaining({
				name: 'Breakout Alpha',
				tp_percentage: 10,
				sl_percentage: 5,
				max_holding_period_days: 30
			})
		);
	});

	it('intercepts submission with AI suggestions and applies rule suggestions upon accept', async () => {
		const user = userEvent.setup();
		const onsubmit = vi.fn();
		const oncancel = vi.fn();

		vi.spyOn(await import('#lib/api.js'), 'getStrategyAiSuggestions').mockResolvedValueOnce([
			{
				id: 'sug-rule-1',
				suggestion_type: 'RULE',
				rule_action: 'ADD_CONDITION',
				rule_payload: {
					group_index: 0,
					condition_index: null,
					variable: 'market_cap',
					operator: '>=',
					value: '1000000000000',
					connector_to_next: 'AND'
				},
				title: 'Filter Out Illiquid Micro-Caps',
				reason: 'Adding market_cap >= 1T IDR'
			}
		]);

		render(StrategyForm, {
			props: {
				enableAiSuggestions: true,
				onsubmit,
				oncancel
			}
		});

		const nameInput = screen.getByLabelText(/strategy name/i);
		await user.type(nameInput, 'Growth Alpha');

		const saveBtn = screen.getByRole('button', { name: /save strategy/i });
		await user.click(saveBtn);

		// First click should intercept and not call onsubmit
		expect(onsubmit).not.toHaveBeenCalled();

		// Suggestion card should appear
		expect(await screen.findByText('Filter Out Illiquid Micro-Caps')).toBeInTheDocument();

		// Accept the rule suggestion
		const acceptBtn = screen.getByRole('button', { name: /accept/i });
		await user.click(acceptBtn);

		// After accepting, suggestion card is cleared and user can submit
		await user.click(screen.getByRole('button', { name: /save strategy/i }));

		expect(onsubmit).toHaveBeenCalledTimes(1);
		expect(onsubmit).toHaveBeenCalledWith(
			expect.objectContaining({
				name: 'Growth Alpha',
				rules: expect.arrayContaining([
					expect.objectContaining({
						conditions: expect.arrayContaining([
							expect.objectContaining({
								variable: 'market_cap',
								operator: '>=',
								value: '1000000000000'
							})
						])
					})
				])
			})
		);
	});
});
