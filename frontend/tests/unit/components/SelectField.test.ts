import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import SelectField from '#lib/components/SelectField.svelte';

describe('SelectField', () => {
	const sampleOptions = [
		{ label: 'Option 1', value: 'opt1' },
		{ label: 'Option 2', value: 'opt2' }
	];

	it('renders without label by default', () => {
		const { container } = render(SelectField, { props: { options: sampleOptions } });
		const label = container.querySelector('label');
		expect(label).toBeNull();
	});

	it('renders a label when provided', () => {
		render(SelectField, { props: { label: 'Choose Strategy', options: sampleOptions } });
		expect(screen.getByText('Choose Strategy')).toBeInTheDocument();
	});

	it('opens dropdown and displays searchable options when clicked', async () => {
		render(SelectField, { props: { options: sampleOptions } });
		const trigger = screen.getByRole('combobox');
		expect(trigger).toBeInTheDocument();

		await fireEvent.click(trigger);
		expect(screen.getByPlaceholderText('Search...')).toBeInTheDocument();
		expect(screen.getByText('Option 1')).toBeInTheDocument();
		expect(screen.getByText('Option 2')).toBeInTheDocument();

		// Search filtering
		const searchInput = screen.getByPlaceholderText('Search...');
		await fireEvent.input(searchInput, { target: { value: 'Option 2' } });
		expect(screen.queryByText('Option 1')).not.toBeInTheDocument();
		expect(screen.getByText('Option 2')).toBeInTheDocument();
	});

	it('shows error message when error prop is provided', () => {
		render(SelectField, { props: { error: 'Selection required', options: sampleOptions } });
		expect(screen.getByText('Selection required')).toBeInTheDocument();
	});

	it('renders as disabled when disabled prop is true', () => {
		render(SelectField, { props: { disabled: true, options: sampleOptions } });
		const select = screen.getByRole('combobox') as HTMLButtonElement;
		expect(select).toBeDisabled();
		expect(select.style.backgroundColor).toContain('var(--bg-input-disabled');
	});

	it('shows required asterisk when required is true', () => {
		render(SelectField, { props: { label: 'Category', required: true, options: sampleOptions } });
		const label = screen.getByText('Category', { exact: false }).closest('label');
		expect(label?.textContent).toContain('*');
	});

	it('applies z-50 elevation when open and triggers onchange callback', async () => {
		let selected = '';
		const { container } = render(SelectField, {
			props: {
				options: sampleOptions,
				onchange: (val) => {
					selected = val;
				}
			}
		});

		const wrapper = container.firstElementChild as HTMLElement;
		expect(wrapper.className).toContain('z-0');

		const trigger = screen.getByRole('combobox');
		await fireEvent.click(trigger);

		expect(wrapper.className).toContain('z-50');

		const option2 = screen.getByRole('option', { name: 'Option 2' });
		await fireEvent.click(option2);

		expect(selected).toBe('opt2');
		expect(wrapper.className).toContain('z-0');
	});
});
