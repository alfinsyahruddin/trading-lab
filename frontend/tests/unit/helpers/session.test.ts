import { describe, it, expect, beforeEach, vi } from 'vitest';

// Mock localStorage and document.cookie
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

let cookieStore = '';
Object.defineProperty(globalThis.document, 'cookie', {
	get: () => cookieStore,
	set: (val: string) => {
		cookieStore = val;
	},
	configurable: true
});

import {
	persistSession,
	getToken,
	getRefreshToken,
	getUser,
	clearSession
} from '$lib/helpers/session';
import type { UserResponse } from '$lib/types';

const mockUser: UserResponse = {
	id: '123e4567-e89b-12d3-a456-426614174000',
	name: 'Admin User',
	email: 'admin@mail.com',
	role: 'ADMIN',
	created_at: '2026-08-30T00:00:00Z',
	updated_at: '2026-08-30T00:00:00Z'
};

describe('session helper', () => {
	beforeEach(() => {
		localStorageMock.clear();
		cookieStore = '';
	});

	it('persistSession stores token and user in localStorage', () => {
		persistSession('access-token', 'refresh-token', mockUser);
		expect(getToken()).toBe('access-token');
		expect(getRefreshToken()).toBe('refresh-token');
	});

	it('getUser returns the stored user', () => {
		persistSession('access-token', 'refresh-token', mockUser);
		const user = getUser();
		expect(user).not.toBeNull();
		expect(user?.email).toBe('admin@mail.com');
		expect(user?.role).toBe('ADMIN');
	});

	it('getToken returns null when not set', () => {
		expect(getToken()).toBeNull();
	});

	it('getUser returns null when not set', () => {
		expect(getUser()).toBeNull();
	});

	it('clearSession removes all auth data from localStorage', () => {
		persistSession('access-token', 'refresh-token', mockUser);
		clearSession();
		expect(getToken()).toBeNull();
		expect(getRefreshToken()).toBeNull();
		expect(getUser()).toBeNull();
	});

	it('getUser handles corrupted JSON gracefully', () => {
		localStorage.setItem('trading_lab_user', 'not-valid-json{');
		expect(getUser()).toBeNull();
	});
});
