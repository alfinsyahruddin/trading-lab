import { redirect } from '@sveltejs/kit';
import { getToken } from '#lib/helpers/session.js';

// Protect all /dashboard/* routes — redirect to login if no token in localStorage.
export const load = () => {
	const token = getToken();
	if (!token) {
		throw redirect(302, '/login');
	}
};
