<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import Crud from '$lib/component/Crud.svelte';
	import type { PageData } from './$types';
	import type { ActionData } from './$types';
	import { CheckCircle2, XCircle } from 'lucide-svelte';

	export let data: PageData;
	export let form: ActionData;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'username', label: 'Username', type: 'string' },
			{ name: 'admin', label: 'Admin', type: 'checkbox' },
			{ name: 'staff', label: 'Staff', type: 'checkbox' }
		],
		data: data.users,
		edit: data.dbUser
	}}
	baseUrl="/admin/users/"
	errorMessage={form?.message}
/>
