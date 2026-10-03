export const APP_NAME = 'Trading Lab';
export const LS_TOKEN = 'trading_lab_token';
export const LS_REFRESH = 'trading_lab_refresh_token';
export const LS_USER = 'trading_lab_user';
export const LS_THEME = 'trading_lab_theme';
export const LS_REMEMBERED_ACCOUNTS = 'trading_lab_remembered_accounts';

export function formatRiskReward(tp: number, sl: number): string {
	if (sl <= 0 || isNaN(sl) || isNaN(tp)) return '—';
	const ratio = tp / sl;
	const formattedRatio = Number(ratio.toFixed(2));
	return `1 : ${formattedRatio}`;
}

export function formatTimeAgo(isoString: string): string {
	if (!isoString) return '—';
	const date = new Date(isoString);
	if (isNaN(date.getTime())) return '—';
	const now = new Date();
	const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

	if (diffInSeconds < 60) {
		return 'just now';
	}
	const minutes = Math.floor(diffInSeconds / 60);
	if (minutes < 60) {
		return `${minutes} ${minutes === 1 ? 'minute' : 'minutes'} ago`;
	}
	const hours = Math.floor(minutes / 60);
	if (hours < 24) {
		return `${hours} ${hours === 1 ? 'hour' : 'hours'} ago`;
	}
	const days = Math.floor(hours / 24);
	if (days < 30) {
		return `${days} ${days === 1 ? 'day' : 'days'} ago`;
	}
	const months = Math.floor(days / 30);
	if (months < 12) {
		return `${months} ${months === 1 ? 'month' : 'months'} ago`;
	}
	const years = Math.floor(days / 365);
	return `${years} ${years === 1 ? 'year' : 'years'} ago`;
}
