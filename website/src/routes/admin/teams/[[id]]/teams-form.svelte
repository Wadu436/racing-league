<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';

	import { valibotClient } from 'sveltekit-superforms/adapters';
	import { formSchema, type FormSchema } from './schema';
	import * as Select from '$lib/components/ui/select';

	import { countryCodeList } from '$lib/flags/countryCodes';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: valibotClient(formSchema),
		resetForm: false
	});

	console.log('Form:', form);

	const { form: formData, message, enhance } = form;

	$: selectedCountry = {
		value: $formData.country ?? '',
		label: countryCodeList.find((c) => c.alpha2 === $formData.country)?.countryName
	};
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="name">
		<Form.Control let:attrs>
			<Form.Label>Name</Form.Label>
			<Input {...attrs} bind:value={$formData.name} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="country">
		<Form.Control let:attrs>
			<Form.Label>Country</Form.Label>
			<Select.Root
				selected={selectedCountry}
				onSelectedChange={(s) => {
					s && ($formData.country = s.value);
				}}
			>
				<Select.Input name={attrs.name} />
				<Select.Trigger {...attrs}>
					<Select.Value />
				</Select.Trigger>
				<Select.Content>
					<ScrollArea class="h-72">
						{#each countryCodeList as { alpha2, countryName }}
							<Select.Item value={alpha2} label={countryName} />
						{/each}
					</ScrollArea>
				</Select.Content>
			</Select.Root>
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button formaction="?/save">Save</Form.Button>
	<Form.Button variant="destructive" formaction="?/delete">Delete</Form.Button>
</form>
{#if $message}
	<div>{$message}</div>
{/if}
