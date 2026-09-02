import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import TextField from '$lib/components/TextField.svelte';

describe('TextField', () => {
	it('renders without label by default', () => {
		const { container } = render(TextField, { props: {} });
		const label = container.querySelector('label');
		expect(label).toBeNull();
	});

	it('renders a label when provided', () => {
		render(TextField, { props: { label: 'Email Address' } });
		expect(screen.getByText('Email Address')).toBeInTheDocument();
	});

	it('shows error message when error prop is provided', () => {
		render(TextField, { props: { label: 'Email', error: 'Invalid email' } });
		expect(screen.getByText('Invalid email')).toBeInTheDocument();
	});

	it('renders placeholder text', () => {
		render(TextField, { props: { placeholder: 'Enter your email' } });
		const input = screen.getByPlaceholderText('Enter your email');
		expect(input).toBeInTheDocument();
	});

	it('renders as password input when type is password', () => {
		render(TextField, { props: { type: 'password', placeholder: 'Password' } });
		const input = screen.getByPlaceholderText('Password') as HTMLInputElement;
		expect(input.type).toBe('password');
	});

	it('shows required asterisk when required is true', () => {
		render(TextField, { props: { label: 'Name', required: true } });
		// The asterisk is a child of the label
		const label = screen.getByText('Name', { exact: false }).closest('label');
		expect(label?.textContent).toContain('*');
	});
});
