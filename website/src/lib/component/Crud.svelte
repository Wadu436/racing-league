<script context="module" lang="ts">
	type KeysOfType<T, U> = Extract<keyof T, { [K in keyof T]: T[K] extends U ? K : never }[keyof T]>;

	type ColumnShapeString<T> = {
		name: string & KeysOfType<T, string|undefined>;
		label: string;
		type: 'string' | 'country';
	};
	type ColumnShapeBoolean<T> = {
		name: string & KeysOfType<T, boolean|undefined>;
		label: string;
		type: 'checkbox';
	};
	type ColumnShape<T> = ColumnShapeBoolean<T> | ColumnShapeString<T>;

	type DataShape = { id: string } & Record<string, string | boolean | null | undefined>;

	export type CrudShape<T extends { id: string }> = {
		columns: ColumnShape<T>[];
		data: T[];
		edit?: T;
		creatable?: boolean;
	} & ({ creatable?: false; create?: never } | { creatable: true; create: boolean });
</script>

<script lang="ts" generics="T extends DataShape">
	import { countryCodeAlpha2Dict } from '$lib/flags/countryCodes';

	import { goto } from '$app/navigation';
	import { CheckCircle2, PlusCircle, XCircle } from 'lucide-svelte';
	import Flag from '$lib/flags/Flag.svelte';

	export let shape: CrudShape<T>;
	export let baseUrl: string;

	import * as Table from '$lib/components/ui/table';
	import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte';
</script>

<div class="flex gap-2">
	<nav class="table-container border-r px-4">
		<div class="flex items-start">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						{#each shape.columns as column}
							<Table.Head>{column.label}</Table.Head>
						{/each}
					</Table.Row>
				</Table.Header>

				<Table.Body>
					{#each shape.data as row}
						<Table.Row
							on:click={() => {
								goto(`${baseUrl}${row.id}`);
							}}
							class="cursor-pointer"
							data-state={shape.edit?.id === row.id ? 'selected' : undefined}
						>
							{#each shape.columns as column}
								{@const value = row[column.name]}
								{#if value != null && typeof value === 'string'}
									{#if column.type === 'string'}
										<Table.Cell>{value}</Table.Cell>
									{:else if column.type === 'country'}
										<Table.Cell
											><Flag size="m" alpha2={value} /><span class="ml-2"
												>{countryCodeAlpha2Dict[value]?.countryName ?? 'World'}</span
											></Table.Cell
										>
									{:else}
										<Table.Cell></Table.Cell>
									{/if}
								{:else if value != null && typeof value === 'boolean'}
									{#if column.type === 'checkbox'}
										<Table.Cell>
											{#if value}
												<CheckCircle2 class="text-success" />
											{:else}
												<XCircle class="text-destructive" />
											{/if}
										</Table.Cell>
									{:else}
										<Table.Cell></Table.Cell>
									{/if}
								{:else}
									<Table.Cell></Table.Cell>
								{/if}
							{/each}
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
			{#if shape.creatable}
				<a href="{baseUrl}?create" class="mt-3 pl-4"><PlusCircle /></a>
			{/if}
		</div>
	</nav>
	<div class="card min-w-96 h-fit flex-shrink-0 flex-grow p-4">
		<slot name="form" />
	</div>
</div>

<style>
</style>
