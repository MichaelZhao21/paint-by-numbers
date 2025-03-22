<script lang="ts">
	let { loading = $bindable() }: { loading: boolean } = $props();
</script>

{#if loading}
	<div
		class="bg-opacity-50 fixed top-0 left-0 z-10 flex h-full w-full items-center justify-center backdrop-blur-lg"
	>
		<div class="h-32 w-32 animate-spin rounded-full border-t-4 border-purple-300"></div>
	</div>
{/if}
