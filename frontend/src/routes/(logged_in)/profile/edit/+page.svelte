<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData } from './$houdini';
	import type { ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: ({ ProfileEditQuery, ProfileQuery } = data);
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
	<form method="post" action="?/update" enctype="multipart/form-data">
		<div>
			<label for="username">Username</label>
			<input name="username" value={$ProfileEditQuery.data.me.username} required />
		</div>
		<div>
			<label for="steam-id">Steam ID</label>
			<input name="steam-id" value={$ProfileEditQuery.data.me.steamId} />
		</div>
		<div>
			<label for="ea-id">EA ID</label>
			<input name="ea-id" value={$ProfileEditQuery.data.me.eaId} />
		</div>
		<div>
			<label for="profile-picture">Profile Picture</label>
			<input name="profile-picture" type="file" accept="image/png, image/jpeg" />
		</div>
		<button>Save</button>
	</form>
{:else}
	<div>User not found</div>
{/if}
