import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { teams } from '$lib/server/db/schema';
import { error, fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { randomUUID } from 'crypto';
import { message, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { formSchema } from './schema';

// Define outside the load function so the adapter can be cached

export const load: PageServerLoad = async ({ params, url }) => {
	const create = url.searchParams.get('create') != undefined;
	const data = { teams: await db.query.teams.findMany(), create };

	if (params.id) {
		const dbTeam = await db.query.teams.findFirst({ where: eq(teams.id, params.id) });
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

		const id = params.id;

		if (id) {
			// Save the entry
			// TODO: Do something with the validated form.data
			const result = await db
				.update(teams)
				.set({ name: form.data.name, country: form.data.country })
				.where(eq(teams.id, id));

			if (result.changes === 0) {
				return message(form, "Team wasn't found in the database", { status: 404 });
			}
		} else {
			// Create an entry
			const id = randomUUID();

			try {
				await db.insert(teams).values({ id, name: form.data.name, country: form.data.country });
			} catch (e) {
				return fail(400, { message: 'There was a database error' });
			}
		}

		// Yep, return { form } here too
		return message(form, 'Saved');
	},
	delete: async ({ params }) => {
		const id = params.id;
		if (!id) {
			return fail(404);
		}

		await db.delete(teams).where(eq(teams.id, id));

		redirect(307, '/admin/teams');
	}
} satisfies Actions;
