<script lang="ts">
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import type { PageData } from '../../profile/$houdini';

	export let data: PageData;

	$: ({ ProfileQuery } = data);

	$: profilePictureUrl =
		$ProfileQuery?.data?.me &&
		`${PUBLIC_BACKEND_URL}/files/${$ProfileQuery.data.me.profilePicturePath}`;
</script>

{#if $ProfileQuery.fetching}
	<div>Fetching...</div>
{:else if $ProfileQuery.errors}
	<div>Error</div>
{:else if $ProfileQuery.data?.me}
	<div>Username: {$ProfileQuery.data.me.username}</div>
	<div>Steam ID: {$ProfileQuery.data.me.steamId}</div>
	<div>EA ID: {$ProfileQuery.data.me.eaId}</div>
	<div>
		Profile Picture: <img
			class="h-48"
			alt={$ProfileQuery.data.me.username + "'s profile picture"}
			src={profilePictureUrl}
		/>
	</div>
	<div>
		<a href="/profile/edit">Edit</a>
	</div>
{:else}
	<div>User not found</div>
{/if}
