<script lang="ts">
	import GoogleOauthPill from '$lib/icons/GoogleOauthPill.svelte';
	
	export let data;
</script>

<form method="post">
	<input type="hidden" id="next" name="next" value="{data.next}"> 
	<button type="submit" formaction="?/google"><GoogleOauthPill /></button>
</form>
