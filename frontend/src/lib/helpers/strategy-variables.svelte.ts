import { getStrategyVariables as fetchApiVariables } from '$lib/api';
import type { StrategyMetadata, StrategyOperator, StrategyVariable } from '$lib/types';

let metadata = $state<StrategyMetadata>({
	variables: [],
	categories: [],
	operators: []
});
let loaded = $state(false);
let loadingPromise: Promise<StrategyMetadata> | null = null;

/**
 * Fetches strategy metadata (variables, categories, operators) from the backend API.
 * Deduplicates in-flight requests and caches the result in reactive state.
 */
export async function fetchStrategyMetadata(token?: string): Promise<StrategyMetadata> {
	if (loaded) {
		return metadata;
	}
	if (loadingPromise) {
		return loadingPromise;
	}

	loadingPromise = (async () => {
		try {
			const res = await fetchApiVariables(token);
			if (res) {
				metadata = res;
				loaded = true;
			}
			return metadata;
		} finally {
			loadingPromise = null;
		}
	})();

	return loadingPromise;
}

/**
 * Gets the current strategy metadata state.
 */
export function getStrategyMetadata(): StrategyMetadata {
	return metadata;
}

/**
 * Gets the reactive list of all strategy variables.
 */
export function getStrategyVariables(): StrategyVariable[] {
	return metadata.variables;
}

/**
 * Gets the reactive list of variable categories.
 */
export function getVariableCategories(): string[] {
	return metadata.categories;
}

/**
 * Gets the reactive list of strategy rule operators.
 */
export function getStrategyOperators(): StrategyOperator[] {
	return metadata.operators;
}

/**
 * Finds a strategy variable by code (case-insensitive).
 */
export function findStrategyVariable(code: string): StrategyVariable | undefined {
	if (!code) return undefined;
	const lower = code.toLowerCase();
	return metadata.variables.find((v) => v.code.toLowerCase() === lower);
}

/**
 * Finds a strategy operator by value (e.g. '=', '>', '<=').
 */
export function findStrategyOperator(value: string): StrategyOperator | undefined {
	if (!value) return undefined;
	return metadata.operators.find((o) => o.value === value);
}

/**
 * Sets metadata directly (useful for tests or preloading).
 */
export function setStrategyMetadata(data: StrategyMetadata): void {
	metadata = data;
	loaded = true;
}

/**
 * Resets metadata state to initial empty values (useful for test isolation).
 */
export function resetStrategyMetadata(): void {
	metadata = {
		variables: [],
		categories: [],
		operators: []
	};
	loaded = false;
	loadingPromise = null;
}
