import { describe, it, expect, vi, beforeEach } from 'vitest';
import { redirect } from '@sveltejs/kit';
import { getToken, persistSession } from '#lib/helpers/session.js';
import type { UserResponse } from '#lib/types.js';

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

// Mock @sveltejs/kit redirect
vi.mock('@sveltejs/kit', () => ({
	redirect: (status: number, location: string) => {
		const err = new Error(`Redirect ${status} to ${location}`) as Error & {
			status: number;
			location: string;
		};
		err.status = status;
		err.location = location;
		throw err;
	}
}));

// Simulate the CSR auth guard logic (mirrors dashboard/+layout.ts)
function dashboardGuard() {
	const token = getToken();
	if (!token) {
		redirect(302, '/login');
	}
}

// Simulate the login/register guard logic (mirrors login/+page.ts & register/+page.ts)
function authEntryGuard() {
	const token = getToken();
	if (token) {
		redirect(302, '/dashboard');
	}
}

const mockUser: UserResponse = {
	id: '123',
	name: 'Admin',
	email: 'admin@mail.com',
	role: 'ADMIN',
	created_at: '2026-08-30T00:00:00Z',
	updated_at: '2026-08-30T00:00:00Z'
};

describe('CSR auth guards', () => {
	beforeEach(() => {
		localStorageMock.clear();
	});

	describe('dashboard guard (requires auth)', () => {
		it('does not redirect when token is present', () => {
			persistSession('valid-token', 'refresh-token', mockUser);
			expect(() => dashboardGuard()).not.toThrow();
		});

		it('redirects to /login when no token', () => {
			expect(() => dashboardGuard()).toThrowError('Redirect 302 to /login');
		});
	});

	describe('login/register auth entry guard (redirect if authenticated)', () => {
		it('does not redirect when no token', () => {
			expect(() => authEntryGuard()).not.toThrow();
		});

		it('redirects to /dashboard when token is present', () => {
			persistSession('valid-token', 'refresh-token', mockUser);
			expect(() => authEntryGuard()).toThrowError('Redirect 302 to /dashboard');
		});
	});
});
