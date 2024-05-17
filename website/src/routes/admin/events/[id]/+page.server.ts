import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { events, tracks } from '$lib/server/db/schema';
import { error, type Actions, fail } from '@sveltejs/kit';
import { message, setError, superValidate } from 'sveltekit-superforms';
import { valibot } from 'sveltekit-superforms/adapters';
import { eventInfoSchema } from './schema';

export const load: PageServerLoad = async ({ params }) => {
	const id = params.id;

	const event = await db.query.events.findFirst({
		where: eq(events.id, id),
		with: { league: true, sessions: true }
	});

	if (!event) {
		error(404, 'Event not found');
	}

	const tracks = await db.query.tracks.findMany();

	const form = await superValidate(event, valibot(eventInfoSchema));

	return { event, form, tracks };
};

export const actions = {
	save: async ({ request, params }) => {
		// console.log(await request.formData());
		const form = await superValidate(request, valibot(eventInfoSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		const track = await db.query.tracks.findFirst({ where: eq(tracks.id, form.data.trackId) });
		if (!track) {
			return setError(form, 'trackId', 'Track not found');
		}

		const id = params.id!; // Why would this be undefined? sveltekit idk?
		try {
			await db.update(events).set(form.data).where(eq(events.id, id));
		} catch (e) {
			return message(form, 'There was a database error', { status: 400 });
		}

		return message(form, 'Saved');
	}
} satisfies Actions;
