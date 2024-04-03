<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';

	import { valibotClient } from 'sveltekit-superforms/adapters';
	import { formSchema, type FormSchema } from './schema';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: valibotClient(formSchema),
		resetForm: false
	});

	const { form: formData, message, enhance } = form;
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="name">
		<Form.Control let:attrs>
			<Form.Label>Name</Form.Label>
			<Input {...attrs} bind:value={$formData.name} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button formaction="?/save">Save</Form.Button>
	<Form.Button variant="destructive" formaction="?/delete">Delete</Form.Button>
</form>
{#if $message}
	<div>{$message}</div>
{/if}
