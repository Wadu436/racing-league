import { Cookie, Lucia, type CookieAttributes } from 'lucia';
import { BetterSqlite3Adapter } from '@lucia-auth/adapter-sqlite';
import { sqliteDb } from './db/db';
import { dev } from '$app/environment';
import type { Cookies } from '@sveltejs/kit';
// import { google } from '@lucia-auth/oauth/providers';

// import { sveltekit } from 'lucia/middleware';

// export const googleAuth = google(auth, {
// 	clientId: GOOGLE_OAUTH_CLIENT_ID,
// 	clientSecret: GOOGLE_OAUTH_CLIENT_SECRET,
// 	redirectUri: GOOGLE_OAUTH_REDIRECT_URI
// });

// export type Auth = typeof auth;

const adapter = new BetterSqlite3Adapter(sqliteDb, { user: 'user', session: 'session' });

export const lucia = new Lucia(adapter, {
	sessionCookie: {
		attributes: {
			secure: !dev
		}
	},
	getUserAttributes(databaseUser) {
		return {
			username: databaseUser.username
		};
	}
});

declare module 'lucia' {
	interface Register {
		Lucia: typeof lucia;
		DatabaseSessionAttributes: DatabaseSessionAttributes;
		DatabaseUserAttributes: DatabaseUserAttributes;
	}

	type DatabaseSessionAttributes = Record<string, never>;

	type DatabaseUserAttributes = {
		username: string;
	};
}

export const setCookie = (cookies: Cookies, luciaCookie: Cookie) => {
	luciaCookie.attributes.path = luciaCookie.attributes.path ?? '/';
	cookies.set(
		luciaCookie.name,
		luciaCookie.value,
		luciaCookie.attributes as CookieAttributes & { path: string }
	);
};
