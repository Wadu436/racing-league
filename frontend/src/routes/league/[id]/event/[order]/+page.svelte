<script lang="ts">
	import type { PageData } from './$houdini';

	export let data: PageData;

	let sessionIdx: number = 0;

	$: ({ Event } = data);
	$: selectedSession = $Event.data?.league.event.sessions[sessionIdx];
</script>

<h1>{$Event.data?.league.event.name}</h1>

<div>
	<div class="flex gap-4">
		{#each $Event.data?.league.event.sessions || [] as session, i}
			<button
				on:click={() => {
					sessionIdx = i;
				}}
				class:underline={i == sessionIdx}
			>
				{session.sessionType}
			</button>
		{/each}
	</div>
	<div>
		{#each selectedSession?.classification || [] as entry, i}
			<div>{i + 1}: {entry.user.name} - {entry.user.nationality} - {entry.team.name}</div>
		{/each}
	</div>
</div>
