<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div>Heath ChecK:</div>

<div>
	{#await data.health_check}
		<div>Loading...</div>
	{:then health_check}
		<div>{health_check}</div>
	{/await}
</div>
