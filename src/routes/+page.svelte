<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import init, { img_to_flat, test } from 'pbn';

	let files = $state<FileList | null>();
	let fn = $state<string>();
	let src = $state<string>();

	$effect(() => {
		fn = files ? files[0].name : '';
	});

	async function convertFile() {
		if (!files) {
			return;
		}

		// Note that `files` is of type `FileList`, not an Array:
		// https://developer.mozilla.org/en-US/docs/Web/API/FileList
		console.log(files);

		const file = files[0];

		// Extract file content as blob
		const blob = await file.arrayBuffer();

		// Convert to vec of u8s
		const u8s = new Uint8Array(blob);

		// Call the wasm function
		const result = img_to_flat(u8s, 10, 20);

		// Convert result back to image
		src = URL.createObjectURL(new Blob([result], { type: 'image/png' }));
	}

	onMount(async () => {
		// Load web assembly module
		await init();

		console.log(test());
	});

	onDestroy(() => {
		if (src) URL.revokeObjectURL(src);
	});
</script>

<div class="flex flex-col items-center">
	<h1 class="my-4 text-4xl font-bold">Image to Paint by Number!</h1>
	<div class="mb-4 flex flex-row items-center">
		<label for="image-upload" class="cursor-pointer p-1">
			{fn === '' ? 'Upload a file' : `${fn} (click to change)`}
		</label>
		<input type="file" id="image-upload" class="absolute z-[-1] opacity-0" bind:files />
		<button onclick={convertFile} class="ml-4 cursor-pointer rounded-md border-2 px-2 py-1">
			Convert
		</button>
	</div>

	<!-- Frame for image -->
	<div>
		<img {src} alt="converted flattened" class="max-h-1/2 rounded-md border-2 border-black" />
	</div>
</div>
