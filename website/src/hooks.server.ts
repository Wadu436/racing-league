import { lucia, setCookie } from '$lib/server/lucia';
import { redirect, type Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

// Hook to authenticate the user on every request
const authHook: Handle = async ({ event, resolve }) => {
	const sessionId = event.cookies.get(lucia.sessionCookieName);
	if (!sessionId) {
		event.locals.user = undefined;
		event.locals.session = undefined;
		return resolve(event);
	}

	const { session, user } = await lucia.validateSession(sessionId);
	if (session && session.fresh) {
		const sessionCookie = lucia.createSessionCookie(session.id);
		setCookie(event.cookies, sessionCookie);
	}
	if (!session) {
		const sessionCookie = lucia.createBlankSessionCookie();
		setCookie(event.cookies, sessionCookie);
	} else {
		event.locals.user = user;
		event.locals.session = session;
	}
	return resolve(event);
};

// Hook to protect certain routes, so that only logged in users can access them
const protectedHook: Handle = async ({ event, resolve }) => {
	if (event.url.pathname.startsWith('/admin')) {
		if (!event.locals.session) {
			// TODO add access control here
			const nextUrl = `${event.url.pathname}${event.url.search}`;
			const signinUrl = new URL('/auth/signin', event.url);
			signinUrl.searchParams.set('next', nextUrl);
			redirect(303, signinUrl.toString());
		}
	}
	return resolve(event);
};

export const handle: Handle = sequence(authHook, protectedHook);
