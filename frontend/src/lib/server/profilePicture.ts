import { PUBLIC_BACKEND_URL } from '$env/static/public';

export const uploadProfilePicture = async (profilePicture: File) => {
    console.log('profilePicture', profilePicture);
	const url = PUBLIC_BACKEND_URL + '/upload';
	const formData = new FormData();
	formData.append('file', profilePicture);
	const response = await fetch(url, {
		method: 'POST',
		body: formData
	});
	console.log('response', response);
	return await response.text();
};
