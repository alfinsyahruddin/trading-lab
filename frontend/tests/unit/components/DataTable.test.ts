import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import { createRawSnippet } from 'svelte';
import DataTable from '$lib/components/DataTable.svelte';

interface Item {
	id: string;
	name: string;
	score: number | null;
	role?: string;
}

describe('DataTable', () => {
	const columns = [
		{ key: 'name', label: 'Item Name' },
		{
			key: 'score',
			label: 'Score',
			render: (row: Item) => (row.score !== null ? `${row.score} pts` : 'No score')
		},
		{ key: 'role', label: 'Role' }
	];

	const rows: Item[] = [
		{ id: '1', name: 'Alpha', score: 95, role: 'Leader' },
		{ id: '2', name: 'Beta', score: null, role: undefined }
	];

	it('renders table headers and row cell contents accurately', () => {
		render(DataTable<Item>, {
			props: {
				columns,
				rows
			}
		});

		expect(screen.getByText('Item Name')).toBeInTheDocument();
		expect(screen.getByText('Score')).toBeInTheDocument();
		expect(screen.getByText('Role')).toBeInTheDocument();

		expect(screen.getByText('Alpha')).toBeInTheDocument();
		expect(screen.getByText('95 pts')).toBeInTheDocument();
		expect(screen.getByText('Leader')).toBeInTheDocument();

		expect(screen.getByText('Beta')).toBeInTheDocument();
		expect(screen.getByText('No score')).toBeInTheDocument();
		expect(screen.getByText('—')).toBeInTheDocument(); // undefined role rendered as '—'
	});

	it('displays skeleton pulse loaders when loading is true', () => {
		const { container } = render(DataTable<Item>, {
			props: {
				columns,
				rows,
				loading: true
			}
		});

		const pulseElements = container.querySelectorAll('.animate-pulse');
		expect(pulseElements.length).toBeGreaterThan(0);
		expect(screen.queryByText('Alpha')).not.toBeInTheDocument();
	});

	it('displays empty message when rows array is empty', () => {
		render(DataTable<Item>, {
			props: {
				columns,
				rows: [],
				emptyMessage: 'No strategies found.'
			}
		});

		expect(screen.getByText('No strategies found.')).toBeInTheDocument();
	});

	it('renders action column and custom actions snippet', () => {
		const actionsSnippet = createRawSnippet<[Item]>((rowFn) => {
			const row = rowFn();
			return {
				render: () => `<button data-testid="action-${row.id}">Edit ${row.name}</button>`
			};
		});

		render(DataTable<Item>, {
			props: {
				columns,
				rows,
				actions: actionsSnippet
			}
		});

		expect(screen.getByText('Actions')).toBeInTheDocument();
		expect(screen.getByTestId('action-1')).toHaveTextContent('Edit Alpha');
		expect(screen.getByTestId('action-2')).toHaveTextContent('Edit Beta');
	});

	it('applies hover and blur styles on table rows', async () => {
		const { container } = render(DataTable<Item>, {
			props: {
				columns,
				rows
			}
		});

		const firstRow = container.querySelector('tbody tr') as HTMLTableRowElement;
		expect(firstRow).not.toBeNull();

		await fireEvent.mouseOver(firstRow);
		expect(firstRow.style.backgroundColor).toBe('var(--bg-card-hover)');

		await fireEvent.mouseOut(firstRow);
		expect(firstRow.style.backgroundColor).toBe('transparent');

		await fireEvent.focus(firstRow);
		expect(firstRow.style.backgroundColor).toBe('var(--bg-card-hover)');

		await fireEvent.blur(firstRow);
		expect(firstRow.style.backgroundColor).toBe('transparent');
	});
});
