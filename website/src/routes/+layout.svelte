<script lang="ts">
	import '../app.pcss';
	import type { LayoutData } from './$types';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ModeWatcher } from 'mode-watcher';
	import LightSwitch from '$lib/components/light-switch.svelte';
	export let data: LayoutData;
</script>

<ModeWatcher />
<!-- Header -->
<div class="flex h-screen w-screen flex-col bg-background">
	<header class="flex-none border-b p-2">
		<div class="container flex max-w-screen-2xl items-center">
			<div class="mx-6 flex gap-4">
				<div class="font-bold">
					<a href="/">racing.warre.dev</a>
				</div>
				{#if data.user?.staff || data.user?.admin}
					<a class="anchor" href="/admin">admin</a>
				{/if}
			</div>

			<!-- Account -->
			<div class="flex flex-1 items-center justify-end gap-4">
				<!-- TODO: make this work better >:( -->
				{#if data.user}
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>{data.user.username}</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item><a class="w-full" href="/profile">Profile</a></DropdownMenu.Item>
							<DropdownMenu.Item
								><form class="w-full" method="post">
									<button class="" type="submit" formaction="/auth?/signout">Sign out</button>
								</form></DropdownMenu.Item
							>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				{:else}
					<a class="anchor" href="/auth/signin">Sign in</a>
				{/if}
				<LightSwitch />
			</div>
		</div>
	</header>

	<div class="flex h-full w-full flex-auto overflow-hidden pt-4">
		<div
			class="container flex flex-1 flex-col overflow-x-hidden"
			style:scrollbar-gutter="auto"
			on:scroll
		>
			<main class=" flex-auto"><slot /></main>

			<!-- Footer -->
			<footer>
				<p>TODO: Footer</p>
			</footer>
		</div>
	</div>
</div>
