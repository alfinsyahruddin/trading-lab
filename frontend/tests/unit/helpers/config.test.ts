import { describe, it, expect, beforeEach } from 'vitest';
import { isComingSoon } from '$lib/helpers/config';

describe('isComingSoon helper', () => {
	beforeEach(() => {
		delete (import.meta.env as Record<string, unknown>).PUBLIC_IS_COMING_SOON;
	});

	it('defaults to true when env variable is not set', () => {
		expect(isComingSoon()).toBe(true);
	});

	it('returns true when PUBLIC_IS_COMING_SOON is "true"', () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'true';
		expect(isComingSoon()).toBe(true);
	});

	it('returns false when PUBLIC_IS_COMING_SOON is "false"', () => {
		import.meta.env.PUBLIC_IS_COMING_SOON = 'false';
		expect(isComingSoon()).toBe(false);
	});
});
