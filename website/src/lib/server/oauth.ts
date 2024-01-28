import { Google } from 'arctic';

import {
	GOOGLE_OAUTH_CLIENT_ID,
	GOOGLE_OAUTH_CLIENT_SECRET,
	GOOGLE_OAUTH_REDIRECT_URI
} from '$env/static/private';

export const googleAuth = new Google(
	GOOGLE_OAUTH_CLIENT_ID,
	GOOGLE_OAUTH_CLIENT_SECRET,
	GOOGLE_OAUTH_REDIRECT_URI
);
