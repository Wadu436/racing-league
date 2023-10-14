/// <references types="houdini-svelte">
import { DateTime, Duration } from 'luxon';

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
			type: 'luxon.DateTime',
			unmarshal(val) {
				return val ? DateTime.fromISO(val) : null;
			},
			marshal(date) {
				return date && date.toIso();
			}
		},
		Laptime: {
			type: 'luxon.Duration',
			unmarshal(val) {
				return val ? Duration.fromMillis(val) : null;
			},
			marshal(dur) {
				return dur && Math.floor(dur.as('milliseconds'));
			}
		}
	}
};

export default config;
