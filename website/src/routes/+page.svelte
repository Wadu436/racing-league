<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;

	$: user = data.user;
</script>

{#if user}
	<h1 class="text-lg font-bold">Profile</h1>
	<p><b>User id:</b> {user.id}</p>
	<p><b>Username:</b> {user.username}</p>
	<form method="post">
		<button class="variant-filled btn" type="submit" formaction="/auth?/signout">Sign out</button>
		<button class="variant-filled btn bg-error-800" type="submit" formaction="/auth?/delete"
			>Delete account</button
		>
	</form>
{:else}
	<div><p>You are not logged in</p></div>
{/if}
