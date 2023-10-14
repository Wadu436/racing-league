import {
	AUTH0_CLIENT_SECRET,
	AUTH0_CLIENT_ID,
	AUTH0_ISSUER,
	AUTH_SECRET
} from '$env/static/private';

import { SvelteKitAuth, type SvelteKitAuthConfig } from '@auth/sveltekit';
import Auth0Provider from '@auth/core/providers/auth0';
import type { Provider } from '@auth/core/providers';
import type { Handle } from '@sveltejs/kit';
import type { Session as AuthSession } from '@auth/core/types';
import { sequence } from '@sveltejs/kit/hooks';
import { getToken } from '@auth/core/jwt';

const config: SvelteKitAuthConfig = {
	providers: [
		Auth0Provider({
			clientId: AUTH0_CLIENT_ID,
			clientSecret: AUTH0_CLIENT_SECRET,
			issuer: AUTH0_ISSUER, // <- remember to add trailing `/`
			wellKnown: `${AUTH0_ISSUER}.well-known/openid-configuration`,
			token: {
				params: {
					audience: 'f1.warre.dev/api/'
				}
			},
			authorization: {
				params: {
					audience: encodeURI('f1.warre.dev/api/')
				}
			}
		}) as Provider
	],
	secret: AUTH_SECRET,
	debug: true,
	session: {
		maxAge: 1800 // 30 mins
	},
	callbacks: {
		async jwt({ token, account }) {
			if (account) {
				token.accessToken = account.access_token;
			}

			return token;
		},
	}
};

const accessTokenCookieHandle: Handle = async ({ event, resolve }) => {
	const cookies = event.cookies.getAll().reduce((acc, cookie) => {
		acc[cookie.name] = cookie.value;
		return acc;
	}, {} as Record<string, string>);

	const token = await getToken({ req: { cookies, headers: {} }, secret: AUTH_SECRET });

	console.log('token', token)

	if (token?.accessToken) {
		event.cookies.set('f1_warre_dev_access_token', token.accessToken as string, { path: '/', sameSite: 'lax'});
	} else {
		event.cookies.delete('f1_warre_dev_access_token', { path: '/', sameSite: 'lax'});
	}

	return await resolve(event);
};

export const handle: Handle = sequence(SvelteKitAuth(config), accessTokenCookieHandle);

// Workaround for a Houdini thing
declare global {
	// eslint-disable-next-line @typescript-eslint/no-namespace
	namespace App {
		interface Session extends AuthSession {}
	}
}
