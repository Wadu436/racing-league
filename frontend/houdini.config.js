/// <references types="houdini-svelte">
import { DateTime } from 'luxon';

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
		DateTime: {
			type: "luxon.DateTime",
			unmarshal(val) {
				return val ? DateTime.fromISO(val) : null;
			},
			marshal(date) {
				return date && date.toIso();
			}
		}
	}
};

export default config;
