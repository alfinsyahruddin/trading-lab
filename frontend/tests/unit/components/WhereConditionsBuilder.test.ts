import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import WhereConditionsBuilder from '$lib/components/strategy/WhereConditionsBuilder.svelte';
import type { StrategyRuleGroup } from '$lib/types';

describe('WhereConditionsBuilder', () => {
	it('renders header and default group', () => {
		const groups: StrategyRuleGroup[] = [
			{
				id: 'g-1',
				connector_to_next: null,
				conditions: [
					{
						id: 'c-1',
						variable: 'price',
						operator: '>',
						value: '50',
						connector_to_next: null
					}
				]
			}
		];

		render(WhereConditionsBuilder, { props: { groups } });
		expect(screen.getByText('Where Conditions')).toBeInTheDocument();
		expect(screen.getByText(/GROUP 1/i)).toBeInTheDocument();
		expect(screen.getByText('Price')).toBeInTheDocument();
		expect(screen.getByText('price')).toBeInTheDocument();
	});

	it('adds a new condition when clicking Add condition button', async () => {
		const user = userEvent.setup();
		const groups: StrategyRuleGroup[] = [
			{
				id: 'g-1',
				connector_to_next: null,
				conditions: [
					{
						id: 'c-1',
						variable: 'price',
						operator: '>',
						value: '50',
						connector_to_next: null
					}
				]
			}
		];

		render(WhereConditionsBuilder, { props: { groups } });
		const addBtn = screen.getByRole('button', { name: /add condition/i });
		await user.click(addBtn);

		expect(groups[0].conditions).toHaveLength(2);
	});

	it('adds a new group when clicking Add Group button', async () => {
		const user = userEvent.setup();
		const groups: StrategyRuleGroup[] = [
			{
				id: 'g-1',
				connector_to_next: null,
				conditions: [
					{
						id: 'c-1',
						variable: 'price',
						operator: '>',
						value: '50',
						connector_to_next: null
					}
				]
			}
		];

		render(WhereConditionsBuilder, { props: { groups } });
		const addGroupBtn = screen.getByRole('button', { name: /add group/i });
		await user.click(addGroupBtn);

		expect(groups).toHaveLength(2);
	});

	it('updates operator when selecting from SelectField dropdown', async () => {
		const user = userEvent.setup();
		const groups: StrategyRuleGroup[] = [
			{
				id: 'g-1',
				connector_to_next: null,
				conditions: [
					{
						id: 'c-1',
						variable: 'price',
						operator: '>',
						value: '50',
						connector_to_next: null
					}
				]
			}
		];

		render(WhereConditionsBuilder, { props: { groups } });
		const comboboxes = screen.getAllByRole('combobox');
		expect(comboboxes.length).toBeGreaterThan(0);
		expect(comboboxes[0].style.backgroundColor).toBe('var(--bg-card)');

		// Click the operator combobox
		await user.click(comboboxes[0]);

		// Select the "<= less than or equals" option
		const option = screen.getByRole('option', { name: /less than or equals/i });
		await user.click(option);

		expect(groups[0].conditions[0].operator).toBe('<=');
	});
});
