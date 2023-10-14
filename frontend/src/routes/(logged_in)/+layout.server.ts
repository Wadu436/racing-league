import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ parent }) => {
	const { session } = await parent();

    // TODO: redirect to a login page or something
	if (session == null) {
		throw redirect(307, '/');
	}
};
