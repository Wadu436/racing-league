<script lang="ts">
	import type { PageData } from './$houdini';

	export let data: PageData;

	$: ({ MeQuery } = data);
</script>

<div>Sub: {$MeQuery.data?.me.sub}</div>
