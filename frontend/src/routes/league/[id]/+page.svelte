<script lang="ts">
	import VerticalTab from '$lib/VerticalTab.svelte';
import type { PageData } from './$houdini';

	export let data: PageData;

	$: ({ League } = data);
</script>

<h1>
	<span>{$League.data?.league.name}</span>
	<span>({$League.data?.league.status})</span>
</h1>

<div>
    <VerticalTab>
        <div slot="left">
            Sessions
            {#each $League.data?.league.events || [] as event}
            <div>
                <div>
                    {event.name}
                </div>
            </div>
        {/each}
        </div><div slot="right">
            Leaderboard
        </div>
    </VerticalTab>
	
</div>

<div />
