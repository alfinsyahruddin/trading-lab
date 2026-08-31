import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import StrategyForm from '$lib/components/strategy/StrategyForm.svelte';

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
});
