import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$houdini';

export const load: PageLoad = async ({ parent, url }) => {
	const data = await parent();

	if (data.ProfileQuery.observer.state.data?.me != null) {
		redirect(307, url.searchParams.get('redirect') ?? '/');
	}

	return data;
};
