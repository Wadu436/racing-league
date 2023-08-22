<script lang="ts">
	import TyreIcon from '$lib/Tyres.svelte';
	import Flag from '$lib/Flag.svelte';
	import type { PageData } from './$houdini';

	export let data: PageData;

	$: ({ Session } = data);
</script>

<h1>{$Session.data?.session.event.name}</h1>

<div>
	<div class="flex gap-4">
		{#each $Session.data?.session.event.sessions || [] as session, i}
			<a href={session.id} class:underline={session.id == $Session.data?.session.id}>
				{session.sessionType}
			</a>
		{/each}
	</div>
	<div>
		{#each $Session.data?.session.participants.sort((a, b) => a.classification.position - b.classification.position) || [] as entry}
			<div
				class="flex gap-4"
				class:bg-purple-200={$Session.data?.session.fastestLap.id == entry.user.id}
			>
				<div>
					{entry.classification.finishStatus == 'FINISHED'
						? entry.classification.position
						: entry.classification.finishStatus}
				</div>
				<Flag alpha2={entry.user.nationality} size="m" />
				<div>{entry.user.name}</div>
				<div>{entry.team.name}</div>
				<div class="flex gap-0.5">
					{#each entry.laps
						.map((lap) => lap.tyres)
						.filter((_, idx, array) => idx == 0 || array[idx - 1] != array[idx]) as tyre}
						<TyreIcon {tyre} class="w-6 h-6" />
					{/each}
				</div>
				{#if entry.fastestLap}
					<div>{entry.fastestLap.laptimeInMs.toFormat('mm:ss.SSS')}</div>
				{:else}
					<div>--:--.---</div>
				{/if}
				<div>
					{entry.points} pts
				</div>
			</div>
		{/each}
	</div>
</div>
