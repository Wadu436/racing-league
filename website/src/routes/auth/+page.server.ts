import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { lucia } from '$lib/server/lucia';
import { db } from '$lib/server/db/db';
import * as schema from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

export const actions = {
	signout: async ({ locals }) => {
		const session = locals.session;
		if (!session) {
			return fail(401);
		}

		lucia.invalidateSession(session.id);
		locals.session = undefined;
		locals.user = undefined;
		redirect(302, '/');
	},
	delete: async ({ locals }) => {
		const session = locals.session;
		if (!session) {
			return fail(401);
		}

		lucia.invalidateUserSessions(session.userId);

		await db.transaction(async (tx) => {
			await tx.delete(schema.sessions).where(eq(schema.sessions.id, session.id));
			await tx.delete(schema.oauthAccounts).where(eq(schema.oauthAccounts.userId, session.userId));
			await tx.delete(schema.users).where(eq(schema.users.id, session.userId));
		});
		locals.session = undefined;
		locals.user = undefined;
		redirect(302, '/');
	}
} satisfies Actions;
