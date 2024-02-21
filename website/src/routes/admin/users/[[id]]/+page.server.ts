import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { users } from '$lib/server/db/schema';
import { error, fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
	const data = { users: await db.query.users.findMany() };

	if (params.id) {
		const dbUser = await db.query.users.findFirst({ where: eq(users.id, params.id) });
		if (!dbUser) {
			error(404);
		}
		return {
			dbUser,
			...data
		};
	}

	return data;
};

import type { Actions } from './$types';

export const actions = {
	save: async ({ request, params, locals }) => {
		if(!locals.user) {
			return fail(401);
		}
		if(!params.id) {
			return fail(404);
		}
		const existingUser = await db.query.users.findFirst({ where: eq(users.id, params.id) });
		if(!existingUser) {
			return fail(404);
		}
		if (existingUser) {
			const formData = await request.formData();

			const username = formData.get('username')?.toString();
			if (!username) {
				return fail(400, { message: 'Username is required' });
			}

			const admin = formData.get('admin') === 'on';
			const staff = formData.get('staff') === 'on';

			// Check if this user is allowed to make this edit
			locals.user.admin;

			const result = await db
				.update(users)
				.set({ username, admin, staff })
				.where(eq(users.id, params.id));

			if (result.changes === 0) {
				return fail(400, { message: "User wasn't found in the database" });
			}

			return;
		}
	}
} satisfies Actions;
