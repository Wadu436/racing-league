import { PUBLIC_BACKEND_URL } from '$env/static/public';
import { HoudiniClient } from '$houdini';

export default new HoudiniClient({
	url: PUBLIC_BACKEND_URL,
	fetchParams: () => {
		return {
			credentials: 'include'
		};
	}
});
