<script>
	import '../app.css';
</script>

<div class="w-screen h-screen flex flex-col bg-gray-50 overflow-auto p-4">
	<nav class="p-4 bg-primary-700 rounded-lg">
		<a class="text-gray-50 hover:text-gray-200 hover:underline" href="/">Home</a>
	</nav>

	<main class="flex-grow">
		<slot />
	</main>
</div>
