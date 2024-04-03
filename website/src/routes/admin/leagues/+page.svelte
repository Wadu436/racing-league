<script lang="ts">
	import Crud from '$lib/component/Crud.svelte';
	import { superForm } from 'sveltekit-superforms';
	import type { PageData } from './$types';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Select, SelectValue } from '$lib/components/ui/select';
	import SelectTrigger from '$lib/components/ui/select/select-trigger.svelte';
	import SelectContent from '$lib/components/ui/select/select-content.svelte';
	import SelectItem from '$lib/components/ui/select/select-item.svelte';

	export let data: PageData;

	const form = superForm(data.form);
	const { form: formData, enhance, message } = form;

	$: gameDict = Object.fromEntries(data.games.map((g) => [g.id, g]));
	$: selectedGame = $formData.gameId
		? {
				value: $formData.gameId,
				label: gameDict[$formData.gameId]?.name ?? $formData.gameId
			}
		: undefined;
</script>

<Crud
	shape={{
		columns: [
			{ name: 'name', label: 'Name', type: 'string' },
			{ name: 'status', label: 'Status', type: 'string' },
			{ name: 'game', label: 'Game', type: 'string' }
		],
		data: data.leagues.map((l) => ({
			id: l.id,
			name: l.name,
			status: l.status,
			game: l.game?.name
		})),
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

				<Form.Field {form} name="gameId">
					<Form.Control let:attrs>
						<Form.Label>Game</Form.Label>
						<Select
							selected={selectedGame}
							onSelectedChange={(v) => {
								v && ($formData.gameId = v.value);
							}}
						>
							<SelectTrigger>
								<SelectValue placeholder="Select a game" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="foobar" label="fakegame >:(" />
								{#each data.games as game (game.id)}
									<SelectItem value={game.id} label={game.name} />
								{/each}
							</SelectContent>
						</Select>
						<input hidden bind:value={$formData.gameId} name={attrs.name} />
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
