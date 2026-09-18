/**
 * Calculates the backtest start and end dates based on screener data year and duration in months.
 *
 * Matching backend logic:
 * The simulation trades on the year following the screener data year (`year + 1`),
 * starting on January 1st and ending at the last day of the N-th month.
 */
export function calculateBacktestDateRange(
	year: number,
	durationMonths: number,
	maxDate: Date = new Date()
): { startDate: Date; endDate: Date; formatted: string; days: number } {
	const safeYear = Number(year) || 2025;
	const safeMonths = Math.max(1, Number(durationMonths) || 12);

	const startYear = safeYear + 1;
	const startDate = new Date(startYear, 0, 1);

	// Last day of the N-th month is day 0 of month index (safeMonths)
	let endDate = new Date(startYear, safeMonths, 0);

	const maxD = new Date(maxDate.getFullYear(), maxDate.getMonth(), maxDate.getDate());
	if (endDate > maxD) {
		endDate = maxD >= startDate ? maxD : startDate;
	}

	const utcStart = Date.UTC(startDate.getFullYear(), startDate.getMonth(), startDate.getDate());
	const utcEnd = Date.UTC(endDate.getFullYear(), endDate.getMonth(), endDate.getDate());
	const days = Math.round((utcEnd - utcStart) / 86_400_000) + 1;

	return {
		startDate,
		endDate,
		formatted: formatBacktestDateRange(startDate, endDate),
		days
	};
}

/**
 * Formats a Date instance as "D MMM YYYY" (e.g. "1 Jan 2026", "31 Jan 2026").
 */
export function formatDateShort(d: Date): string {
	const day = d.toLocaleDateString('en-US', { day: 'numeric' });
	const month = d.toLocaleDateString('en-US', { month: 'short' });
	const y = d.toLocaleDateString('en-US', { year: 'numeric' });
	return `${day} ${month} ${y}`;
}

/**
 * Formats a date range into "D MMM YYYY - D MMM YYYY" (e.g. "1 Jan 2026 - 31 Jan 2026").
 */
export function formatBacktestDateRange(startDate: Date, endDate: Date): string {
	return `${formatDateShort(startDate)} - ${formatDateShort(endDate)}`;
}

/**
 * Formats a Date instance as "YYYY-MM-DD" in local time.
 */
export function formatDateISO(d: Date): string {
	const year = d.getFullYear();
	const month = String(d.getMonth() + 1).padStart(2, '0');
	const day = String(d.getDate()).padStart(2, '0');
	return `${year}-${month}-${day}`;
}

/**
 * Calculates inclusive calendar days between two dates.
 */
export function calculateDaysBetween(startDate: Date, endDate: Date): number {
	const utcStart = Date.UTC(startDate.getFullYear(), startDate.getMonth(), startDate.getDate());
	const utcEnd = Date.UTC(endDate.getFullYear(), endDate.getMonth(), endDate.getDate());
	return Math.max(1, Math.round((utcEnd - utcStart) / 86_400_000) + 1);
}
