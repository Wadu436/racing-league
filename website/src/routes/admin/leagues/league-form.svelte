<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';

	import { valibotClient } from 'sveltekit-superforms/adapters';
	import { formSchema, type FormSchema } from './schema';
	import * as Select from '$lib/components/ui/select';
	import * as Dialog from '$lib/components/ui/dialog/index.js';

	import { PlusCircle } from 'lucide-svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { buttonVariants } from '$lib/components/ui/button';
	import DatePicker from '$lib/components/date-picker.svelte';

	export let data: SuperValidated<Infer<FormSchema>>;
	export let tracks: { id: string; name: string }[];

	const form = superForm(data, {
		validators: valibotClient(formSchema),
		resetForm: false,
		dataType: 'json'
	});

	const { form: formData, message, enhance } = form;

	$: selectedStatus = {
		value: $formData.status,
		label: $formData.status.charAt(0).toUpperCase() + $formData.status.slice(1)
	};

	$: selectedTracks = $formData.events.map((e) => ({
		value: e.trackId,
		label: tracks.find((t) => t.id === e.trackId)?.name
	}));
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="name">
		<Form.Control let:attrs>
			<Form.Label>Name</Form.Label>
			<Input {...attrs} bind:value={$formData.name} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="status">
		<Form.Control let:attrs>
			<Form.Label>Status</Form.Label>
			<Select.Root
				selected={selectedStatus}
				onSelectedChange={(s) => {
					s && ($formData.status = s.value);
				}}
			>
				<Select.Input name={attrs.name} />
				<Select.Trigger {...attrs}>
					<Select.Value />
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="upcoming" label="Upcoming" />
					<Select.Item value="ongoing" label="Ongoing" />
					<Select.Item value="finished" label="Finished" />
				</Select.Content>
			</Select.Root>
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<div>Events:</div>
	<div class="ml-4">
		{#each $formData.events as _, i}
			<div>
				<Dialog.Root>
					<Dialog.Trigger class={buttonVariants({ variant: 'outline' })}>
						{i + 1}. {$formData.events[i].name}
					</Dialog.Trigger>
					<Dialog.Content>
						<Dialog.Header>
							<Dialog.Title>Edit Event {i + 1}</Dialog.Title>
							<Dialog.Description>Make changes to the event here.</Dialog.Description>
						</Dialog.Header>
						<div>
							<Form.Field {form} name={`events[${i}].name`}>
								<Form.Control let:attrs>
									<Form.Label>Event Name</Form.Label>
									<Input {...attrs} bind:value={$formData.events[i].name} />
								</Form.Control>
								<Form.FieldErrors />
							</Form.Field>
							<DatePicker {form} label="Date" name={`events[${i}].date`} />
							<Form.Field {form} name={`events[${i}].trackId`}>
								<Form.Control let:attrs>
									<Form.Label>Track</Form.Label>
									<Select.Root
										selected={selectedTracks[i]}
										onSelectedChange={(s) => {
											s && ($formData.events[i].trackId = s.value);
										}}
									>
										<Select.Input name={attrs.name} />
										<Select.Trigger {...attrs}>
											<Select.Value />
										</Select.Trigger>
										<Select.Content>
											<ScrollArea class="max-h-72">
												{#each tracks as { id, name }}
													<Select.Item value={id} label={name} />
												{/each}
											</ScrollArea>
										</Select.Content>
									</Select.Root>
								</Form.Control>
								<Form.FieldErrors />
							</Form.Field>
						</div>
					</Dialog.Content>
				</Dialog.Root>
			</div>
		{/each}
		<div>
			Add a new race <Button
				variant="default"
				on:click={() =>
					($formData.events = [...$formData.events, { name: 'New event', date: new Date(), trackId: '' }])}
				><PlusCircle /></Button
			>
		</div>
	</div>

	<Form.Button formaction="?/save">Save</Form.Button>
	<Form.Button variant="destructive" formaction="?/delete">Delete</Form.Button>
</form>
{#if $message}
	<div>{$message}</div>
{/if}
