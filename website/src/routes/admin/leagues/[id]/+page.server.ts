import { db } from '$lib/server/db/db';
import { asc, eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { leagues, events as dbEvents, events, games } from '$lib/server/db/schema';
import { error, fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import { message, setError, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { leagueInfoSchema } from './schema';

// Define outside the load function so the adapter can be cached

export const load: PageServerLoad = async ({ params }) => {
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

	const games = await db.query.games.findMany();

	const form = await superValidate(league, valibot(leagueInfoSchema));

	return { league, events, form, games };
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
	save: async ({ request, params }) => {
		const form = await superValidate(request, valibot(leagueInfoSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		// Check if the game exists
		if (form.data.gameId) {
			const game = await db.query.games.findFirst({ where: eq(games.id, form.data.gameId) });
			console.log(game);
			if (!game) {
				return setError(form, 'gameId', 'Game not found');
			}
		}

		const id = params.id;
		try {
			await db
				.update(leagues)
				.set({ name: form.data.name, status: form.data.status, gameId: form.data.gameId })
				.where(eq(leagues.id, id));
		} catch (e) {
			return message(form, 'There was a database error', { status: 400 });
		}

		return message(form, 'Saved');
	}
} satisfies Actions;
