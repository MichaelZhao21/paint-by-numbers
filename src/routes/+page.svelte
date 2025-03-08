<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import init, { img_to_flat, test } from 'pbn';
	import Button from '../components/Button.svelte';
	import { goto } from '$app/navigation';
	import ImageUpload from '../components/ImageUpload.svelte';

	let files = $state<FileList | null>(null);
	let fn = $derived(files ? files[0].name : '');
	let src = $state<string>();
	let colors = $state<string>('10');
	let minArea = $state<string>('20');
	let loading = $state<boolean>(false);

	async function convertFilesInner() {
		if (!files) {
			return;
		}

		// Make sure color is right
		const c = parseInt(colors);
		if (isNaN(c) || c < 1) {
			alert('Colors must be a positive integer');
			return;
		}
		if (c >= 100) {
			alert('Colors must be less than 100');
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

		loading = false;
	}

	function convertFile() {
		if (!files) {
			return;
		}

		loading = true;
		convertFilesInner();
	}

	async function download() {
		if (!src) {
			return;
		}

		const a = document.createElement('a');
		a.href = src;
		a.download = `${fn?.split('.')[0]}_flat.png`;
		a.click();
	}

	async function startPaint() {
		// Navigate to paint page
		goto('/paint');
	}

	onMount(async () => {
		// Load web assembly module
		await init();

		// Clear file input
		files = null;

		console.log(test());
	});

	onDestroy(() => {
		if (src) URL.revokeObjectURL(src);
	});
</script>

<div class="flex flex-col items-center px-4">
	<h1 class="mt-8 text-center text-4xl font-bold">Image to Paint by Number!</h1>
	<p class="my-4 text-center text-slate-700">
		Upload an image and convert it to a paint by number image!
	</p>
	<!-- <p class="my-4 text-center text-slate-700">
		Generate a painting here! Once you are happy with your result, download your painting and click
		"Start Painting"
	</p> -->
	<div
		class="mb-4 flex flex-col flex-wrap items-center gap-y-2 rounded-lg bg-white p-4 drop-shadow-md"
	>
		<ImageUpload bind:files />
		<div class="flex flex-wrap gap-x-4 gap-y-2">
			<div class="flex">
				<label for="colors" class="pr-1 font-bold">Colors:</label>
				<input
					id="colors"
					type="text"
					bind:value={colors}
					class="w-8 rounded-sm border-2 px-1 duration-150 outline-none hover:border-purple-300 focus:border-purple-300"
				/>
			</div>
			<div class="flex">
				<label for="minArea" class="pr-1 font-bold">Min Area:</label>
				<input
					id="minArea"
					type="text"
					bind:value={minArea}
					class="w-8 rounded-sm border-2 px-1 duration-150 outline-none hover:border-purple-300 focus:border-purple-300"
				/>
			</div>
		</div>
	</div>
	<div class="flex flex-row flex-wrap gap-4">
		<Button text="Convert" handleClick={convertFile} disabled={files === null} />
		{#if src && src !== ''}
			<Button text="Download" handleClick={download} />
			<Button text="Start Painting" handleClick={startPaint} />
		{/if}
	</div>
	{#if loading}
		<div
			class="bg-opacity-50 fixed top-0 left-0 z-10 flex h-full w-full items-center justify-center backdrop-blur-lg"
		>
			<div class="h-32 w-32 animate-spin rounded-full border-t-4 border-purple-300"></div>
		</div>
	{/if}

	<!-- Frame for image -->
	{#if src && src !== ''}
		<div class="mt-4 mb-8">
			<img
				{src}
				alt="converted flattened"
				class="rounded-md bg-white object-contain text-center drop-shadow-md"
				height={800}
				width={800}
			/>
		</div>
	{/if}
</div>
