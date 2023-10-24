import { fail, type Actions, redirect } from '@sveltejs/kit';
import { graphql } from '$houdini';
import { uploadProfilePicture } from '$lib/server/profilePicture';

export const actions: Actions = {
	update: async (event) => {
		const data = await event.request.formData();

		console.log('data', data);

		const username = data.get('username')?.toString();
		const steamId = data.get('steam-id')?.toString();
		const eaId = data.get('ea-id')?.toString();
		const profilePicture = data.get('profile-picture');

		if (!username) {
			return fail(400, { usernameMissing: true });
		}

		let profilePicturePath: string | undefined;
		console.log('profilePicture Edit', profilePicture);
		console.log('profilePicture Edit', typeof profilePicture);
		if (profilePicture && profilePicture instanceof File && profilePicture.size > 0) {
			profilePicturePath = await uploadProfilePicture(profilePicture);
		}

		console.log('profilePicturePath', profilePicturePath);

		const updateUserMutation = graphql(`
			mutation UpdateUserMutation(
				$username: String!
				$steamId: String
				$eaId: String
				$profilePicturePath: String
			) {
				updateUser(
					data: {
						username: $username
						steamId: $steamId
						eaId: $eaId
						profilePicturePath: $profilePicturePath
						deleteProfilePicture: false
					}
				) {
					id
				}
			}
		`);

		const response = await updateUserMutation.mutate(
			{ username, steamId, eaId, profilePicturePath },
			{ event }
		);

		if (response.errors == null) {
			throw redirect(303, '/profile');
			return { success: true };
		} else {
			return fail(400, { serverErrors: response.errors });
		}
	}
};
