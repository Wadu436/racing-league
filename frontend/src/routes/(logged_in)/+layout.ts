import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$houdini';
import { load_ProfileQuery } from '$houdini';

export const load: LayoutLoad = async (event) => {
	const data = await event.parent();

	// TODO: redirect to a login page or something
	if (data.session == null) {
		throw redirect(307, '/');
	}

	const profileQuery = await load_ProfileQuery({ event });
	console.log(profileQuery);

	return { ...profileQuery, ...data };
};
