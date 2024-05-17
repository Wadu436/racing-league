<script context="module" lang="ts">
	type Item = {
		id: string;
		[SHADOW_ITEM_MARKER_PROPERTY_NAME]?: boolean | undefined;
	} & Record<string, unknown>;
</script>

<script lang="ts" generics="T extends Item">
	import { GripVertical } from 'lucide-svelte';

	import { cubicIn } from 'svelte/easing';
	import { fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	import { dndzone, SHADOW_ITEM_MARKER_PROPERTY_NAME, SOURCES, TRIGGERS } from 'svelte-dnd-action';
	import { cn } from '$lib/utils';

	export let items: T[];
	export let flipDurationMs = 200;

	// Stuff for the handle
	let dragDisabled = true;
</script>

<section
	class="grid gap-2"
	use:dndzone={{ items, flipDurationMs, dragDisabled }}
	on:consider={(e) => {
		items = e.detail.items;
		// Ensure dragging is stopped on drag finish via keyboard
		if (
			e.detail.info.source === SOURCES.KEYBOARD &&
			e.detail.info.trigger === TRIGGERS.DRAG_STOPPED
		) {
			dragDisabled = true;
		}
	}}
	on:finalize={(e) => {
		items = e.detail.items;
		// Ensure dragging is stopped on drag finish via pointer (mouse, touch)
		if (e.detail.info.source === SOURCES.POINTER) {
			dragDisabled = true;
		}
	}}
>
	{#each items as item, i (item.id)}
		<div class="relative" animate:flip={{ duration: flipDurationMs }}>
			<div class="flex items-center rounded-lg border p-4">
				<div
					role="none"
					class={cn('mr-2', dragDisabled ? 'cursor-grab' : 'cursor-grabbing')}
					on:mousedown={(e) => {
						e.preventDefault();
						dragDisabled = false;
					}}
					on:touchstart={(e) => {
						e.preventDefault();
						dragDisabled = false;
					}}
					on:keydown={(e) => {
						if ((e.key === 'Enter' || e.key === ' ') && dragDisabled) {
							dragDisabled = false;
						}
					}}
				>
					<GripVertical />
				</div>
				<div class="flex-grow">
					<slot name="item" {item} {i} />
				</div>
			</div>
			{#if item[SHADOW_ITEM_MARKER_PROPERTY_NAME]}
				<div
					in:fade={{ duration: flipDurationMs, easing: cubicIn }}
					class="visible absolute bottom-0 left-0 right-0 top-0 m-0 rounded-lg border border-dashed"
				></div>
			{/if}
		</div>
	{/each}
</section>

<style>
</style>
