<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import init, { img_to_flat, test } from 'pbn';

	let files = $state<FileList | null>();
	let fn = $state<string>();
	let src = $state<string>();
	let colors = $state<string>('10');
	let minArea = $state<string>('20');

	$effect(() => {
		fn = files ? files[0].name : '';
	});

	async function convertFile() {
		if (!files) {
			return;
		}

		// Make sure color is right
		const c = parseInt(colors);
		if (isNaN(c) || c < 1) {
			alert('Colors must be a positive integer');
			return;
		}

		// Make sure minArea is right
		const ma = parseInt(minArea);
		if (isNaN(ma) || ma < 1) {
			alert('Min Area must be a positive integer');
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
		const result = img_to_flat(u8s, c, ma);

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
	<div class="mb-2 flex flex-row items-center">
		<label for="image-upload" class="cursor-pointer p-1">
			{fn === '' ? 'Click to upload a file' : `${fn} (click to change)`}
		</label>
		<input type="file" id="image-upload" class="absolute z-[-1] opacity-0" bind:files />

		<label for="colors" class="pl-4 pr-1 font-bold">Colors:</label>
		<input id="colors" type="text" bind:value={colors} class="w-8" />

		<label for="minArea" class="pl-4 pr-1 font-bold">Min Area:</label>
		<input id="minArea" type="text" bind:value={minArea} class="w-8" />
	</div>
	<button onclick={convertFile} class="ml-4 cursor-pointer rounded-md border-2 px-2 py-1">
		Convert
	</button>

	<!-- Frame for image -->
	<div class="mt-4">
		<img
			{src}
			alt="converted flattened"
			class="rounded-md border-2 border-black object-contain text-center"
			height={800}
			width={800}
		/>
	</div>
</div>
