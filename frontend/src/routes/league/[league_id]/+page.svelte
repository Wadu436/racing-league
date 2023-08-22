<script lang="ts">
	import Flag from '$lib/Flag.svelte';
	import VerticalTab from '$lib/VerticalTab.svelte';
	import { countryCodeAlpha2Dict, flagUrlFromAlpha2 } from '$lib/countryCodes';
	import type { PageData } from './$houdini';
	export let data: PageData;
	import { Icon } from 'svelte-flags';

	$: ({ League } = data);
</script>

<h1>
	<span>{$League.data?.league.name}</span>
	<span>({$League.data?.league.status})</span>
</h1>

<div>
	<VerticalTab>
		<div slot="left">
			Events
			{#each $League.data?.league.events || [] as event}
				<a
					href="/session/{event.sessions.sort((a, b) => {
						if (a.sessionType == b.sessionType) {
							return 0;
						}
						if (a.sessionType == 'RACE') {
							return -1;
						} else if (b.sessionType == 'RACE') {
							return 1;
						}
						if (a.sessionType == 'SPRINT') {
							return -1;
						} else if (b.sessionType == 'SPRINT') {
							return 1;
						}
						if (a.sessionType == 'QUALIFYING') {
							return  -1;
						} else if (b.sessionType == 'QUALIFYING') {
							return 1;
						}
						if (a.sessionType == 'SPRINT_QUALIFYING') {
							return -1;
						} else if (b.sessionType == 'SPRINT_QUALIFYING') {
							return 1;
						}
						if (a.sessionType == 'PRACTICE') {
							return -1;
						} else if (b.sessionType == 'PRACTICE') {
							return 1;
						}
						return 0;
					})[0]?.id}"
					class="bg-gray-200 p-4 rounded-lg mb-2 flex justify-start"
				>
					<div class="flex flex-col justify-center w-[4.1rem]">
						<div class="text-lg font-semibold flex items-center justify-between">
							<div>
								{event.championshipOrder.toString().padStart(2, '0')}
							</div>
							<div class="pl-2">
								<Flag alpha2={event.track.country} />
							</div>
						</div>
						<div class="text-sm text-start">
							{event.date.toFormat('dd LLL')}
						</div>
					</div>
					<div class="w-1 rounded-lg bg-gray-300 mx-2" />
					<div>
						<div class="text-lg font-semibold">
							{event.name}
						</div>
						<div>
							{event.track.name}
						</div>
					</div>
				</a>
			{/each}
		</div>
		<div slot="right">Leaderboard</div>
	</VerticalTab>
</div>

<div />
