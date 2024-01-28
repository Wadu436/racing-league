import { lucia, setCookie } from '$lib/server/lucia';

import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import { newUsers, oauth_account, user } from '$lib/server/db/schema';

export const load = async ({ url }) => {
	const newUserKey = url.searchParams.get('new_user_key');
	const newUser =
		newUserKey && (await db.query.newUsers.findFirst({ where: eq(newUsers.id, newUserKey) }));

	if (!newUser || newUser.expirationTime < new Date()) {
		redirect(302, '/auth/signin');
	}

	return {};
};

export const actions = {
	default: async ({ url, request, cookies }) => {
		const formData = await request.formData();

		const username = formData.get('username');

		if (typeof username !== 'string' || username.length < 3 || username.length > 32) {
			return fail(400, { invalid: { username: true }, username });
		}

		const newUserKey = url.searchParams.get('new_user_key');
		const newUser =
			newUserKey && (await db.query.newUsers.findFirst({ where: eq(newUsers.id, newUserKey) }));

		if (!newUser || newUser.expirationTime < new Date()) {
			return fail(404);
		}

		try {
			const dbUser = { id: newUser.id, username: username };
			await db.transaction(async (tx) => {
				await tx.insert(user).values(dbUser);
				await tx.insert(oauth_account).values({
					providerId: newUser.providerId,
					providerUserId: newUser.providerUserId,
					userId: newUser.id
				});
				await tx.delete(newUsers).where(eq(newUsers.id, newUserKey));
			});

			const session = await lucia.createSession(dbUser.id, {});
			const sessionCookie = lucia.createSessionCookie(session.id);
			setCookie(cookies, sessionCookie);
		} catch (e) {
			console.error('Error while creating a user or session:', e);
			return fail(500);
		}

		redirect(302, '/');
	}
} satisfies Actions;
