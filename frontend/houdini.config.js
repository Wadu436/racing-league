/// <references types="houdini-svelte">

/** @type {import('houdini').ConfigFile} */
const config = {
	watchSchema: {
		url: 'http://127.0.0.1:8000'
	},
	plugins: {
		'houdini-svelte': {}
	},

	scalars: {
		/* in your case, something like */
		UUID: {
			// <- The GraphQL Scalar
			type: 'string' // <-  The TypeScript type
		}
	}
};

export default config;
