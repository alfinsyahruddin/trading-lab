import { LS_THEME } from '$lib/constants';

export type Theme = 'dark' | 'light';

/**
 * Reads the stored theme preference and applies it to the document.
 * Falls back to system preference if nothing is stored.
 * Call this in onMount of the root layout.
 */
export function initTheme(): Theme {
	const stored = localStorage.getItem(LS_THEME) as Theme | null;
	if (stored === 'dark' || stored === 'light') {
		applyTheme(stored);
		return stored;
	}
	const preferred: Theme = window.matchMedia('(prefers-color-scheme: dark)').matches
		? 'dark'
		: 'light';
	applyTheme(preferred);
	return preferred;
}

/**
 * Toggles the current theme and persists the preference.
 * Returns the new theme value.
 */
export function toggleTheme(): Theme {
	const current: Theme = document.documentElement.classList.contains('dark') ? 'dark' : 'light';
	const next: Theme = current === 'dark' ? 'light' : 'dark';
	applyTheme(next);
	localStorage.setItem(LS_THEME, next);
	return next;
}

/** Returns the currently active theme. */
export function getCurrentTheme(): Theme {
	return document.documentElement.classList.contains('dark') ? 'dark' : 'light';
}

function applyTheme(theme: Theme): void {
	if (theme === 'dark') {
		document.documentElement.classList.add('dark');
	} else {
		document.documentElement.classList.remove('dark');
	}
}
