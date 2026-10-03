import { describe, it, expect } from 'vitest';
import { formatRiskReward, formatTimeAgo } from '#lib/constants.js';

describe('Strategy Constants & Formatters', () => {
	it('formats risk reward ratio as 1 : ratio without redundant trailing zeros (e.g. 1 : 2.00 -> 1 : 2)', () => {
		expect(formatRiskReward(10, 5)).toBe('1 : 2');
		expect(formatRiskReward(15, 10)).toBe('1 : 1.5');
		expect(formatRiskReward(20, 10)).toBe('1 : 2');
		expect(formatRiskReward(7, 3)).toBe('1 : 2.33');
		expect(formatRiskReward(10, 0)).toBe('—');
		expect(formatRiskReward(10, -1)).toBe('—');
		expect(formatRiskReward(NaN, 10)).toBe('—');
	});

	it('formats time ago relative to now', () => {
		const now = new Date();
		const oneMinAgo = new Date(now.getTime() - 65 * 1000).toISOString();
		const threeMinsAgo = new Date(now.getTime() - 180 * 1000).toISOString();
		const twoHoursAgo = new Date(now.getTime() - 2 * 3600 * 1000).toISOString();
		const fiveDaysAgo = new Date(now.getTime() - 5 * 86400 * 1000).toISOString();

		expect(formatTimeAgo(oneMinAgo)).toBe('1 minute ago');
		expect(formatTimeAgo(threeMinsAgo)).toBe('3 minutes ago');
		expect(formatTimeAgo(twoHoursAgo)).toBe('2 hours ago');
		expect(formatTimeAgo(fiveDaysAgo)).toBe('5 days ago');
		expect(formatTimeAgo('')).toBe('—');
	});
});
