<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData, ActionData } from './$types';
	

	export let data: PageData;
	export let form: ActionData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'country', label: 'Country', type: 'country' },
		],
		data: data.tracks,
		edit: data.dbTrack,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/tracks/"
	errorMessage={form?.message}
/>
