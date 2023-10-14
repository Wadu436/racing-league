<script lang="ts">
	import type { PageData } from './$houdini';
	import type { ActionData } from './$types';

	export let data: PageData;
    export let form: ActionData;

	$: ({ ProfileEditQuery } = data);
</script>

{#if $ProfileEditQuery.fetching}
	<div>Fetching...</div>
{:else if $ProfileEditQuery.errors}
	<div>Error</div>
{:else if $ProfileEditQuery.data?.me}
    {#if form?.success}
        <div>Successfully updated!</div>
    {:else}
        {#each form?.serverErrors ?? [] as error}
            <div>{error.message}</div>
        {/each}
        {#if form?.usernameMissing}
            <div>Missing username</div>
        {/if}
    {/if}
	<form method="post" action="?/update">
		<div>
			<label for="username">Username</label>
			<input name="username" value={$ProfileEditQuery.data.me.username} required />
		</div>
		<div>
			<label for="steamId">Steam ID</label>
			<input name="steamId" value={$ProfileEditQuery.data.me.steamId} />
		</div>
		<div>
			<label for="eaId">EA ID</label>
			<input name="eaId" value={$ProfileEditQuery.data.me.eaId} />
		</div>
        <button>Save</button>
	</form>
{:else}
	<div>User not found</div>
{/if}
