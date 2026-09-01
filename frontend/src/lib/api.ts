import { clearSession, getRefreshToken, getToken, persistSession } from '$lib/helpers/session';
import type {
	CreateStrategyPayload,
	LoginResponse,
	TradingStrategy,
	UpdateStrategyPayload,
	UserResponse,
	UserRole,
	BacktestJob,
	CreateBacktestPayload,
	UpdateBacktestPayload,
	DashboardStats,
	LeaderboardEntry
} from '$lib/types';

const BASE_URL = import.meta.env.PUBLIC_API_BASE_URL || 'http://127.0.0.1:8000';

export class ApiError extends Error {
	constructor(
		public override message: string,
		public status: number
	) {
		super(message);
		this.name = 'ApiError';
	}
}

interface ApiEnvelope<T> {
	data: T | null;
	status: number;
	message: string | null;
	timestamp: string;
}

let refreshPromise: Promise<string | null> | null = null;

/**
 * Handles terminal authentication failure: clears localStorage and redirects to /login.
 */
export function handleAuthFailure(): void {
	clearSession();
	if (
		typeof window !== 'undefined' &&
		!window.location.pathname.startsWith('/login') &&
		!window.location.pathname.startsWith('/register')
	) {
		window.location.href = '/login';
	}
}

/**
 * Attempts to obtain a new access token using the stored refresh token.
 * Deduplicates simultaneous concurrent refresh requests.
 */
export async function attemptTokenRefresh(): Promise<string | null> {
	const refreshToken = getRefreshToken();
	if (!refreshToken) {
		handleAuthFailure();
		return null;
	}

	if (refreshPromise) {
		return refreshPromise;
	}

	refreshPromise = (async () => {
		try {
			const res = await fetch(`${BASE_URL}/api/users/refresh`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ refresh_token: refreshToken })
			});

			const envelope: ApiEnvelope<LoginResponse> = await res.json();
			if (!res.ok || envelope.status >= 400 || !envelope.data) {
				handleAuthFailure();
				return null;
			}

			const { user, tokens } = envelope.data;
			persistSession(tokens.access_token, tokens.refresh_token, user);
			return tokens.access_token;
		} catch {
			handleAuthFailure();
			return null;
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}

async function request<T>(
	path: string,
	init?: RequestInit,
	token?: string,
	retryOnAuth = true
): Promise<T> {
	const authToken = token ?? getToken() ?? undefined;
	const headers: Record<string, string> = {
		'Content-Type': 'application/json'
	};
	if (authToken) {
		headers['Authorization'] = `Bearer ${authToken}`;
	}

	const res = await fetch(`${BASE_URL}${path}`, {
		...init,
		headers: { ...headers, ...(init?.headers ?? {}) }
	});

	let envelope: ApiEnvelope<T>;
	try {
		envelope = await res.json();
	} catch {
		if (
			res.status === 401 &&
			retryOnAuth &&
			path !== '/api/users/login' &&
			path !== '/api/users/register' &&
			path !== '/api/users/refresh'
		) {
			const newToken = await attemptTokenRefresh();
			if (newToken) {
				return request<T>(path, init, newToken, false);
			}
		}
		throw new ApiError('An unexpected server response occurred', res.status);
	}

	if (!res.ok || envelope.status >= 400) {
		if (
			(res.status === 401 || envelope.status === 401) &&
			retryOnAuth &&
			path !== '/api/users/login' &&
			path !== '/api/users/register' &&
			path !== '/api/users/refresh'
		) {
			const newToken = await attemptTokenRefresh();
			if (newToken) {
				return request<T>(path, init, newToken, false);
			}
		}

		throw new ApiError(
			envelope.message || 'An unexpected error occurred',
			envelope.status || res.status
		);
	}

	return envelope.data as T;
}

// --- Auth ---

export function login(email: string, password: string): Promise<LoginResponse> {
	return request<LoginResponse>('/api/users/login', {
		method: 'POST',
		body: JSON.stringify({ email, password })
	});
}

export function register(name: string, email: string, password: string): Promise<UserResponse> {
	return request<UserResponse>('/api/users/register', {
		method: 'POST',
		body: JSON.stringify({ name, email, password })
	});
}

export function logout(token?: string): Promise<string> {
	return request<string>('/api/users/logout', { method: 'POST' }, token);
}

export function refreshTokens(refreshToken: string): Promise<LoginResponse> {
	return request<LoginResponse>('/api/users/refresh', {
		method: 'POST',
		body: JSON.stringify({ refresh_token: refreshToken })
	});
}

// --- Profile & Account ---

export function getCurrentUser(token?: string): Promise<UserResponse> {
	return request<UserResponse>('/api/users/me', { method: 'GET' }, token);
}

export function updateProfile(
	data: { name: string; email: string },
	token?: string
): Promise<UserResponse> {
	return request<UserResponse>(
		'/api/users/me',
		{
			method: 'PATCH',
			body: JSON.stringify(data)
		},
		token
	);
}

export function changePassword(
	data: { current_password: string; new_password: string },
	token?: string
): Promise<string> {
	return request<string>(
		'/api/users/me/password',
		{
			method: 'POST',
			body: JSON.stringify(data)
		},
		token
	);
}

// --- Users (Admin only) ---

export function listUsers(token?: string): Promise<UserResponse[]> {
	return request<UserResponse[]>('/api/users', { method: 'GET' }, token);
}

export function getUser(token: string | undefined, id: string): Promise<UserResponse> {
	return request<UserResponse>(`/api/users/${id}`, { method: 'GET' }, token);
}

export function createUser(
	token: string | undefined,
	data: { name: string; email: string; password: string; role: UserRole }
): Promise<UserResponse> {
	return request<UserResponse>('/api/users', { method: 'POST', body: JSON.stringify(data) }, token);
}

export function updateUser(
	token: string | undefined,
	id: string,
	data: { name?: string; email?: string; password?: string; role?: UserRole }
): Promise<UserResponse> {
	return request<UserResponse>(
		`/api/users/${id}`,
		{ method: 'PATCH', body: JSON.stringify(data) },
		token
	);
}

export function deleteUser(token: string | undefined, id: string): Promise<string> {
	return request<string>(`/api/users/${id}`, { method: 'DELETE' }, token);
}

// --- Trading Strategies ---

export function listTradingStrategies(token?: string): Promise<TradingStrategy[]> {
	return request<TradingStrategy[]>('/api/strategies', { method: 'GET' }, token);
}

export function getTradingStrategy(
	token: string | undefined,
	id: string
): Promise<TradingStrategy> {
	return request<TradingStrategy>(`/api/strategies/${id}`, { method: 'GET' }, token);
}

export function createTradingStrategy(
	token: string | undefined,
	data: CreateStrategyPayload
): Promise<TradingStrategy> {
	return request<TradingStrategy>(
		'/api/strategies',
		{ method: 'POST', body: JSON.stringify(data) },
		token
	);
}

export function updateTradingStrategy(
	token: string | undefined,
	id: string,
	data: UpdateStrategyPayload
): Promise<TradingStrategy> {
	return request<TradingStrategy>(
		`/api/strategies/${id}`,
		{ method: 'PATCH', body: JSON.stringify(data) },
		token
	);
}

export function deleteTradingStrategy(token: string | undefined, id: string): Promise<string> {
	return request<string>(`/api/strategies/${id}`, { method: 'DELETE' }, token);
}

export function duplicateTradingStrategy(
	token: string | undefined,
	id: string,
	name: string
): Promise<TradingStrategy> {
	return request<TradingStrategy>(
		`/api/strategies/${id}/duplicate`,
		{ method: 'POST', body: JSON.stringify({ name }) },
		token
	);
}

// --- Backtests ---

export function listBacktests(token?: string): Promise<BacktestJob[]> {
	return request<BacktestJob[]>('/api/backtests', { method: 'GET' }, token);
}

export function getBacktest(token: string | undefined, id: string): Promise<BacktestJob> {
	return request<BacktestJob>(`/api/backtests/${id}`, { method: 'GET' }, token);
}

export function createBacktest(
	token: string | undefined,
	data: CreateBacktestPayload
): Promise<BacktestJob> {
	return request<BacktestJob>(
		'/api/backtests',
		{ method: 'POST', body: JSON.stringify(data) },
		token
	);
}

export function updateBacktest(
	token: string | undefined,
	id: string,
	data: UpdateBacktestPayload
): Promise<BacktestJob> {
	return request<BacktestJob>(
		`/api/backtests/${id}`,
		{ method: 'PATCH', body: JSON.stringify(data) },
		token
	);
}

export function deleteBacktest(token: string | undefined, id: string): Promise<void> {
	return request<void>(`/api/backtests/${id}`, { method: 'DELETE' }, token);
}

// --- Dashboard ---

export function getDashboardStats(token?: string): Promise<DashboardStats> {
	return request<DashboardStats>('/api/dashboard/stats', { method: 'GET' }, token);
}

export function getLeaderboard(token?: string): Promise<LeaderboardEntry[]> {
	return request<LeaderboardEntry[]>('/api/dashboard/leaderboard', { method: 'GET' }, token);
}

export function getTopStars(token?: string): Promise<LeaderboardEntry[]> {
	return request<LeaderboardEntry[]>('/api/dashboard/top-stars', { method: 'GET' }, token);
}

export function starBacktest(token: string | undefined, backtestId: string): Promise<string> {
	return request<string>(`/api/dashboard/stars/${backtestId}`, { method: 'POST' }, token);
}

export function unstarBacktest(token: string | undefined, backtestId: string): Promise<string> {
	return request<string>(`/api/dashboard/stars/${backtestId}`, { method: 'DELETE' }, token);
}
