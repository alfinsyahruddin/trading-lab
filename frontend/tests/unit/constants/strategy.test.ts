import { describe, it, expect } from 'vitest';
import {
	STRATEGY_OPERATORS,
	STRATEGY_VARIABLES,
	VARIABLE_CATEGORIES,
	formatRiskReward,
	formatTimeAgo
} from '$lib/constants';

describe('Strategy Constants', () => {
	it('defines exactly the 9 required operators', () => {
		expect(STRATEGY_OPERATORS).toHaveLength(9);
		const values = STRATEGY_OPERATORS.map((o) => o.value);
		expect(values).toEqual(['=', '!=', '>', '<', '>=', '<=', '~~', 'in', 'is']);
	});

	it('strictly contains only price and volume in Price & Market category', () => {
		const priceMarketVars = STRATEGY_VARIABLES.filter((v) => v.category === 'Price & Market');
		expect(priceMarketVars.map((v) => v.code)).toEqual(['price', 'volume']);
		expect(priceMarketVars.every((v) => !v.isHistorical)).toBe(true);
	});

	it('includes insurance metrics in Income Statement category', () => {
		const incomeVars = STRATEGY_VARIABLES.filter((v) => v.category === 'Income Statement');
		const codes = incomeVars.map((v) => v.code);
		expect(codes).toContain('premium_income');
		expect(codes).toContain('net_premium_income');
		expect(codes).toContain('premium_expense');
	});

	it('includes valuation ratios in Valuation Ratios category', () => {
		const valVars = STRATEGY_VARIABLES.filter((v) => v.category === 'Valuation Ratios');
		const codes = valVars.map((v) => v.code);
		expect(codes).toContain('pe');
		expect(codes).toContain('pb');
		expect(codes).toContain('ps');
		expect(codes).toContain('pcf');
		expect(codes).toContain('peg');
	});

	it('has unique variable codes across all categories', () => {
		const codes = STRATEGY_VARIABLES.map((v) => v.code);
		const uniqueCodes = new Set(codes);
		expect(uniqueCodes.size).toBe(codes.length);
	});

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
