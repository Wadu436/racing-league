import { db } from '$lib/server/db/db';
import { asc, desc, eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { leagues, events as dbEvents, tracks, events } from '$lib/server/db/schema';
import { error, fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { randomUUID } from 'crypto';
import { message, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { leagueInfoSchema } from './schema';

// Define outside the load function so the adapter can be cached

export const load: PageServerLoad = async ({ params, url }) => {
	const id = params.id;

	const league = await db.query.leagues.findFirst({
		where: eq(leagues.id, id)
	});

	if (!league) {
		error(404, 'League not found');
	}

	const events = await db.query.events.findMany({
		where: eq(dbEvents.leagueId, id),
		with: { track: true },
		orderBy: [asc(dbEvents.leagueOrder)]
	});

	console.log(events);

	const form = await superValidate(league, valibot(leagueInfoSchema));

	return { league, events, form };
};

export const actions = {
	saveOrder: async ({ request }) => {
		const formData = await request.formData();

		try {
			await db.transaction(async (tx) => {
				for (const [key, value] of formData.entries()) {
					await tx
						.update(events)
						.set({ leagueOrder: Number(value) })
						.where(eq(events.id, key));
				}
			});
		} catch (e) {
			return fail(400, { reorderMessage: 'There was a database error' });
		}
	},
	orderByDate: async ({ params }) => {},
	save: async ({ request, params }) => {
		const form = await superValidate(request, valibot(leagueInfoSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		const id = params.id;
		try {
			await db
				.update(leagues)
				.set({ name: form.data.name, status: form.data.status })
				.where(eq(leagues.id, id));
		} catch (e) {
			return message(form, 'There was a database error', { status: 400 });
		}

		return message(form, 'Saved');
	}
} satisfies Actions;
