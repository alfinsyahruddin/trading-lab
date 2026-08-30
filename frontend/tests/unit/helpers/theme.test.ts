import { describe, it, expect, beforeEach } from 'vitest';

// Mock localStorage
const localStorageMock = (() => {
	let store: Record<string, string> = {};
	return {
		getItem: (key: string) => store[key] ?? null,
		setItem: (key: string, value: string) => {
			store[key] = value;
		},
		removeItem: (key: string) => {
			delete store[key];
		},
		clear: () => {
			store = {};
		}
	};
})();

Object.defineProperty(globalThis, 'localStorage', { value: localStorageMock });

// Mock matchMedia
Object.defineProperty(globalThis, 'matchMedia', {
	value: (query: string) => ({
		matches: query.includes('dark'),
		addListener: () => {},
		removeListener: () => {}
	})
});

// Mock document.documentElement.classList
const classList = new Set<string>();
Object.defineProperty(globalThis.document, 'documentElement', {
	value: {
		classList: {
			add: (c: string) => classList.add(c),
			remove: (c: string) => classList.delete(c),
			contains: (c: string) => classList.has(c)
		}
	},
	configurable: true
});

import { initTheme, toggleTheme, getCurrentTheme } from '$lib/helpers/theme';

describe('theme helper', () => {
	beforeEach(() => {
		localStorageMock.clear();
		classList.clear();
	});

	it('initTheme returns dark when system prefers dark and nothing stored', () => {
		const theme = initTheme();
		expect(theme).toBe('dark');
		expect(classList.has('dark')).toBe(true);
	});

	it('initTheme uses stored preference over system preference', () => {
		localStorage.setItem('trading_lab_theme', 'light');
		const theme = initTheme();
		expect(theme).toBe('light');
		expect(classList.has('dark')).toBe(false);
	});

	it('initTheme applies dark theme from storage', () => {
		localStorage.setItem('trading_lab_theme', 'dark');
		const theme = initTheme();
		expect(theme).toBe('dark');
		expect(classList.has('dark')).toBe(true);
	});

	it('toggleTheme switches from dark to light', () => {
		classList.add('dark');
		const result = toggleTheme();
		expect(result).toBe('light');
		expect(classList.has('dark')).toBe(false);
		expect(localStorage.getItem('trading_lab_theme')).toBe('light');
	});

	it('toggleTheme switches from light to dark', () => {
		// ensure no dark class
		classList.delete('dark');
		const result = toggleTheme();
		expect(result).toBe('dark');
		expect(classList.has('dark')).toBe(true);
		expect(localStorage.getItem('trading_lab_theme')).toBe('dark');
	});

	it('getCurrentTheme returns dark when dark class is present', () => {
		classList.add('dark');
		expect(getCurrentTheme()).toBe('dark');
	});

	it('getCurrentTheme returns light when dark class is absent', () => {
		classList.delete('dark');
		expect(getCurrentTheme()).toBe('light');
	});
});
