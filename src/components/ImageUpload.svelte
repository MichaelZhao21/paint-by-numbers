<script lang="ts">
	let { files = $bindable() }: { files: FileList | null } = $props();
	let fn = $derived(files && files.length > 0 ? files[0].name : '');
</script>

<label for="image-upload" class="cursor-pointer p-1 hover:underline focus:underline">
	{fn === '' ? 'Click to upload a file' : `${fn} (click to change)`}
</label>
<input
	type="file"
	id="image-upload"
	accept="image/*"
	class="absolute z-[-1] opacity-0"
	bind:files={files}
/>
