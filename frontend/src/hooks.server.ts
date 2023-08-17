import type { HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ request, fetch }) => {
	//   if (request.url.startsWith('https://api.yourapp.com/')) {
	//     // clone the original request, but change the URL
	//     request = new Request(
	//       request.url.replace('https://api.yourapp.com/', 'http://localhost:9999/'),
	//       request
	//     );
	//   }

	console.log('Fetching: ', request);

	return fetch(request);
};
