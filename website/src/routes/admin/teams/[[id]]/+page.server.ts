import { db } from '$lib/server/db/db';
import { eq } from 'drizzle-orm';
import type { PageServerLoad } from './$types';
import { teams } from '$lib/server/db/schema';
import { error, fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, url }) => {
	const create = url.searchParams.get('create') != undefined;
	const data = { teams: await db.query.teams.findMany(), create };

	if (!create && params.id) {
		const dbTeam = await db.query.teams.findFirst({ where: eq(teams.id, params.id) });
		if (!dbTeam) {
			error(404);
		}
		return {
			dbTeam,
			...data
		};
	}

	return data;
};

import type { Actions } from './$types';
import { randomUUID } from 'crypto';

async function save(id: string, formData: FormData) {
	const name = formData.get('name')?.toString();
	if (!name) {
		return fail(400, { message: 'Name is required' });
	}

	const country = formData.get('country')?.toString();
	if (!country) {
		return fail(400, { message: 'Country is required ' });
	}

	const result = await db.update(teams).set({ name, country }).where(eq(teams.id, id));

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
		await db.insert(teams).values({
			id,
			country,
			name
		});
	} catch (e) {
		console.error(e);
		return fail(400, { message: 'There was a database error' });
	}

	if (!addAnother) {
		redirect(302, `/admin/teams/${id}`);
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
		const formData = await request.formData();

		return await create(formData, false);
	},
	create_another: async ({ request }) => {
		const formData = await request.formData();

		return await create(formData, true);
	}
} satisfies Actions;
