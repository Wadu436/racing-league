<script lang="ts">
	import TyreIcon from '$lib/Tyres.svelte';
	import Flag from '$lib/Flag.svelte';
	import { Duration } from 'luxon';

	import { SessionFragmentStore } from '$houdini';
	import type { SessionFragment } from '$houdini';

	export let session: SessionFragment;
	$: data = new SessionFragmentStore().get(session);
</script>

<h1>{$data?.event.name}</h1>

<div class="flex justify-center">
	<div class="w-fit">
		<div class="flex gap-4 py-2 px-16 justify-around">
			{#each $data?.event.sessions || [] as session}
				<a data-sveltekit-replacestate href={session.id} class:underline={session.id == $data?.id}>
					{session.sessionType
						.toLowerCase()
						.split('_')
						.map((s) => s.charAt(0).toUpperCase() + s.slice(1))
						.join(' ')}
				</a>
			{/each}
		</div>
		<div class="flex flex-col">
			{#each $data?.entries.sort((a, b) => a.finishPosition - b.finishPosition) || [] as entry}
				<div
					class="flex gap-4 whitespace-nowrap p-3 m-1 border border-gray-700 rounded-xl w-fit"
					class:bg-purple-600={$data?.fastestLap.id == entry.driver.id}
					class:text-white={$data?.fastestLap.id == entry.driver.id}
					class:bg-primary-50={!($data?.fastestLap.id == entry.driver.id)}
				>
					<div class="w-8 text-right">
						{entry.finishStatus == 'CLASSIFIED' ? entry.finishPosition : entry.finishStatus}
					</div>
					<Flag alpha2={entry.driver.nationality} size="l" />
					<div class="w-32 overflow-hidden text-ellipsis text-right">{entry.driver.name}</div>
					<div class="w-60 overflow-hidden text-ellipsis">{entry.team.name}</div>
					<div class="w-24 flex gap-0.5 overflow-x-auto">
						{#each entry.laps
							.map((lap) => lap.tyres)
							.filter((_, idx, array) => idx == 0 || array[idx - 1] != array[idx]) as tyre}
							<TyreIcon {tyre} class="w-6 h-6" />
						{/each}
					</div>
					<div class="w-20">
						{#if entry.fastestLap}
							{Duration.fromMillis(entry.fastestLap.laptimeInMs).toFormat('mm:ss.SSS')}
						{:else}
							--:--.---
						{/if}
					</div>
					<div class="w-16 text-right">
						{entry.points} pts
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
