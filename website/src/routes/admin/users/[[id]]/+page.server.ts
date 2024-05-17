import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { users } from '$lib/server/db/schema';
import { error, fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { randomUUID } from 'crypto';
import { message, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { formSchema } from './schema';

// Define outside the load function so the adapter can be cached

export const load: PageServerLoad = async ({ params, url }) => {
	const create = url.searchParams.get('create') != undefined;
	const data = { users: await db.query.users.findMany(), create };

	if (params.id) {
		const dbTeam = await db.query.users.findFirst({ where: eq(users.id, params.id) });
		if (!dbTeam) {
			error(404);
		}

		const form = await superValidate(dbTeam, valibot(formSchema));

		return {
			form,
			...data
		};
	} else if (create) {
		const form = await superValidate(valibot(formSchema));
		return {
			form,
			...data
		};
	} else {
		return data;
	}
};

export const actions = {
	save: async ({ request, params }) => {
		const form = await superValidate(request, valibot(formSchema));

		if (!form.valid) {
			// Again, return { form } and things will just work.
			return fail(400, { form });
		}

		console.log('form', form);

		const id = params.id;

		if (id) {
			// Save the entry
			// TODO: Do something with the validated form.data
			const result = await db
				.update(users)
				.set({ username: form.data.username, admin: form.data.admin, staff: form.data.staff })
				.where(eq(users.id, id));

			if (result.changes === 0) {
				return message(form, "User wasn't found in the database", { status: 404 });
			}
		} else {
			// Create an entry
			const id = randomUUID();

			try {
				await db.insert(users).values({
					id,
					username: form.data.username,
					admin: form.data.admin,
					staff: form.data.staff
				});
			} catch (e) {
				return fail(400, { message: 'There was a database error' });
			}
		}

		// Yep, return { form } here too
		return message(form, 'Saved');
	}
	// delete: async ({ params }) => {
	// 	const id = params.id;
	// 	if (!id) {
	// 		return fail(404);
	// 	}

	// 	await db.delete(users).where(eq(users.id, id));

	// 	redirect(307, '/admin/drivers');
	// }
} satisfies Actions;
