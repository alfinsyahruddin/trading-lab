import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SelectField from '$lib/components/SelectField.svelte';

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

	it('renders all options', () => {
		render(SelectField, { props: { options: sampleOptions } });
		expect(screen.getByRole('combobox')).toBeInTheDocument();
		expect(screen.getByText('Option 1')).toBeInTheDocument();
		expect(screen.getByText('Option 2')).toBeInTheDocument();
	});

	it('shows error message when error prop is provided', () => {
		render(SelectField, { props: { error: 'Selection required', options: sampleOptions } });
		expect(screen.getByText('Selection required')).toBeInTheDocument();
	});

	it('renders as disabled when disabled prop is true', () => {
		render(SelectField, { props: { disabled: true, options: sampleOptions } });
		const select = screen.getByRole('combobox') as HTMLSelectElement;
		expect(select).toBeDisabled();
		expect(select.style.backgroundColor).toContain('var(--bg-input-disabled');
	});

	it('shows required asterisk when required is true', () => {
		render(SelectField, { props: { label: 'Category', required: true, options: sampleOptions } });
		const label = screen.getByText('Category', { exact: false }).closest('label');
		expect(label?.textContent).toContain('*');
	});
});
