<script lang="ts">
	import init, { flat_to_svg } from 'pbn';
	import Button from '../../components/Button.svelte';
	import ImageUpload from '../../components/ImageUpload.svelte';
	import { onMount } from 'svelte';

	let files = $state<FileList | null>(null);
	let loaded = $state<boolean>(false);
	let shape = $state<string | null>(null);
    let colors = $state<string[]>([]);

	async function startPainting() {
		if (!files) {
			return;
		}

		const file = files[0];

		const blob = await file.arrayBuffer();
		const u8s = new Uint8Array(blob);

		const data = flat_to_svg(u8s);
        shape = data.svg;
        colors = data.colors;

		loaded = true;
	}

	onMount(async () => {
		// Load web assembly module
		await init();
	});

	$effect(() => {
        if (!shape) return;

		const count = shape.match(/<path/g)?.length || 0;

		// Create click event listeners for each path
		for (let i = 0; i < count; i++) {
			console.log(document.getElementById(`shape-${i}`));
			document.getElementById(`shape-${i}`)?.addEventListener('click', () => {
                const label = document.getElementById(`label-${i}`);
                if (!label) return;
                const el = document.getElementById(`shape-${i}`);
                const color = colors[Number(label?.textContent)];
                el?.setAttribute('fill', color);
                el?.setAttribute('stroke', color);
                label?.remove();
			});
		}
	});
</script>

{#if !loaded}
	<div class="flex flex-col items-center px-4">
		<h1 class="mt-8 text-center text-4xl font-bold">Paint by Number!</h1>
		<p class="my-4 text-center text-slate-700">
			Upload a converted image and start painting! If you would like to convert an image,
			<a href="/" class="text-purple-400 underline hover:text-purple-500">
				go to the previous page.
			</a>
		</p>
		<div
			class="mb-4 flex flex-col flex-wrap items-center gap-y-2 rounded-lg bg-white p-4 drop-shadow-md"
		>
			<ImageUpload bind:files />
			<Button text="Start Painting" handleClick={startPainting} disabled={!files} />
		</div>
	</div>
{/if}

{#if loaded}
	<div class="absolute">
		{@html shape}
	</div>
{/if}

<style>
    :global(path) {
        background-color: #ffffff;
    }

    :global(text) {
        fill: oklch(0.714 0.203 305.504);
        font-weight: bold;
        font-family: 'Inter', sans-serif;
    }
</style>