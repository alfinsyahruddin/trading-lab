import { LS_REFRESH, LS_TOKEN, LS_USER } from '$lib/constants';
import type { UserResponse } from '$lib/types';

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

/**
 * Clears all auth data from localStorage.
 * Should be called on logout.
 */
export function clearSession(): void {
	localStorage.removeItem(LS_TOKEN);
	localStorage.removeItem(LS_REFRESH);
	localStorage.removeItem(LS_USER);
}
