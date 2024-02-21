<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData } from './$types';
	import type { ActionData } from './$types';
	import { CheckCircle2, XCircle, PlusCircle } from 'lucide-svelte';

	export let data: PageData;
	export let form: ActionData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'country', label: 'Country', type: 'country' }
		],
		data: data.teams,
		edit: data.dbTeam,
		creatable: true,
		create: data.create
	}}
	baseUrl="/admin/teams/"
	errorMessage={form?.message}
/>
