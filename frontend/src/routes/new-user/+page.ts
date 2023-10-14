import { load_NewUserQuery } from '$houdini';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$houdini';

export const load: PageLoad = async (event) => {
	const newUserQuery = await load_NewUserQuery({ event });

	if (newUserQuery.NewUserQuery.observer.state.data?.me?.sub) {
		throw redirect(302, event.url.searchParams.get('redirect') ?? '/');
	}

	return {
		...newUserQuery
	};
};
