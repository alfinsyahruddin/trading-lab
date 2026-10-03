import { redirect } from '@sveltejs/kit';
import { getToken } from '#lib/helpers/session.js';

// Redirect already-authenticated users away from register.
export const load = () => {
	const token = getToken();
	if (token) {
		throw redirect(302, '/dashboard');
	}
};
