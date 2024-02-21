<script lang="ts">
	import Flag from '$lib/flags/Flag.svelte';
	import { countryCodeList } from '$lib/flags/countryCodes';
</script>

{#each countryCodeList as { alpha2, countryName }}
	<div>
		<Flag {alpha2} size="l" /> <span>{countryName}</span>
	</div>
{/each}

<style>
</style>
