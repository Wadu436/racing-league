<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData, ActionData } from './$types';
	import GamesForm from './games-form.svelte';

	export let data: PageData;
</script>

<Crud
	shape={{
		columns: [{ name: 'name', label: 'Name', type: 'string' }],
		data: data.games,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/games/"
	><div slot="form">
		{#if data.form}
			<GamesForm data={data.form} />
		{/if}
	</div>
</Crud>
