<script lang="ts">
	import { page } from '$app/stores';
	import { signIn, signOut } from '@auth/sveltekit/client';
	import '../app.css';
	import type { PageData } from './$houdini';

	export let data: PageData;

	$: ({ RootLayoutQuery } = data);
</script>

<div class="w-screen h-screen flex flex-col bg-gray-50 overflow-auto p-4">
	<nav class="p-4 bg-primary-700 rounded-lg text-gray-50 flex">
		<div id="links" class="flex-grow">
			<a class="hover:text-gray-200 hover:underline" href="/">Home</a>
		</div>
		<div id="account" class="flex gap-2">
			{#if $RootLayoutQuery.data?.me}
				<a href="/profile">{$RootLayoutQuery.data?.me?.username}</a>
				<button class="hover:text-gray-200 hover:underline" on:click={() => signOut()}
					>Log out</button
				>
			{:else}
				<button
					class="hover:text-gray-200 hover:underline"
					on:click={() => signIn('auth0', { callbackUrl: `/new-user?redirect=${$page.url.pathname}` })}>Log in</button
				>
			{/if}
		</div>
	</nav>

	<main class="flex-grow">
		<slot />
	</main>
</div>
