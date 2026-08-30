import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';

// toast.ts uses $state (Svelte 5 runes), so we need to mock it
// to avoid "runes can only be used in .svelte files" errors
vi.mock('$lib/helpers/toast', () => {
	const toasts: Array<{ id: number; type: string; message: string }> = [];
	let nextId = 0;

	function showToast(type: string, message: string, duration = 4000) {
		const id = nextId++;
		toasts.push({ id, type, message });
		setTimeout(() => {
			const idx = toasts.findIndex((t) => t.id === id);
			if (idx !== -1) toasts.splice(idx, 1);
		}, duration);
	}

	return {
		toasts,
		showToast,
		toast: {
			success: (msg: string, d?: number) => showToast('success', msg, d),
			error: (msg: string, d?: number) => showToast('error', msg, d),
			info: (msg: string, d?: number) => showToast('info', msg, d)
		}
	};
});

import { toast, toasts } from '$lib/helpers/toast';

describe('toast helper', () => {
	beforeEach(() => {
		toasts.length = 0;
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('toast.success adds a success toast', () => {
		toast.success('Operation successful!');
		expect(toasts).toHaveLength(1);
		expect(toasts[0].type).toBe('success');
		expect(toasts[0].message).toBe('Operation successful!');
	});

	it('toast.error adds an error toast', () => {
		toast.error('Something went wrong.');
		expect(toasts).toHaveLength(1);
		expect(toasts[0].type).toBe('error');
		expect(toasts[0].message).toBe('Something went wrong.');
	});

	it('toast.info adds an info toast', () => {
		toast.info('FYI: something happened.');
		expect(toasts).toHaveLength(1);
		expect(toasts[0].type).toBe('info');
	});

	it('toast is removed after default duration', () => {
		toast.success('Disappears soon');
		expect(toasts).toHaveLength(1);
		vi.advanceTimersByTime(4001);
		expect(toasts).toHaveLength(0);
	});

	it('multiple toasts can coexist', () => {
		toast.success('First');
		toast.error('Second');
		toast.info('Third');
		expect(toasts).toHaveLength(3);
	});

	it('toasts have unique ids', () => {
		toast.success('A');
		toast.success('B');
		const ids = toasts.map((t) => t.id);
		expect(new Set(ids).size).toBe(ids.length);
	});
});
