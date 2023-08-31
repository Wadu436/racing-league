<script lang="ts">
	import { page } from '$app/stores';

	const messageMap: Record<number, string> = {
		404: 'lost on track',
		500: 'the server binned it'
	};

	$: message = messageMap[$page.status] || $page.error?.message;
</script>

<div class="flex flex-col flex-grow h-full items-center justify-center select-none">
	<div class="text-7xl font-light">{message}</div>
	<div class="text-3xl font-light">status code: {$page.status}</div>
</div>
