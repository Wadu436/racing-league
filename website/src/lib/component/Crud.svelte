<script context="module" lang="ts">
	type KeysOfType<T, U> = Extract<keyof T, { [K in keyof T]: T[K] extends U ? K : never }[keyof T]>;

	type ColumnShapeString<T> = {
		name: string & KeysOfType<T, string>;
		label: string;
		type: 'string' | 'country';
	};
	type ColumnShapeBoolean<T> = {
		name: string & KeysOfType<T, boolean>;
		label: string;
		type: 'checkbox';
	};
	type ColumnShape<T> = ColumnShapeBoolean<T> | ColumnShapeString<T>;

	type DataShape = { id: string } & Record<string, string | boolean | null>;

	export type CrudShape<T extends { id: string }> = {
		columns: ColumnShape<T>[];
		data: T[];
		edit?: T;
		creatable?: boolean;
	} & ({ creatable?: false; create?: never } | { creatable: true; create: boolean });
</script>

<script lang="ts" generics="T extends DataShape">
	import { countryCodeAlpha2Dict } from '$lib/flags/countryCodes';

	import { enhance } from '$app/forms';

	import { goto } from '$app/navigation';
	import { CheckCircle2, PlusCircle, XCircle } from 'lucide-svelte';
	import Flag from '$lib/flags/Flag.svelte';
	import CountryPicker from './CountryPicker.svelte';

	export let shape: CrudShape<T>;
	export let baseUrl: string;
	export let errorMessage: string | undefined;

	// $: transformedColumns =
</script>

<div class="flex gap-2">
	<nav class="table-container">
		<div class="flex items-start">
			<table class="table-interactive table">
				<thead>
					<tr>
						{#each shape.columns as column}
							<th>{column.label}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each shape.data as row}
						<tr
							on:click={() => {
								goto(`${baseUrl}${row.id}`);
							}}
						>
							{#each shape.columns as column}
								{@const value = row[column.name]}
								{#if value != null && typeof value === 'string'}
									{#if column.type === 'string'}
										<td>{value}</td>
									{:else if column.type === 'country'}
										<td><Flag size="m" alpha2={value} /><span class="ml-2">{countryCodeAlpha2Dict[value]?.countryName}</span></td>
									{:else}
										<td></td>
									{/if}
								{:else if value != null && typeof value === 'boolean'}
									{#if column.type === 'checkbox'}
										<td>
											{#if value}
												<CheckCircle2 class="text-success-800" />
											{:else}
												<XCircle class="text-error-800" />
											{/if}
										</td>
									{:else}
										<td></td>
									{/if}
								{:else}
									<td></td>
								{/if}
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
			{#if shape.creatable}
				<a href="{baseUrl}?create" class="btn mt-3"><PlusCircle /></a>
			{/if}
		</div>
	</nav>
	{#if (shape.creatable && shape.create) || shape.edit}
		<div class="card min-w-96 h-fit flex-shrink-0 p-4">
			{#if shape.creatable && shape.create}
				<form method="post" action="?/create" use:enhance>
					<div class="grid auto-rows-fr grid-cols-[auto_auto] items-center gap-x-4 gap-y-2">
						{#each shape.columns as column}
							<label for={column.name} class="label">{column.label}</label>
							{#if column.type === 'string'}
								<input id={column.name} name={column.name} class="input" type="text" value={''} />
							{:else if column.type === 'country'}
								<CountryPicker id={column.name} name={column.name} value={''}/>
							{:else if column.type === 'checkbox'}
								<!-- The == true is needed because Typescript is dumb -->
								<input
									id={column.name}
									name={column.name}
									class="checkbox"
									type="checkbox"
									checked={false}
								/>
							{/if}
						{/each}
					</div>
					<div class="mt-2 flex items-center justify-between">
						<div class="text-error-700 p-2">
							{#if errorMessage}
								{errorMessage}
							{/if}
						</div>
						<div>
							<button class="btn variant-filled" type="submit">Create</button>
							<button class="btn variant-filled" type="submit" formaction="?/create_another"
								>Create and add another</button
							>
						</div>
					</div>
				</form>
			{:else if shape.edit}
				<form
					method="post"
					action="?/save"
					use:enhance={() =>
						({ update }) =>
							update({ reset: false })}
				>
					<div class="grid auto-rows-fr grid-cols-[auto_auto] items-center gap-x-4 gap-y-2">
						{#each shape.columns as column}
							<label for={column.name} class="label">{column.label}</label>
							{#if column.type === 'string'}
								<input
									id={column.name}
									name={column.name}
									class="input"
									type="text"
									value={shape.edit[column.name]}
								/>
							{:else if column.type === 'country'}
								<CountryPicker
									id={column.name}
									name={column.name}
									value={shape.edit[column.name]?.toString()}
								/>
							{:else if column.type === 'checkbox'}
								<!-- The == true is needed because Typescript is dumb -->
								<input
									id={column.name}
									name={column.name}
									class="checkbox"
									type="checkbox"
									checked={shape.edit[column.name] == true}
								/>
							{/if}
						{/each}
					</div>
					<div class="mt-2 flex items-center justify-between">
						<div class="text-error-700 p-2">
							{#if errorMessage}
								{errorMessage}
							{/if}
						</div>
						<div>
							<button class="btn variant-filled" type="submit">Save</button>
						</div>
					</div>
				</form>
			{/if}
		</div>
	{/if}
</div>

<style>
</style>
