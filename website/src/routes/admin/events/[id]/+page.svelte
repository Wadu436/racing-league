<script lang="ts">
	import { FormField, FormControl, FormLabel, FormButton } from '$lib/components/ui/form';
	import FormFieldErrors from '$lib/components/ui/form/form-field-errors.svelte';
	import { Input } from '$lib/components/ui/input/index.js';
	import { superForm } from 'sveltekit-superforms';
	import DatePicker from '$lib/components/date-picker.svelte';
	import { Select, SelectItem, SelectValue } from '$lib/components/ui/select';
	import SelectTrigger from '$lib/components/ui/select/select-trigger.svelte';
	import SelectContent from '$lib/components/ui/select/select-content.svelte';
	import Breadcrumb from '$lib/components/ui/breadcrumb/breadcrumb.svelte';
	import BreadcrumbList from '$lib/components/ui/breadcrumb/breadcrumb-list.svelte';
	import BreadcrumbItem from '$lib/components/ui/breadcrumb/breadcrumb-item.svelte';
	import BreadcrumbLink from '$lib/components/ui/breadcrumb/breadcrumb-link.svelte';
	import BreadcrumbSeparator from '$lib/components/ui/breadcrumb/breadcrumb-separator.svelte';
	import BreadcrumbPage from '$lib/components/ui/breadcrumb/breadcrumb-page.svelte';
	import ReorderList from '$lib/components/reorder-list.svelte';

	export let data;

	let form = superForm(data.form, { resetForm: false });
	let { form: formData, enhance, message } = form;

	$: trackDict = Object.fromEntries(data.tracks.map((t) => [t.id, t]));
	$: selectedTrack = $formData.trackId
		? { label: trackDict[$formData.trackId].name, value: $formData.trackId }
		: undefined;
</script>

<nav class="flex">
	<Breadcrumb>
		<BreadcrumbList>
			<BreadcrumbItem>
				<BreadcrumbLink href="/admin/leagues">Leagues</BreadcrumbLink>
			</BreadcrumbItem>
			<BreadcrumbSeparator />
			<BreadcrumbItem>
				<BreadcrumbLink href={`/admin/leagues/${data.event.leagueId}`}>
					{data.event.league.name}
				</BreadcrumbLink>
			</BreadcrumbItem>
			<BreadcrumbSeparator />
			<BreadcrumbItem>
				<BreadcrumbPage>{data.event.name}</BreadcrumbPage>
			</BreadcrumbItem>
		</BreadcrumbList>
	</Breadcrumb>
</nav>

<div class="flex">
	<div>
		<div class="text-bold text-lg">Info</div>
		<form method="post" action="?/save" use:enhance>
			<FormField {form} name="name">
				<FormControl let:attrs>
					<FormLabel>Name</FormLabel>
					<Input {...attrs} bind:value={$formData.name} />
				</FormControl>
				<FormFieldErrors />
			</FormField>

			<DatePicker {form} name="date" label="Date" />
			<input type="hidden" name="date" value={$formData.date?.toISOString() ?? null} />

			<FormField {form} name="trackId">
				<FormControl let:attrs>
					<FormLabel>Track</FormLabel>
					<Select
						selected={selectedTrack}
						onSelectedChange={(v) => {
							v && ($formData.trackId = v.value);
						}}
					>
						<SelectTrigger {...attrs}>
							<SelectValue placeholder="Select a track" />
						</SelectTrigger>
						<SelectContent>
							{#each data.tracks as track}
								<SelectItem value={track.id} label={track.name} />
							{/each}
						</SelectContent>
					</Select>
					<input type="hidden" name={attrs.name} value={$formData.trackId} />
				</FormControl>
				<FormFieldErrors />

				<FormButton type="submit">Save</FormButton>
			</FormField>
		</form>
		{#if $message}
			<div>{$message}</div>
		{/if}
	</div>
	<div>
		<div class="text-bold text-lg">Sessions</div>
		{#each data.event.sessions as session}
			<div>
				<a href={`/admin/sessions/${session.id}`}>{session.sessionType}</a>
			</div>
		{/each}
	</div>
</div>

<style>
</style>
