<script lang="ts">
	import TyreIcon from '$lib/Tyres.svelte';
	import Flag from '$lib/Flag.svelte';
	import type { PageData } from './$houdini';
	import { Duration } from 'luxon';

	export let data: PageData;

	$: ({ SessionQuery } = data);
</script>

<h1>{$SessionQuery.data?.session.event.name}</h1>

<div>
	<div class="flex gap-4">
		{#each $SessionQuery.data?.session.event.sessions || [] as session}
			<a
				data-sveltekit-replacestate
				href={session.id}
				class:underline={session.id == $SessionQuery.data?.session.id}
			>
				{session.sessionType}
			</a>
		{/each}
	</div>
	<div>
		{#each $SessionQuery.data?.session.entries.sort((a, b) => a.finishPosition - b.finishPosition) || [] as entry}
			<div
				class="flex gap-4"
				class:bg-purple-200={$SessionQuery.data?.session.fastestLap.id == entry.driver.id}
			>
				<div>
					{entry.finishStatus == 'CLASSIFIED' ? entry.finishPosition : entry.finishStatus}
				</div>
				<Flag alpha2={entry.driver.nationality} size="m" />
				<div>{entry.driver.name}</div>
				<div>{entry.team.name}</div>
				<div class="flex gap-0.5">
					{#each entry.laps
						.map((lap) => lap.tyres)
						.filter((_, idx, array) => idx == 0 || array[idx - 1] != array[idx]) as tyre}
						<TyreIcon {tyre} class="w-6 h-6" />
					{/each}
				</div>
				{#if entry.fastestLap}
					<div>{Duration.fromMillis(entry.fastestLap.laptimeInMs).toFormat('mm:ss.SSS')}</div>
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
