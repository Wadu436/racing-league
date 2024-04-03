import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { leagues, games } from '$lib/server/db/schema';
import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import { randomUUID } from 'crypto';
import { message, setError, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { newLeagueSchema } from './schema';
import { capitalize } from '$lib/util';

export const load: PageServerLoad = async () => {
	const form = await superValidate(valibot(newLeagueSchema));
	const leagues = (await db.query.leagues.findMany({ with: { game: true } })).map((league) => ({
		...league,
		status: capitalize(league.status)
	}));
	const games = await db.query.games.findMany();

	return {
		leagues,
		games,
		form
	};
};

export const actions = {
	create: async ({ request }) => {
		const form = await superValidate(request, valibot(newLeagueSchema));

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

		const id = randomUUID();
		try {
			await db
				.insert(leagues)
				.values({ id, name: form.data.name, status: 'upcoming', gameId: form.data.gameId });
		} catch (e) {
			return message(form, 'There was a database error', { status: 400 });
		}

		return message(form, 'Saved');
	}
} satisfies Actions;
