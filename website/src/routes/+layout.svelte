<script>
	import '../app.pcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';

	export let data;
</script>

<AppShell>
	<slot />
	<div slot="header">
		<AppBar>
			<div slot="lead">
				<a href="/">racing.warre.dev</a>
			</div>
			<a class="anchor" href="/">home</a>
			<a class="anchor" href="/protected">protected</a>

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
