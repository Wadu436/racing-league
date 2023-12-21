<script lang="ts">
	import type { PageData } from './$houdini';
	import Session from '$lib/Session.svelte';

	export let data: PageData;

	$: ({ SessionQuery } = data);
</script>

{#if $SessionQuery.data}
	<Session session={$SessionQuery.data.session} />
{/if}
