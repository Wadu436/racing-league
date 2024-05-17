<script lang="ts" context="module">
	type T = Record<string, unknown>;
</script>

<script lang="ts" generics="T extends Record<string, unknown>">
	import Button from './ui/button/button.svelte';

	import Input from './ui/input/input.svelte';

	import {
		today,
		type DateValue,
		getLocalTimeZone,
		parseDate,
		CalendarDate,
		fromDate,
		DateFormatter,
		now
	} from '@internationalized/date';

	import * as Form from '$lib/components/ui/form';
	import * as Popover from '$lib/components/ui/popover';
	import { Calendar } from '$lib/components/ui/calendar';
	import { fieldProxy, type FieldProxy, type FormPath, type SuperForm } from 'sveltekit-superforms';
	import { CalendarIcon } from 'lucide-svelte';
	import { buttonVariants } from './ui/button';
	import { cn } from '$lib/utils';

	export let form: SuperForm<T>;
	export let name: FormPath<T, Date | undefined | null>;
	export let label: string;

	const formValue = fieldProxy(form, name) satisfies FieldProxy<Date | undefined | null>;

	let value: DateValue | null;

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	$: value = $formValue ? fromDate($formValue, getLocalTimeZone()) : null;

	let hour: string | undefined;
	let minute: string | undefined;

	$: hour = $formValue?.getHours().toString().padStart(2, '0');
	$: minute = $formValue?.getMinutes().toString().padStart(2, '0');

	$formValue?.setSeconds(0);

	let placeholder: DateValue = today(getLocalTimeZone());
</script>

<Form.Field {form} {name} class="flex flex-col">
	<Form.Control let:attrs>
		<Form.Label>{label}</Form.Label>
		<div class="flex gap-8">
			<Popover.Root>
				<Popover.Trigger
					{...attrs}
					class={cn(
						buttonVariants({ variant: 'outline' }),
						'w-[280px] justify-start pl-4 text-left font-normal',
						!value && 'text-muted-foreground'
					)}
				>
					{value ? df.format(value.toDate(getLocalTimeZone())) : 'Pick a date'}
					<CalendarIcon class="ml-auto h-4 w-4 opacity-50" />
				</Popover.Trigger>
				<Popover.Content class="w-auto p-0" side="top">
					<Calendar
						value={value ?? undefined}
						bind:placeholder
						minValue={new CalendarDate(1900, 1, 1)}
						calendarLabel="Date of birth"
						initialFocus
						onValueChange={(v) => {
							if (v) {
								console.log($formValue);
								if ($formValue) {
									console.log($formValue);
									$formValue.setDate(v.toDate(getLocalTimeZone()).getDate());
									$formValue = $formValue; // Needed because Svelte reactivity
								} else {
									$formValue = v.toDate(getLocalTimeZone());
									const n = now(getLocalTimeZone());
									$formValue.setHours(n.hour);
									$formValue.setMinutes(Math.round(n.minute / 5) * 5);
								}
							}
						}}
					/>
				</Popover.Content>
			</Popover.Root>
			<div class="flex items-center gap-2">
				<div class="max-w-20">
					<Input
						type="number"
						min="0"
						max="23"
						value={hour}
						disabled={$formValue === undefined}
						on:change={(v) => {
							v.currentTarget.value = v.currentTarget.value.padStart(2, '0');
							if ($formValue) {
								if (+v.currentTarget.value > 23) {
									$formValue.setHours(0);
								} else if (+v.currentTarget.value < 0) {
									$formValue.setHours(23);
								} else $formValue.setHours(+v.currentTarget.value);
								$formValue = $formValue; // Needed because Svelte reactivity
							}
						}}
					/>
				</div>
				<div class="text-md">:</div>
				<div class="max-w-20">
					<Input
						type="number"
						min="0"
						step="5"
						max="59"
						value={minute}
						disabled={$formValue === undefined}
						on:change={(v) => {
							v.currentTarget.value = v.currentTarget.value.padStart(2, '0');
							if ($formValue) {
								if (+v.currentTarget.value > 59) {
									$formValue.setMinutes(0);
								} else if (+v.currentTarget.value < 0) {
									$formValue.setMinutes(59);
								} else $formValue.setMinutes(+v.currentTarget.value);

								// $formValue.setMinutes(+v.currentTarget.value);
								$formValue = $formValue; // Needed because Svelte reactivity
							}
						}}
					/>
				</div>
			</div>
			<Button
				on:click={() => {
					value = null;
					$formValue = null;
				}}
			>
				Clear
			</Button>
		</div>

		<Form.FieldErrors />
	</Form.Control>
</Form.Field>

<style>
</style>
