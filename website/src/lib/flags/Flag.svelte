<script lang="ts">
	import { countryCodeAlpha2Dict, flagUrlFromAlpha2 } from './countryCodes';

	export let alpha2: string;
	export let size: 'l' | 'm' | 's' = 'l';

	let pixels = 64;

	$: {
		if (size === 'l') {
			pixels = 48;
		} else if (size === 'm') {
			pixels = 32;
		} else if (size === 's') {
			pixels = 16;
		}
	}
</script>

<img
	class="inline-block h-auto"
	src={flagUrlFromAlpha2(alpha2)}
	alt="Flag of {countryCodeAlpha2Dict[alpha2]?.countryName}"
	width={pixels}
/>
