import { HoudiniClient } from '$houdini';

export default new HoudiniClient({
	url: 'http://127.0.0.1:8000',
	fetchParams: () => {
		return {
			credentials: 'include'
		};
	}
});
