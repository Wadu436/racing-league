<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import { superForm } from 'sveltekit-superforms';
	import type { PageData } from './$types';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';

	export let data: PageData;

	const form = superForm(data.form);
	const { form: formData, enhance, message } = form;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'status', label: 'Status', type: 'string' }
		],
		data: data.leagues,
		creatable: false
	}}
	baseUrl="/admin/leagues/"
	><div slot="form">
		<div class="max-w-96">
			<div class="mb-2">Or create a new league!</div>
			<form method="POST" use:enhance>
				<Form.Field {form} name="name">
					<Form.Control let:attrs>
						<Form.Label>Name</Form.Label>
						<Input {...attrs} bind:value={$formData.name} />
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Button formaction="?/create">Create</Form.Button>
			</form>
			{#if $message}
				<div>{$message}</div>
			{/if}
		</div>
	</div>
</Crud>
