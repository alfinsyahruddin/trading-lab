import { redirect } from '@sveltejs/kit';
import { getToken } from '$lib/helpers/session';

// If the user is already logged in, skip the index page and go straight to the dashboard.
export const load = () => {
	const token = getToken();
	if (token) {
		throw redirect(302, '/dashboard');
	}
};
