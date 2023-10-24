import { HoudiniClient } from '$houdini';
import { backendServerUrl } from './config';

export default new HoudiniClient({
	url: backendServerUrl,
	fetchParams: () => {
		return {
			credentials: 'include'
		};
	}
});
