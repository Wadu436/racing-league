<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Card from '$lib/components/ui/card';
	import type { PageData } from './$types';

	export let data: PageData;

	$: user = data.user;
</script>

{#if user}
	<Card.Root class="w-[380px]">
		<Card.Header>
			<Card.Title>Profile</Card.Title>
		</Card.Header>
		<Card.Content>
			<p><b>User id:</b> {user.id}</p>
			<p><b>Username:</b> {user.username}</p>
			<p><b>Admin:</b> {user.admin}</p>
			<p><b>Staff:</b> {user.staff}</p>
		</Card.Content>
		<Card.Footer>
			<form method="post">
				<Button type="submit" formaction="/auth?/signout">Sign out</Button>
				<Button variant="destructive" type="submit" formaction="/auth?/delete"
					>Delete account</Button
				>
			</form>
		</Card.Footer>
	</Card.Root>
{:else}
	<div><p>You are not logged in</p></div>
{/if}
