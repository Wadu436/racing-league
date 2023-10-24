import { backendServerUrl } from '../../config';

export const uploadProfilePicture = async (profilePicture: File) => {
    console.log('profilePicture', profilePicture);
	const url = backendServerUrl + '/upload';
	const formData = new FormData();
	formData.append('file', profilePicture);
	const response = await fetch(url, {
		method: 'POST',
		body: formData
	});
	console.log('response', response);
	return await response.text();
};
