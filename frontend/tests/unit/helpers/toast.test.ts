import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';

// Mock toast.svelte.ts for unit testing
vi.mock('#lib/helpers/toast.svelte', () => {
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

	function dismissToast(id: number) {
		const idx = toasts.findIndex((t) => t.id === id);
		if (idx !== -1) toasts.splice(idx, 1);
	}

	return {
		toasts,
		showToast,
		dismissToast,
		toast: {
			success: (msg: string, d?: number) => showToast('success', msg, d),
			error: (msg: string, d?: number) => showToast('error', msg, d),
			info: (msg: string, d?: number) => showToast('info', msg, d),
			dismiss: (id: number) => dismissToast(id)
		}
	};
});

import { toast, toasts, dismissToast } from '#lib/helpers/toast.svelte';

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
		const ids = (toasts as Array<{ id: number }>).map((t) => t.id);
		expect(new Set(ids).size).toBe(ids.length);
	});

	it('dismissToast removes a toast by id', () => {
		toast.success('To dismiss');
		expect(toasts).toHaveLength(1);
		const id = toasts[0].id;
		dismissToast(id);
		expect(toasts).toHaveLength(0);
	});

	it('toast.dismiss removes a toast by id', () => {
		toast.error('To dismiss');
		expect(toasts).toHaveLength(1);
		const id = toasts[0].id;
		toast.dismiss(id);
		expect(toasts).toHaveLength(0);
	});
});
