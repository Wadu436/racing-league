<script lang="ts">
	import type { PageData } from './$houdini';

	export let data: PageData;

	$: ({ MeQuery } = data);
</script>

{#if $MeQuery.fetching}
	<div>Fetching...</div>
{:else if $MeQuery.errors}
	<div>Error</div>
{:else if $MeQuery.data?.me}
	<div>Username: {$MeQuery.data.me.username}</div>
	<div>Sub: {$MeQuery.data.me.sub}</div>
{:else}
	<div>User not found</div>
{/if}
