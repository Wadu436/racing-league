<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData, ActionData } from './$types';
	import TracksForm from './tracks-form.svelte';

	export let data: PageData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'country', label: 'Country', type: 'country' }
		],
		data: data.tracks,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/tracks/"
	><div slot="form">
		{#if data.form}
			<TracksForm data={data.form} />
		{/if}
	</div>
</Crud>
