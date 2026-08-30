import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { listTradingStrategies, ApiError } from '$lib/api';
import * as session from '$lib/helpers/session';
import type { UserResponse } from '$lib/types';

describe('API client auto-refresh and auth handling', () => {
	const mockUser: UserResponse = {
		id: '123e4567-e89b-12d3-a456-426614174000',
		name: 'Test User',
		email: 'test@example.com',
		role: 'MEMBER',
		created_at: '2026-08-30T00:00:00Z',
		updated_at: '2026-08-30T00:00:00Z'
	};

	beforeEach(() => {
		vi.restoreAllMocks();
		localStorage.clear();
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('successfully fetches data with valid token', async () => {
		session.persistSession('valid-token', 'valid-refresh', mockUser);

		const mockFetch = vi.fn().mockResolvedValueOnce({
			ok: true,
			status: 200,
			json: async () => ({
				data: [{ id: 'strat-1', name: 'Strategy 1' }],
				status: 200,
				message: null,
				timestamp: '2026-08-30T00:00:00Z'
			})
		});
		vi.stubGlobal('fetch', mockFetch);

		const res = await listTradingStrategies();
		expect(res).toHaveLength(1);
		expect(res[0].name).toBe('Strategy 1');
		expect(mockFetch).toHaveBeenCalledTimes(1);
	});

	it('automatically refreshes token and retries request on 401 response', async () => {
		session.persistSession('expired-token', 'valid-refresh', mockUser);

		const mockFetch = vi
			.fn()
			// 1. First request fails with 401
			.mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: async () => ({
					data: null,
					status: 401,
					message: 'Invalid or expired access token',
					timestamp: '2026-08-30T00:00:00Z'
				})
			})
			// 2. Token refresh request succeeds
			.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({
					data: {
						user: mockUser,
						tokens: {
							access_token: 'new-access-token',
							refresh_token: 'new-refresh-token',
							token_type: 'Bearer',
							expires_in: 900
						}
					},
					status: 200,
					message: null,
					timestamp: '2026-08-30T00:00:00Z'
				})
			})
			// 3. Retry of original request with new token succeeds
			.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({
					data: [{ id: 'strat-2', name: 'Alpha Trend' }],
					status: 200,
					message: null,
					timestamp: '2026-08-30T00:00:00Z'
				})
			});

		vi.stubGlobal('fetch', mockFetch);

		const res = await listTradingStrategies();

		// Check that the returned data is from the retried request
		expect(res).toHaveLength(1);
		expect(res[0].name).toBe('Alpha Trend');

		// Check that new tokens were persisted in storage
		expect(session.getToken()).toBe('new-access-token');
		expect(session.getRefreshToken()).toBe('new-refresh-token');

		// 3 calls: initial 401 -> refresh -> retried request
		expect(mockFetch).toHaveBeenCalledTimes(3);
	});

	it('clears session and triggers logout when refresh token fails', async () => {
		session.persistSession('expired-token', 'expired-refresh', mockUser);

		const mockFetch = vi
			.fn()
			// 1. Initial request fails with 401
			.mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: async () => ({
					data: null,
					status: 401,
					message: 'Invalid or expired access token',
					timestamp: '2026-08-30T00:00:00Z'
				})
			})
			// 2. Refresh request also fails with 401
			.mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: async () => ({
					data: null,
					status: 401,
					message: 'Invalid or expired refresh token',
					timestamp: '2026-08-30T00:00:00Z'
				})
			});

		vi.stubGlobal('fetch', mockFetch);

		await expect(listTradingStrategies()).rejects.toThrow(ApiError);

		// Session should have been cleared
		expect(session.getToken()).toBeNull();
		expect(session.getRefreshToken()).toBeNull();
		expect(session.getUser()).toBeNull();
	});
});
