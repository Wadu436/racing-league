<script lang="ts">
	import { page as pageStore } from '$app/stores';

	const pages = [
		{ name: 'Users', path: '/admin/users' },
		{ name: 'Games', path: '/admin/games' },
		{ name: 'Teams', path: '/admin/teams' },
		{ name: 'Drivers', path: '/admin/drivers' },
		{ name: 'Tracks', path: '/admin/tracks' },
		{ name: 'Leagues', path: '/admin/leagues' },
	];
</script>

<div class="flex">
	<nav class="mr-2 border-r pr-2">
		<ul class="flex flex-col gap-2">
			{#each pages as page}
				<li>
					<a
						class="selectable block py-2 px-4"
						data-selected={$pageStore.url.pathname.startsWith(page.path)}
						href={page.path}>{page.name}</a
					>
				</li>
			{/each}
		</ul>
	</nav>
	<div class="flex-grow">
		<slot />
	</div>
</div>
