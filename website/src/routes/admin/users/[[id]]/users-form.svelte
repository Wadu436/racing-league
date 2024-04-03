<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';

	import { valibotClient } from 'sveltekit-superforms/adapters';
	import { formSchema, type FormSchema } from './schema';
	import * as Select from '$lib/components/ui/select';

	import { countryCodeList } from '$lib/flags/countryCodes';
	import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte';
	import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: valibotClient(formSchema),
		resetForm: false
	});

	const { form: formData, message, enhance } = form;
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Form.Label>Username</Form.Label>
			<Input {...attrs} bind:value={$formData.username} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="admin">
		<Form.Control let:attrs>
			<Form.Label>Admin</Form.Label>
			<Checkbox {...attrs} bind:checked={$formData.admin} />
			<input name={attrs.name} value={$formData.admin} hidden />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="staff">
		<Form.Control let:attrs>
			<Form.Label>Staff</Form.Label>
			<Checkbox {...attrs} bind:checked={$formData.staff} />
			<input name={attrs.name} value={$formData.staff} hidden />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button formaction="?/save">Save</Form.Button>
</form>
{#if $message}
	<div>{$message}</div>
{/if}
