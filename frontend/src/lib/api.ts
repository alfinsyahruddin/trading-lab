import type { LoginResponse, UserResponse } from '$lib/types';
import type { UserRole } from '$lib/types';

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

async function request<T>(path: string, init?: RequestInit, token?: string): Promise<T> {
	const headers: Record<string, string> = {
		'Content-Type': 'application/json'
	};
	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	const res = await fetch(`${BASE_URL}${path}`, {
		...init,
		headers: { ...headers, ...(init?.headers ?? {}) }
	});

	const envelope: ApiEnvelope<T> = await res.json();

	if (!res.ok || envelope.status >= 400) {
		throw new ApiError(envelope.message || 'An unexpected error occurred', envelope.status);
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

export function logout(token: string): Promise<string> {
	return request<string>('/api/users/logout', { method: 'POST' }, token);
}

export function refreshTokens(refreshToken: string): Promise<LoginResponse> {
	return request<LoginResponse>('/api/users/refresh', {
		method: 'POST',
		body: JSON.stringify({ refresh_token: refreshToken })
	});
}

// --- Users (Admin only) ---

export function listUsers(token: string): Promise<UserResponse[]> {
	return request<UserResponse[]>('/api/users', { method: 'GET' }, token);
}

export function getUser(token: string, id: string): Promise<UserResponse> {
	return request<UserResponse>(`/api/users/${id}`, { method: 'GET' }, token);
}

export function createUser(
	token: string,
	data: { name: string; email: string; password: string; role: UserRole }
): Promise<UserResponse> {
	return request<UserResponse>('/api/users', { method: 'POST', body: JSON.stringify(data) }, token);
}

export function updateUser(
	token: string,
	id: string,
	data: { name?: string; email?: string; password?: string; role?: UserRole }
): Promise<UserResponse> {
	return request<UserResponse>(
		`/api/users/${id}`,
		{ method: 'PATCH', body: JSON.stringify(data) },
		token
	);
}

export function deleteUser(token: string, id: string): Promise<string> {
	return request<string>(`/api/users/${id}`, { method: 'DELETE' }, token);
}
