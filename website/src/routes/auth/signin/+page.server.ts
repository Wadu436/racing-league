import { redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import type { OauthStateCookie } from '../callback/+server';
import { dev } from '$app/environment';
import { googleAuth } from '$lib/server/oauth';
import { generateCodeVerifier, generateState } from 'arctic';

export const actions = {
	google: async ({ request, cookies }) => {
		const formData = await request.formData();
		console.log(`formData.get("next")`, formData.get('next'));
		const next = formData.get('next')?.toString();

		const state = generateState();
		const codeVerifier = generateCodeVerifier();
		const oauthUrl = await googleAuth.createAuthorizationURL(state, codeVerifier);

		console.log('next:', next);

		const cookie: OauthStateCookie = { state, codeVerifier, provider: 'google' };
		if (next != null) {
			cookie.next = next;
		}

		cookies.set('oauth-state', btoa(JSON.stringify(cookie)), {
			httpOnly: true,
			secure: !dev,
			path: '/',
			maxAge: 60 * 60
		});

		redirect(302, oauthUrl);
	}
} satisfies Actions;

export const load = (async ({ url, locals }) => {
	if (locals.session) {
		return redirect(302, '/');
	}
	// Store state in a cookie
	const next = url.searchParams.get('next');
	console.log('next:', next);
	return { next };
}) satisfies PageServerLoad;
