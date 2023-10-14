import { fail, type Actions } from '@sveltejs/kit';
import { graphql } from '$houdini';

export const actions: Actions = {
	update: async (event) => {
		const data = await event.request.formData();

		const username = data.get('username')?.toString();
		const steamId = data.get('steamId')?.toString();
		const eaId = data.get('eaId')?.toString();

		if (!username) {
			return fail(400, { usernameMissing: true });
		}

		const updateUserMutation = graphql(`
			mutation UpdateUserMutation($username: String!, $steamId: String, $eaId: String) {
				updateUser(data: { username: $username, steamId: $steamId, eaId: $eaId })
			}
		`);

		const response = await updateUserMutation.mutate({ username, steamId, eaId }, { event });

		if (response.errors == null) {
			return { success: true };
		} else {
			return fail(400, { serverErrors: response.errors });
		}
	}
};
