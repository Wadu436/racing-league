import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { error, fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, url }) => {
	const create = url.searchParams.get('create') != undefined;
	const data = { tracks: await db.query.tracks.findMany(), create };

	if (!create && params.id) {
		const dbTrack = await db.query.tracks.findFirst({ where: eq(tracks.id, params.id) });
		if (!dbTrack) {
			error(404);
		}
		return {
			dbTrack,
			...data
		};
	}

	return data;
};

import type { Actions } from './$types';
import { randomUUID } from 'crypto';
import { tracks } from '$lib/server/db/schema';

async function save(id: string, formData: FormData) {
	const name = formData.get('name')?.toString();
	if (!name) {
		return fail(400, { message: 'Name is required' });
	}

	const country = formData.get('country')?.toString();
	if (!country) {
		return fail(400, { message: 'Country is required ' });
	}

	const result = await db.update(tracks).set({ name, country }).where(eq(tracks.id, id));

	if (result.changes === 0) {
		return fail(400, { message: "Team wasn't found in the database" });
	}

	return;
}

async function create(formData: FormData, addAnother: boolean) {
	const name = formData.get('name')?.toString();
	if (!name) {
		return fail(400, { message: 'Name is required' });
	}

	const country = formData.get('country')?.toString();
	if (!country) {
		return fail(400, { message: 'Country is required ' });
	}

	const id = randomUUID();

	try {
		await db.insert(tracks).values({
			id,
			name,
			country,
		});
	} catch (e) {
		return fail(400, { message: 'There was a database error' });
	}

	if (!addAnother) {
		console.log("Redirect");
		redirect(302, `/admin/tracks/${id}`);
	}
}

export const actions = {
	save: async ({ request, params }) => {
		if (params.id) {
			const formData = await request.formData();

			return await save(params.id, formData);
		}
	},
	create: async ({ request }) => {
		console.log("Create");
		const formData = await request.formData();

		return await create(formData, false);
	},
	create_another: async ({ request }) => {
		console.log("Create Another");
		const formData = await request.formData();

		return await create(formData, true);
	}
} satisfies Actions;
