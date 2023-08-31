import { HoudiniClient } from '$houdini';

export default new HoudiniClient({
	url: 'http://localhost:8000',
	fetchParams: () => {
		return {
			credentials: 'include'
		};
	}
});
