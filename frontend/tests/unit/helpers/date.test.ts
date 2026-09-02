import { describe, it, expect } from 'vitest';
import {
	calculateBacktestDateRange,
	formatDateShort,
	formatBacktestDateRange
} from '$lib/helpers/date';

describe('date helper', () => {
	it('calculates 1 month date range for screener year 2025 (matching backend)', () => {
		const result = calculateBacktestDateRange(2025, 1);
		expect(result.startDate.getFullYear()).toBe(2026);
		expect(result.startDate.getMonth()).toBe(0); // Jan
		expect(result.startDate.getDate()).toBe(1);

		expect(result.endDate.getFullYear()).toBe(2026);
		expect(result.endDate.getMonth()).toBe(0); // Jan
		expect(result.endDate.getDate()).toBe(31);

		expect(result.formatted).toBe('1 Jan 2026 - 31 Jan 2026');
	});

	it('calculates 3 months date range for screener year 2025 (matching backend)', () => {
		const result = calculateBacktestDateRange(2025, 3);
		expect(result.startDate.getFullYear()).toBe(2026);
		expect(result.startDate.getMonth()).toBe(0);
		expect(result.startDate.getDate()).toBe(1);

		expect(result.endDate.getFullYear()).toBe(2026);
		expect(result.endDate.getMonth()).toBe(2); // Mar
		expect(result.endDate.getDate()).toBe(31);

		expect(result.formatted).toBe('1 Jan 2026 - 31 Mar 2026');
	});

	it('calculates 6 months date range for screener year 2025 (matching backend)', () => {
		const result = calculateBacktestDateRange(2025, 6);
		expect(result.startDate.getFullYear()).toBe(2026);
		expect(result.startDate.getMonth()).toBe(0);
		expect(result.startDate.getDate()).toBe(1);

		expect(result.endDate.getFullYear()).toBe(2026);
		expect(result.endDate.getMonth()).toBe(5); // Jun
		expect(result.endDate.getDate()).toBe(30);

		expect(result.formatted).toBe('1 Jan 2026 - 30 Jun 2026');
	});

	it('calculates 12 months date range for screener year 2025 (matching backend)', () => {
		const result = calculateBacktestDateRange(2025, 12);
		expect(result.startDate.getFullYear()).toBe(2026);
		expect(result.startDate.getMonth()).toBe(0);
		expect(result.startDate.getDate()).toBe(1);

		expect(result.endDate.getFullYear()).toBe(2026);
		expect(result.endDate.getMonth()).toBe(11); // Dec
		expect(result.endDate.getDate()).toBe(31);

		expect(result.formatted).toBe('1 Jan 2026 - 31 Dec 2026');
	});

	it('calculates leap year February correctly (screener year 2023 -> 2024)', () => {
		const result = calculateBacktestDateRange(2023, 2);
		expect(result.startDate.getFullYear()).toBe(2024);
		expect(result.startDate.getMonth()).toBe(0);
		expect(result.startDate.getDate()).toBe(1);

		expect(result.endDate.getFullYear()).toBe(2024);
		expect(result.endDate.getMonth()).toBe(1); // Feb
		expect(result.endDate.getDate()).toBe(29); // Leap day

		expect(result.formatted).toBe('1 Jan 2024 - 29 Feb 2024');
	});

	it('calculates non-leap year February correctly (screener year 2022 -> 2023)', () => {
		const result = calculateBacktestDateRange(2022, 2);
		expect(result.startDate.getFullYear()).toBe(2023);
		expect(result.startDate.getMonth()).toBe(0);
		expect(result.startDate.getDate()).toBe(1);

		expect(result.endDate.getFullYear()).toBe(2023);
		expect(result.endDate.getMonth()).toBe(1); // Feb
		expect(result.endDate.getDate()).toBe(28);

		expect(result.formatted).toBe('1 Jan 2023 - 28 Feb 2023');
	});

	it('formats individual date with formatDateShort', () => {
		const d = new Date(2026, 0, 1);
		expect(formatDateShort(d)).toBe('1 Jan 2026');
	});

	it('formats date range with formatBacktestDateRange', () => {
		const d1 = new Date(2026, 0, 1);
		const d2 = new Date(2026, 4, 31);
		expect(formatBacktestDateRange(d1, d2)).toBe('1 Jan 2026 - 31 May 2026');
	});
});
