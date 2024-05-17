import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { sessions, tracks } from '$lib/server/db/schema';
import { error, type Actions, fail } from '@sveltejs/kit';
import { message, setError, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { sessionInfoSchema } from './schema';

export const load: PageServerLoad = async ({ params }) => {
	const id = params.id;

	const session = await db.query.sessions.findFirst({
		where: eq(sessions.id, id),
		with: { league: true, sessions: true }
	});

	if (!session) {
		error(404, 'Event not found');
	}

	const tracks = await db.query.tracks.findMany();

	const form = await superValidate(session, valibot(sessionInfoSchema));

	return { event: session, form, tracks };
};

export const actions = {
	save: async ({ request, params }) => {
		// console.log(await request.formData());
		const form = await superValidate(request, valibot(sessionInfoSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		const track = await db.query.tracks.findFirst({ where: eq(tracks.id, form.data.trackId) });
		if (!track) {
			return setError(form, 'trackId', 'Track not found');
		}

		const id = params.id!; // Why would this be undefined? sveltekit idk?
		try {
			await db.update(sessions).set(form.data).where(eq(sessions.id, id));
		} catch (e) {
			return message(form, 'There was a database error', { status: 400 });
		}

		return message(form, 'Saved');
	}
} satisfies Actions;
