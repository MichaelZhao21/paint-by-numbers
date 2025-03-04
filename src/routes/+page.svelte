<script lang="ts">
	import { onMount } from 'svelte';
	import init, { img_to_flat, test } from 'pbn';

	let files = $state<FileList | null>();

	async function convertFile() {
		if (files) {
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
			const img = URL.createObjectURL(new Blob([result], { type: 'image/png' }));

			console.log(img);

			// Display the image
			const imgElement = document.createElement('img');
			imgElement.src = img;
			document.body.appendChild(imgElement);

			// Clean up
			URL.revokeObjectURL(img);
		}
	}

	onMount(async () => {
		// Load web assembly module
		await init();

		console.log(test());
	});
</script>

<h1>hewwo</h1>

<input type="file" bind:files onchange={convertFile} />
