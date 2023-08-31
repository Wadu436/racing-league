<script lang="ts">
	import type { PageData } from './$houdini';

	export let data: PageData;

	$: ({ InfoQuery } = data);
</script>

{#if $InfoQuery.fetching}
	<div>Loading...</div>
{:else if $InfoQuery.data}
	{#each $InfoQuery.data.leagues as league}
		<!-- TODO: Add a League card here, maybe with a picture? -->
		<div>
			<a href="/league/{league.id}">{league.name}</a>
		</div>
	{/each}
{:else}
	<div>Error</div>
{/if}
