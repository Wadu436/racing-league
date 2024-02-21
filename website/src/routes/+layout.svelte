<script lang="ts">
	import '../app.pcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import type { LayoutData } from './$types';
	export let data: LayoutData;
</script>

<AppShell>
	<slot />
	<div slot="header">
		<AppBar>
			<div slot="lead">
				<a href="/">racing.warre.dev</a>
			</div>
			<a class="anchor" href="/">home</a>
			{#if data.user?.staff || data.user?.admin}
				<a class="anchor" href="/admin">admin</a>
			{/if}

			<div slot="trail">
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
		</AppBar>
	</div>
	<div slot="pageFooter">
		<p>Footer</p>
	</div>
</AppShell>
