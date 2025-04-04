<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import init, { img_to_flat } from 'pbn';
	import Button from '../components/Button.svelte';
	import { goto } from '$app/navigation';
	import ImageUpload from '../components/ImageUpload.svelte';
	import Card from '../components/Card.svelte';
	import TextField from '../components/TextField.svelte';
	import Loading from '../components/Loading.svelte';

	let files = $state<FileList | null>(null);
	let fn = $derived(files ? files[0].name : '');
	let src = $state<string>();
	let colors = $state<string>('10');
	let minArea = $state<string>('20');
	let name = $state<string>('');
	let savedList = $state<string[]>([]);
	let loading = $state<boolean>(false);
	let outBlob = $state<Blob | null>(null);

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
		const file = files[0];

		// Set name to filename
		fn.replace(/\.[^/.]+$/, '');
		console.log(name);

		// Extract file content as blob
		// and convert to vec of u8s
		const blob = await file.arrayBuffer();
		const u8s = new Uint8Array(blob);

		// Call the wasm function
		const result = img_to_flat(u8s, c, ma);

		// Convert result back to image
		outBlob = new Blob([result], { type: 'image/png' });
		src = URL.createObjectURL(outBlob);

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

	async function save() {
		if (!outBlob) {
			return;
		}

		// Save the file to local storage
		const rawSavedList = localStorage.getItem('files');
		const savedList = rawSavedList ? rawSavedList.split(',') : [];
		if (savedList.includes(name)) {
			let overwrite = confirm('Painting with same name already saved, overwrite?');
			if (!overwrite) return;
		}
		savedList.push(name);
		localStorage.setItem('files', savedList.join(','));

		// Save the file to the OPFS
		const dir = await navigator.storage.getDirectory();
		const fileHandler = await dir.getFileHandle(name, { create: true });
		const writable = await fileHandler.createWritable();
		await writable.write(outBlob);
		await writable.close();
	}

	async function startPaint() {
		// Check to see if the file is saved
		const rawSavedList = localStorage.getItem('files');
		const savedList = rawSavedList ? rawSavedList.split(',') : [];
		if (!savedList.includes(name)) {
			await save();
		}

		// Navigate to paint page
		goto(`/paint?name=${encodeURIComponent(name)}`);
	}

	$effect(() => {
		if (!files) return;

		name = fn.replace(/\.[^/.]+$/, '');
	});

	onMount(async () => {
		loading = true;

		// Load web assembly module
		await init();

		// Clear file input
		files = null;

		// Get the list of files from local storage
		const rawSavedList = localStorage.getItem('files');
		if (!rawSavedList) {
			loading = false;
			return;
		}
		savedList = rawSavedList.split(',');

		loading = false;
	});

	onDestroy(() => {
		if (src) URL.revokeObjectURL(src);
	});
</script>

<div class="flex flex-col items-center px-4">
	<h1 class="mt-8 text-center text-4xl font-bold">Image to Paint by Number!</h1>
	<p class="my-4 max-w-lg text-center text-slate-700">
		Generate a painting from an image. Once you are happy with your result, download your painting,
		save your painting to your browser, or click "Start Painting" to start painting immediately!
	</p>
	<Card>
		<ImageUpload bind:files />
		<div class="flex flex-wrap gap-x-4 gap-y-2">
			<TextField label="Colors" bind:value={colors} placeholder="10" className="w-8" />
			<TextField label="Min Area" bind:value={minArea} placeholder="20" className="w-8" />
		</div>
		<TextField label="Name" bind:value={name} placeholder="name painting" className="w-36" />
	</Card>
	<div class="flex flex-row flex-wrap gap-4">
		<Button text="Convert" handleClick={convertFile} disabled={files === null} />
		{#if src && src !== ''}
			<Button text="Download" handleClick={download} />
			<Button text="Save" handleClick={save} />
			<Button text="Start Painting" handleClick={startPaint} />
		{/if}
	</div>
	<Loading bind:loading />

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
