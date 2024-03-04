<script lang="ts">
	import '../app.pcss';
	import type { LayoutData } from './$types';
	export let data: LayoutData;
</script>

<!-- Header -->
<div class="flex h-screen w-screen flex-col">
	<header class="flex-none">
		<div class="flex">
			<div>
				<a href="/">racing.warre.dev</a>
			</div>
			<a class="anchor" href="/">home</a>
			{#if data.user?.staff || data.user?.admin}
				<a class="anchor" href="/admin">admin</a>
			{/if}
			<div>
				{#if data.user}
					<p>Logged in as {data.user.username}</p>
					<a class="anchor" href="/profile">Profile</a>
					<form method="post">
						<button class="variant-filled btn" type="submit" formaction="/auth?/signout"
							>Sign out</button
						>
					</form>
				{:else}
					<a class="anchor" href="/auth/signin">Sign in</a>
				{/if}
			</div>
		</div>
	</header>

	<div class="flex h-full w-full flex-auto overflow-hidden">
		<div class="flex flex-1 flex-col overflow-x-hidden" style:scrollbar-gutter="auto" on:scroll>
			<main class="flex-auto"><slot /></main>

			<!-- Footer -->
			<footer>
				<p>Footer</p>
			</footer>
		</div>
	</div>
</div>
