import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { leagues, events } from '$lib/server/db/schema';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { randomUUID } from 'crypto';
import { message, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { newLeagueSchema } from './schema';
import { capitalize } from '$lib/util';

// Define outside the load function so the adapter can be cached

export const load: PageServerLoad = async () => {
	const form = await superValidate(valibot(newLeagueSchema));
	const leagues = (await db.query.leagues.findMany()).map((league) => ({
		...league,
		status: capitalize(league.status)
	}));
	return {
		leagues,
		form
	};
};

export const actions = {
	create: async ({ request }) => {
		const form = await superValidate(request, valibot(newLeagueSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		const id = randomUUID();
		try {
			await db.insert(leagues).values({ id, name: form.data.name, status: 'upcoming' });
		} catch (e) {
			return message(form, 'There was a database error', { status: 400 });
		}

		return message(form, 'Saved');
	}
} satisfies Actions;
