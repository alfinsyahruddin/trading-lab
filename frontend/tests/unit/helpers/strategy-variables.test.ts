import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
	fetchStrategyMetadata,
	getStrategyMetadata,
	getStrategyVariables,
	getStrategyOperators,
	getVariableCategories,
	findStrategyVariable,
	findStrategyOperator,
	setStrategyMetadata,
	resetStrategyMetadata
} from '$lib/helpers/strategy-variables.svelte';
import * as api from '$lib/api';
import { mockStrategyMetadata } from '../../fixtures/strategy-metadata';

describe('Strategy Variables Reactive Helper', () => {
	beforeEach(() => {
		resetStrategyMetadata();
		vi.restoreAllMocks();
	});

	it('returns initial empty metadata before load', () => {
		expect(getStrategyVariables()).toEqual([]);
		expect(getVariableCategories()).toEqual([]);
		expect(getStrategyOperators()).toEqual([]);
	});

	it('sets metadata directly using setStrategyMetadata', () => {
		setStrategyMetadata(mockStrategyMetadata);

		expect(getStrategyVariables()).toHaveLength(114);
		expect(getVariableCategories()).toHaveLength(10);
		expect(getStrategyOperators()).toHaveLength(9);
		expect(getStrategyMetadata()).toEqual(mockStrategyMetadata);
	});

	it('fetches metadata from API via fetchStrategyMetadata', async () => {
		const apiSpy = vi
			.spyOn(api, 'getStrategyVariables')
			.mockResolvedValueOnce(mockStrategyMetadata);

		const result = await fetchStrategyMetadata();

		expect(apiSpy).toHaveBeenCalledTimes(1);
		expect(result).toEqual(mockStrategyMetadata);
		expect(getStrategyVariables()).toHaveLength(114);
	});

	it('deduplicates simultaneous fetchStrategyMetadata calls', async () => {
		const apiSpy = vi
			.spyOn(api, 'getStrategyVariables')
			.mockResolvedValueOnce(mockStrategyMetadata);

		const [res1, res2] = await Promise.all([fetchStrategyMetadata(), fetchStrategyMetadata()]);

		expect(apiSpy).toHaveBeenCalledTimes(1);
		expect(res1).toEqual(mockStrategyMetadata);
		expect(res2).toEqual(mockStrategyMetadata);
	});

	it('returns cached metadata on subsequent fetchStrategyMetadata calls', async () => {
		const apiSpy = vi
			.spyOn(api, 'getStrategyVariables')
			.mockResolvedValueOnce(mockStrategyMetadata);

		await fetchStrategyMetadata();
		await fetchStrategyMetadata();

		expect(apiSpy).toHaveBeenCalledTimes(1);
	});

	it('finds variable by code case-insensitively', () => {
		setStrategyMetadata(mockStrategyMetadata);

		const pe = findStrategyVariable('pe');
		expect(pe).toBeDefined();
		expect(pe?.name).toBe('P/E Ratio');
		expect(pe?.category).toBe('Valuation Ratios');

		const peUpper = findStrategyVariable('PE');
		expect(peUpper).toEqual(pe);

		const missing = findStrategyVariable('non_existent_var');
		expect(missing).toBeUndefined();

		expect(findStrategyVariable('')).toBeUndefined();
	});

	it('finds operator by value', () => {
		setStrategyMetadata(mockStrategyMetadata);

		const eq = findStrategyOperator('=');
		expect(eq).toBeDefined();
		expect(eq?.label).toBe('equals');
		expect(eq?.symbol).toBe('[=]');

		const gte = findStrategyOperator('>=');
		expect(gte).toBeDefined();
		expect(gte?.display).toBe('[>=] greater than or equals');

		expect(findStrategyOperator('unknown_op')).toBeUndefined();
		expect(findStrategyOperator('')).toBeUndefined();
	});

	it('verifies categories and historical flags on variables', () => {
		setStrategyMetadata(mockStrategyMetadata);

		const priceMarketVars = getStrategyVariables().filter((v) => v.category === 'Price & Market');
		expect(priceMarketVars.map((v) => v.code)).toEqual([
			'price',
			'volume',
			'value',
			'market_cap',
			'last_1_week_foreign_flow',
			'last_1_month_foreign_flow',
			'last_3_months_foreign_flow'
		]);
		expect(priceMarketVars.every((v) => !v.is_historical)).toBe(true);

		const valuationVars = getStrategyVariables().filter((v) => v.category === 'Valuation Ratios');
		expect(valuationVars.map((v) => v.code)).toContain('pe');
		expect(valuationVars.map((v) => v.code)).toContain('pb');
		expect(valuationVars.every((v) => v.is_historical)).toBe(true);
	});
});
