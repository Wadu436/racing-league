<script lang="ts">
	import type { PageData } from './$houdini';

	export let data: PageData;

	let foo = 'Bar!';

	$: ({ Info } = data);
</script>

{#if $Info.fetching}
	<div>Loading...</div>
{:else if $Info.data}
	{#each $Info.data.leagues as league}
		<!-- TODO: Add a League card here, maybe with a picture? -->
		<div>
			<a href="/league/{league.id}">{league.name}</a>
		</div>
	{/each}
{:else}
	<div>Error</div>
{/if}
