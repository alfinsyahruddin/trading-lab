import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import StrategyAiSuggestionsCard from '$lib/components/strategy/StrategyAiSuggestionsCard.svelte';
import type { StrategyAiSuggestion } from '$lib/types';

describe('StrategyAiSuggestionsCard', () => {
	const mockSuggestions: StrategyAiSuggestion[] = [
		{
			id: 'sug-1',
			field: 'sl_percentage',
			title: 'Optimize Risk-Reward Ratio',
			current_value: 5.0,
			suggested_value: 3.5,
			reason: 'Lowering stop loss from 5% to 3.5% increases your R:R to 2.86:1.'
		},
		{
			id: 'sug-2',
			field: 'max_holding_period_days',
			title: 'Shorten Holding Window',
			current_value: 30,
			suggested_value: 15,
			reason: 'IDX momentum setups achieve optimal profit capture within 15 days.'
		}
	];

	it('renders suggestions with remaining count badge and details', () => {
		render(StrategyAiSuggestionsCard, {
			props: {
				suggestions: mockSuggestions,
				onaccept: vi.fn(),
				onignore: vi.fn()
			}
		});

		expect(screen.getByText('AI Strategy Suggestions')).toBeInTheDocument();
		expect(screen.getByText(/2 remaining/i)).toBeInTheDocument();
		expect(screen.getByText('Optimize Risk-Reward Ratio')).toBeInTheDocument();
		expect(screen.getByText('Shorten Holding Window')).toBeInTheDocument();
	});

	it('triggers onaccept callback when Accept button is clicked', async () => {
		const onaccept = vi.fn();
		const onignore = vi.fn();
		render(StrategyAiSuggestionsCard, {
			props: {
				suggestions: mockSuggestions,
				onaccept,
				onignore
			}
		});

		const acceptButtons = screen.getAllByRole('button', { name: /accept/i });
		await fireEvent.click(acceptButtons[0]);

		expect(onaccept).toHaveBeenCalledWith(mockSuggestions[0]);
		expect(onignore).not.toHaveBeenCalled();
	});

	it('triggers onignore callback when Ignore button is clicked', async () => {
		const onaccept = vi.fn();
		const onignore = vi.fn();
		render(StrategyAiSuggestionsCard, {
			props: {
				suggestions: mockSuggestions,
				onaccept,
				onignore
			}
		});

		const ignoreButtons = screen.getAllByRole('button', { name: /ignore/i });
		await fireEvent.click(ignoreButtons[1]);

		expect(onignore).toHaveBeenCalledWith(mockSuggestions[1]);
		expect(onaccept).not.toHaveBeenCalled();
	});
});
