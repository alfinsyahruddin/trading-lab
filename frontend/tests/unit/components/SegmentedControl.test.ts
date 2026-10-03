import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import SegmentedControl from '#lib/components/SegmentedControl.svelte';

describe('SegmentedControl', () => {
	const options = [
		{ value: false, label: 'Private', icon: 'lucide:lock' },
		{ value: true, label: 'Public', icon: 'lucide:globe' }
	];

	it('renders all options', () => {
		render(SegmentedControl, {
			props: { options, value: false }
		});

		expect(screen.getByText('Private')).toBeInTheDocument();
		expect(screen.getByText('Public')).toBeInTheDocument();
	});

	it('marks the active option as aria-pressed', () => {
		render(SegmentedControl, {
			props: { options, value: true }
		});

		const publicBtn = screen.getByRole('button', { name: /public/i });
		const privateBtn = screen.getByRole('button', { name: /private/i });

		expect(publicBtn).toHaveAttribute('aria-pressed', 'true');
		expect(privateBtn).toHaveAttribute('aria-pressed', 'false');
	});

	it('triggers onchange when a segment is clicked', async () => {
		const user = userEvent.setup();
		const onchange = vi.fn();
		render(SegmentedControl, {
			props: { options, value: false, onchange }
		});

		const publicBtn = screen.getByRole('button', { name: /public/i });
		await user.click(publicBtn);

		expect(onchange).toHaveBeenCalledWith(true);
	});

	it('applies custom background color when isInsideCard is false', () => {
		render(SegmentedControl, {
			props: { options, value: false, isInsideCard: false }
		});

		const track = screen.getByRole('group');
		expect(track).toHaveStyle('--segmented-light-bg: #e4e6ec');
	});
});
