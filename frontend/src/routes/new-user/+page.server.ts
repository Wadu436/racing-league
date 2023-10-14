import { graphql } from '$houdini';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions: Actions = {
	createUser: async (event) => {
		const data = await event.request.formData();

		const username = data.get('username')?.toString();
		const steamId = data.get('steamId')?.toString();
		const eaId = data.get('eaId')?.toString();

		if (!username) {
			return fail(400, { usernameMissing: true });
		}

		const newUserMutation = graphql(`
			mutation NewUserMutation($username: String!, $steamId: String, $eaId: String) {
				signup(data: { username: $username, steamId: $steamId, eaId: $eaId })
			}
		`);

		const response = await newUserMutation.mutate({ username, steamId, eaId }, { event });

        if (response.errors == null) {
            throw redirect(303, '/');
        }

		return response;
	}
};
