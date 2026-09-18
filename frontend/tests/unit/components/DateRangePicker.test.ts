import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import DateRangePicker from '$lib/components/DateRangePicker.svelte';

describe('DateRangePicker', () => {
	it('renders without label by default', () => {
		const { container } = render(DateRangePicker, { props: {} });
		const label = container.querySelector('label');
		expect(label).toBeNull();
	});

	it('renders a label when provided', () => {
		render(DateRangePicker, { props: { label: 'Backtest Date Range' } });
		expect(screen.getByText('Backtest Date Range')).toBeInTheDocument();
	});

	it('shows required asterisk when required is true', () => {
		const { container } = render(DateRangePicker, {
			props: { label: 'Date Range', required: true }
		});
		const label = container.querySelector('label');
		expect(label?.textContent).toContain('Date Range');
		expect(label?.textContent).toContain('*');
	});

	it('renders placeholder when dates are null', () => {
		render(DateRangePicker, { props: { placeholder: 'Select custom range' } });
		expect(screen.getByText('Select custom range')).toBeInTheDocument();
	});

	it('displays formatted date range when startDate and endDate are provided', () => {
		const startDate = new Date(2026, 0, 1);
		const endDate = new Date(2026, 2, 31);
		render(DateRangePicker, { props: { startDate, endDate } });
		expect(screen.getByText('1 Jan 2026 - 31 Mar 2026')).toBeInTheDocument();
	});

	it('renders as disabled when disabled prop is true', () => {
		render(DateRangePicker, { props: { disabled: true } });
		const trigger = screen.getByRole('button');
		expect(trigger).toBeDisabled();
	});

	it('opens calendar popover when clicked', async () => {
		const startDate = new Date(2026, 0, 1);
		const endDate = new Date(2026, 2, 31);
		render(DateRangePicker, { props: { startDate, endDate } });

		const trigger = screen.getByRole('button');
		expect(screen.queryByLabelText('Previous Month')).not.toBeInTheDocument();

		await fireEvent.click(trigger);
		expect(screen.getByLabelText('Previous Month')).toBeInTheDocument();
		expect(screen.getByLabelText('Next Month')).toBeInTheDocument();
	});

	it('navigates months when clicking navigation buttons', async () => {
		const startDate = new Date(2026, 0, 1);
		const endDate = new Date(2026, 2, 31);
		render(DateRangePicker, { props: { startDate, endDate } });

		const trigger = screen.getByRole('button');
		await fireEvent.click(trigger);

		expect(screen.getByText('January 2026')).toBeInTheDocument();

		const nextBtn = screen.getByLabelText('Next Month');
		await fireEvent.click(nextBtn);
		expect(screen.getByText('February 2026')).toBeInTheDocument();

		const prevBtn = screen.getByLabelText('Previous Month');
		await fireEvent.click(prevBtn);
		expect(screen.getByText('January 2026')).toBeInTheDocument();
	});

	it('handles date range selection and triggers onchange', async () => {
		const onchange = vi.fn();
		render(DateRangePicker, {
			props: {
				startDate: null,
				endDate: null,
				minDate: new Date(2026, 0, 1),
				maxDate: new Date(2026, 11, 31),
				onchange
			}
		});

		const trigger = screen.getByRole('button');
		await fireEvent.click(trigger);

		// Click Jan 10
		const day10Btn = screen.getByRole('button', { name: '10' });
		await fireEvent.click(day10Btn);

		// Click Jan 20
		const day20Btn = screen.getByRole('button', { name: '20' });
		await fireEvent.click(day20Btn);

		expect(onchange).toHaveBeenCalledTimes(1);
	});
});
