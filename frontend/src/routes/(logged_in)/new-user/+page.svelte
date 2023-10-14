<script lang="ts">
	import type { PageData } from './$houdini';
	import { page } from '$app/stores';
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let data: PageData;
    export let form: ActionData;

	$: ({ ProfileQuery } = data);
</script>

{#if $ProfileQuery.fetching}
	<div>Fetching...</div>
{:else if $ProfileQuery.errors}
	<div>Error</div>
{:else if $ProfileQuery.data?.me}
	<div>You already have a user account!</div>
{:else if $page.data.session?.user == null}
    <div>You are not logged in!</div>
{:else}
    {#if form?.usernameMissing}<p>The Username field is required</p>{/if}
	<form method="POST" action="?/createUser" use:enhance>
		<div>Create new account</div>
		<div>
			<label for="email">Email:</label>
			<span>{$page.data.session?.user?.email}</span>
		</div>
		<div>
			<label for="username">Username</label>
			<input name="username" type="text" required/>
		</div>
        <div>
			<label for="steam-id">Steam ID (optional)</label>
			<input name="steam-id" type="text"/>
		</div>
        <div>
			<label for="steam-id">EA ID (optional)</label>
			<input name="steam-id" type="text"/>
		</div>
        <button>Submit</button>
	</form>
{/if}
