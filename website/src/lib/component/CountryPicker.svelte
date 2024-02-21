<script lang="ts">
	import { countryCodeList } from '$lib/flags/countryCodes';

	export let name: string;
	export let id: string;
	export let value: string | null | undefined;
</script>

<select {name} {id} {value}>
	{#each countryCodeList as { alpha2, countryName }}
		<option value={alpha2}>{countryName}</option>
	{/each}
	<option value="BE">Belgium</option>
	<option value="IT">Italy</option>
	<option value="GB">United Kingdom</option>
</select>
