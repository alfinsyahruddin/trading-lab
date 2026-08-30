import { redirect } from '@sveltejs/kit';
import { getToken } from '$lib/helpers/session';

// Redirect already-authenticated users away from login.
export const load = () => {
	const token = getToken();
	if (token) {
		throw redirect(302, '/dashboard');
	}
};
