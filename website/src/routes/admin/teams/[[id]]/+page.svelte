<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData } from './$types';
	import type { ActionData } from './$types';
	import TeamsForm from './teams-form.svelte';

	export let data: PageData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'country', label: 'Country', type: 'country' }
		],
		data: data.teams,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/teams/"
	><div slot="form">
		{#if data.form}
			<TeamsForm data={data.form} />
		{/if}
	</div>
</Crud>
