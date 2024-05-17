<script lang="ts">
	import { superForm } from 'sveltekit-superforms';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select/index.js';
	import { capitalize } from '$lib/util.js';
	import FormField from '$lib/components/ui/form/form-field.svelte';
	import {
		FormButton,
		FormControl,
		FormFieldErrors,
		FormLabel
	} from '$lib/components/ui/form/index.js';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import dayjs from 'dayjs';
	import { buttonVariants } from '$lib/components/ui/button';
	import { enhance as kitEnhance } from '$app/forms';

	import ReorderList from '$lib/components/reorder-list.svelte';

	export let data;
	export let form;

	let supaForm = superForm(data.form, { resetForm: false });
	let { form: formData, enhance, message } = supaForm;

	$: selectedStatus = $formData.status
		? {
				label: capitalize($formData.status),
				value: $formData.status
			}
		: undefined;

	$: gameDict = Object.fromEntries(data.games.map((g) => [g.id, g]));
	$: selectedGame = $formData.gameId
		? {
				value: $formData.gameId,
				label: gameDict[$formData.gameId]?.name ?? $formData.gameId
			}
		: undefined;

	function compareArrays<T extends { id: string }>(a: T[], b: T[]): boolean {
		return a.length === b.length && a.every((el, i) => el.id === b[i].id);
	}

	let draggableEvents: typeof data.events = [...data.events];
	$: draggableEventsSorted = draggableEvents.toSorted((a, b) => {
		if (a.date && b.date) {
			return a.date.getTime() - b.date.getTime();
		} else if (a.date) {
			return -1;
		} else if (b.date) {
			return 1;
		} else {
			return 0;
		}
	});

	$: eventsTainted = !compareArrays(data.events, draggableEvents);
</script>

<div class="mx-4">
	<nav>
		<div class="text-xl font-bold">{data.league.name}</div>
	</nav>

	<Separator class="my-4" />

	<div>
		<div class="gap-4 md:flex">
			<div>
				<div class="text-lg font-bold">League Info</div>
				<div>
					<form method="POST" action="?/save" use:enhance>
						<FormField form={supaForm} name="name">
							<FormControl let:attrs>
								<FormLabel>Name</FormLabel>
								<Input {...attrs} bind:value={$formData.name} />
							</FormControl>
							<FormFieldErrors />
						</FormField>

						<FormField form={supaForm} name="status">
							<FormControl let:attrs>
								<FormLabel>Status</FormLabel>
								<Select.Root
									selected={selectedStatus}
									onSelectedChange={(v) => {
										v && ($formData.status = v.value);
									}}
								>
									<Select.Trigger {...attrs}>
										<Select.Value />
									</Select.Trigger>
									<Select.Content>
										<Select.Item value="upcoming">Upcoming</Select.Item>
										<Select.Item value="ongoing">Ongoing</Select.Item>
										<Select.Item value="finished">Finished</Select.Item>
									</Select.Content>
								</Select.Root>
								<input hidden bind:value={$formData.status} name={attrs.name} />
							</FormControl>
							<FormFieldErrors />
						</FormField>

						<FormField form={supaForm} name="gameId">
							<FormControl let:attrs>
								<FormLabel>Game</FormLabel>
								<Select.Root
									selected={selectedGame}
									onSelectedChange={(v) => {
										v && ($formData.gameId = v.value);
									}}
								>
									<Select.Trigger>
										<Select.Value placeholder="Select a game" />
									</Select.Trigger>
									<Select.Content>
										<Select.Item value="foobar" label="fakegame >:(" />
										{#each data.games as game (game.id)}
											<Select.Item value={game.id} label={game.name} />
										{/each}
									</Select.Content>
								</Select.Root>
								<input hidden bind:value={$formData.gameId} name={attrs.name} />
							</FormControl>
							<FormFieldErrors />
						</FormField>

						<FormButton>Save</FormButton>
					</form>
					{#if $message}
						<div>{$message}</div>
					{/if}
				</div>
			</div>
			<div>
				<form method="post" action="?/saveOrder" use:kitEnhance>
					<div class="p-2">
						<div class="flex items-center justify-between">
							<div class="text-lg font-bold">Events</div>
							<div>
								<Button
									disabled={!eventsTainted}
									class={buttonVariants({ variant: 'secondary' })}
									on:click={() => {
										draggableEvents = [...data.events];
									}}>Reset order</Button
								>
								<FormButton disabled={!eventsTainted}>Save event order</FormButton>
								<Button
									disabled={compareArrays(draggableEvents, draggableEventsSorted)}
									on:click={() => {
										draggableEvents = draggableEventsSorted;
									}}>Order by date</Button
								>
							</div>
						</div>
						{#if form?.reorderMessage}
							<div>{form?.reorderMessage}</div>
						{/if}
					</div>
					<div>
						<ReorderList bind:items={draggableEvents}>
							<div slot="item" let:item={event} let:i class="select-none1">
								<a href="/admin/events/{event.id}" class="flex flex-1 justify-between gap-8">
									<div class="min-w-0 items-center overflow-hidden text-ellipsis whitespace-nowrap">
										<span class="font-bold">{event.name}</span>
										<span> - </span>
										<span>{event.track.name}</span>
									</div>
									<span class="min-w-fit"
										>{event.date ? dayjs(event.date).format('MMMM D, YYYY HH:mm') : 'TBD'}</span
									>
								</a>
								<input id={event.id} name={event.id} value={i} type="hidden" />
							</div>
						</ReorderList>
					</div>
				</form>
			</div>
		</div>
	</div>
</div>

<style>
</style>
