<script lang="ts">
    import Flag from '$lib/Flag.svelte';
    import type { PageData } from './$houdini';

    export let data: PageData;

    $: ({ LeagueQuery } = data);
</script>

<div>Leaderboard</div>
<div class="w-fit">
{#each $LeagueQuery.data?.league.wdcLeaderboard || [] as entry, i}
	<div class="flex gap-4 whitespace-nowrap p-3 m-1 border border-gray-700 rounded-xl w-fit bg-purple-50">
		<div class="w-8 text-right">{i + 1}</div>
        <Flag alpha2={entry.driver.nationality} size="l" />
		<div class="w-32 overflow-hidden text-ellipsis text-right">{entry.driver.name}</div>
		<div class="w-80 overflow-hidden text-ellipsis">{entry.team.name}</div>
		<div class="w-16 text-right">{entry.points} pts</div>
	</div>
{/each}
</div>