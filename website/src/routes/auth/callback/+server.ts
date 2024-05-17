import { db } from '$lib/server/db/db.js';
import { newUsers, oauthAccounts } from '$lib/server/db/schema.js';
import { lucia, setCookie } from '$lib/server/lucia.js';
import { googleAuth } from '$lib/server/oauth.js';
import { error, redirect } from '@sveltejs/kit';
import { OAuth2RequestError } from 'arctic';
import { parseJWT } from 'oslo/jwt';
import { eq, and } from 'drizzle-orm';
import { object, string, parse } from 'valibot';
import { generateId } from 'lucia';

export type OauthStateCookie = {
	state: string;
	provider: string;
	codeVerifier: string;
	next?: string;
};

const GoogleUserSchema = object({
	sub: string()
});

// type GoogleUser = Output<typeof GoogleUserSchema>;

export const GET = async ({ url, cookies }) => {
	const cookieStateJson = cookies.get('oauth-state');
	if (!cookieStateJson) {
		error(400, "Invalid OAuth callback (no state cookie found)");
	}
	const cookieState: OauthStateCookie = JSON.parse(atob(cookieStateJson));

	const state = url.searchParams.get('state');
	const code = url.searchParams.get('code');

	if (!cookieState || !state || cookieState.state !== state || !code) {
		error(400, "Invalid OAuth callback (invalid state)");
	}

	// If cookieState.next is undefind, null, or empty string, redirect to the home page
	let redirect_to = cookieState.next ? cookieState.next : '/';
	try {
		const googleTokens = await googleAuth.validateAuthorizationCode(code, cookieState.codeVerifier);
		const googleUser = parse(GoogleUserSchema, parseJWT(googleTokens.idToken)!.payload);

		// Check if the user already exists in the database
		const existingUser = await db.query.oauthAccounts.findFirst({
			where: and(
				eq(oauthAccounts.providerUserId, googleUser.sub),
				eq(oauthAccounts.providerId, 'google')
			)
		});

		if (existingUser) {
			const session = await lucia.createSession(existingUser.userId, {});
			const sessionCookie = lucia.createSessionCookie(session.id);
			setCookie(cookies, sessionCookie);
			// Redirect to the next url after the catch block
		} else {
			// Insert the new user into the database
			const new_user_id = generateId(15);

			await db
				.insert(newUsers)
				.values({
					id: new_user_id,
					providerId: 'google',
					providerUserId: googleUser.sub,
					expirationTime: new Date(Date.now() + 60 * 60 * 1000) // Expires after 1 hour
				})
				.execute();

			// Redirect to the signup page
			url = new URL('/auth/signup', url.origin);
			url.searchParams.set('new_user_key', new_user_id);
			// Redirect to this url outside of the try/catch block
			redirect_to = url.toString();
		}
	} catch (e) {
		if (e instanceof OAuth2RequestError) {
			// invalid code
			console.error("OAuth2 error:", e);
			error(400, "Invalid OAuth callback (OAuth2 error)");
		}

		console.error(e);
		error(500);
	}

	redirect(302, redirect_to);
};
