import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const res = await fetch('http://localhost:8000/health_check', {
		credentials: 'include',
	});
	return { health_check: res.text() };
};
