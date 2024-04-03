<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData } from './$types';
	import DriversForm from './drivers-form.svelte';

	export let data: PageData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'country', label: 'Country', type: 'country' },
			{ name: 'bot', label: 'Bot', type: 'checkbox' }
		],
		data: data.drivers,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/drivers/"
>
	<div slot="form">
		{#if data.form}
			<DriversForm data={data.form} />
		{/if}
	</div>
</Crud>
