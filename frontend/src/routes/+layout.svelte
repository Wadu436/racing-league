<script>
	import '../app.css';
</script>

<div class="w-screen h-screen flex flex-col bg-gray-50">
	<nav class="p-4 bg-primary-700">
		<a class="text-gray-50 hover:text-gray-200 hover:underline" href="/">Home</a>
	</nav>

	<slot />
</div>
