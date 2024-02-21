<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData } from './$types';
	import type { ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'country', label: 'Country', type: 'country' },
			{ name: 'bot', label: 'Bot', type: 'checkbox' }
		],
		data: data.drivers,
		edit: data.dbDriver,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/drivers/"
	errorMessage={form?.message}
/>
