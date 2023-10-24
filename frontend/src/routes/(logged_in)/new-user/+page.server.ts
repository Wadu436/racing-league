import { graphql } from '$houdini';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { uploadProfilePicture } from '$lib/server/profilePicture';

export const actions: Actions = {
	createUser: async (event) => {
		const data = await event.request.formData();

		console.log('data', data);

		const username = data.get('username')?.toString();
		const steamId = data.get('steam-id')?.toString();
		const eaId = data.get('ea-id')?.toString();
		const profilePicture = data.get('profile-picture');

		if (!username) {
			return fail(400, { usernameMissing: true });
		}

		console.log('profilePicture', profilePicture);
		console.log('profilePicture', profilePicture);

		let profilePicturePath: string | undefined;
		if (profilePicture && profilePicture instanceof Blob && profilePicture.size > 0) {
			profilePicturePath = await uploadProfilePicture(profilePicture);
		}

		const newUserMutation = graphql(`
			mutation NewUserMutation(
				$username: String!
				$steamId: String
				$eaId: String
				$profilePicturePath: String
			) {
				signup(
					data: {
						username: $username
						steamId: $steamId
						eaId: $eaId
						profilePicturePath: $profilePicturePath
					}
				) {
					id
				}
			}
		`);

		const response = await newUserMutation.mutate(
			{ username, steamId, eaId, profilePicturePath },
			{ event }
		);

		if (response.errors == null) {
			throw redirect(307, '/');
		}

		return response;
	}
};
