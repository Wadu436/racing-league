<script lang="ts">
	import { countryCodeAlpha2Dict, flagUrlFromAlpha2 } from './countryCodes';

	export let alpha2: string;
	export let size: 'l' | 'm' | 's' = 'l';
</script>

<img
	src={flagUrlFromAlpha2(alpha2, size)}
	alt="Flag of {countryCodeAlpha2Dict[alpha2]?.countryName}"
/>
