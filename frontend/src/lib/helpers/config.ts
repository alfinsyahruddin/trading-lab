/**
 * Application environment configuration helpers.
 */

/**
 * Check if the application is in "Coming Soon" mode.
 * Checks the PUBLIC_IS_COMING_SOON environment variable.
 * Defaults to true if set or fallback.
 */
export function isComingSoon(): boolean {
	const val = import.meta.env.PUBLIC_IS_COMING_SOON;
	if (val === undefined || val === null || val === '') {
		return true;
	}
	return String(val).toLowerCase() === 'true' || val === true || val === '1';
}
