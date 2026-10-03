import { LS_REFRESH, LS_REMEMBERED_ACCOUNTS, LS_TOKEN, LS_USER } from '#lib/constants.js';
import type { RememberedAccount, UserResponse } from '#lib/types.js';

/**
 * Persists auth session to localStorage for client-side API calls and auth guards.
 */
export function persistSession(
	accessToken: string,
	refreshToken: string,
	user: UserResponse
): void {
	localStorage.setItem(LS_TOKEN, accessToken);
	localStorage.setItem(LS_REFRESH, refreshToken);
	localStorage.setItem(LS_USER, JSON.stringify(user));
}

/** Returns the stored access token, or null if not found. */
export function getToken(): string | null {
	return localStorage.getItem(LS_TOKEN);
}

/** Returns the stored refresh token, or null if not found. */
export function getRefreshToken(): string | null {
	return localStorage.getItem(LS_REFRESH);
}

/** Returns the stored user object, or null if not found. */
export function getUser(): UserResponse | null {
	const raw = localStorage.getItem(LS_USER);
	if (!raw) return null;
	try {
		return JSON.parse(raw) as UserResponse;
	} catch {
		return null;
	}
}

/** Updates the stored user object in localStorage. */
export function updateUserSession(user: UserResponse): void {
	localStorage.setItem(LS_USER, JSON.stringify(user));
}

/** Returns the list of remembered accounts for quick login. */
export function getRememberedAccounts(): RememberedAccount[] {
	const raw = localStorage.getItem(LS_REMEMBERED_ACCOUNTS);
	if (!raw) return [];
	try {
		const parsed = JSON.parse(raw);
		return Array.isArray(parsed) ? parsed : [];
	} catch {
		return [];
	}
}

/** Adds or updates an account in the remembered accounts list (brought to top). */
export function saveRememberedAccount(account: RememberedAccount): void {
	const accounts = getRememberedAccounts().filter(
		(a) => a.email.toLowerCase() !== account.email.toLowerCase()
	);
	accounts.unshift(account);
	localStorage.setItem(LS_REMEMBERED_ACCOUNTS, JSON.stringify(accounts));
}

/** Removes an account from the remembered accounts list. */
export function removeRememberedAccount(email: string): void {
	const accounts = getRememberedAccounts().filter(
		(a) => a.email.toLowerCase() !== email.toLowerCase()
	);
	localStorage.setItem(LS_REMEMBERED_ACCOUNTS, JSON.stringify(accounts));
}

/**
 * Clears all auth data from localStorage.
 * Should be called on logout.
 */
export function clearSession(): void {
	localStorage.removeItem(LS_TOKEN);
	localStorage.removeItem(LS_REFRESH);
	localStorage.removeItem(LS_USER);
}
