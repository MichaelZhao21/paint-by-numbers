<script lang="ts">
	import init, { flat_to_svg } from 'pbn';
	import Button from '../../components/Button.svelte';
	import ImageUpload from '../../components/ImageUpload.svelte';
	import { onMount } from 'svelte';

	let files = $state<FileList | null>(null);
	let loaded = $state<boolean>(false);
	let shape = $state<string | null>(null);
	let colors = $state<string[]>([]);
	let active = $state<number | null>(0);
	let transX = 0;
	let transY = 0;

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
			function paint() {
				const label = document.getElementById(`label-${i}`);
				if (!label) return;
				const el = document.getElementById(`shape-${i}`);
				const numLabel = Number(label?.textContent);
				if (active === null || numLabel !== active + 1) return;
				const color = colors[numLabel - 1];

				el?.setAttribute('fill', color);
				el?.setAttribute('stroke', color);
				el?.classList.remove('unfilled');
				el?.removeEventListener('click', paint);
				label?.removeEventListener('click', paint);
				label?.remove();
			}
			document.getElementById(`shape-${i}`)?.addEventListener('click', () => {
				paint();
			});
			document.getElementById(`label-${i}`)?.addEventListener('click', () => {
				paint();
			});
		}

		// Create drag event listener
		document.addEventListener('mousedown', (e) => {
			let startX = e.clientX;
			let startY = e.clientY;

			function drag(e: MouseEvent) {
				const dx = e.clientX - startX;
				const dy = e.clientY - startY;
				transX += dx;
				transY += dy;
				startX = e.clientX;
				startY = e.clientY;

				const svg = document.querySelector('svg');
				if (!svg) return;

				svg.style.transform = `translate(${transX}px, ${transY}px)`;
			}

			function drop() {
				document.removeEventListener('mousemove', drag);
				document.removeEventListener('mouseup', drop);
			}

			document.addEventListener('mousemove', drag);
			document.addEventListener('mouseup', drop);
		});
	});

	function tooDark(color: string) {
		const hex = color.replace('#', '');
		const r = parseInt(hex.substring(0, 0 + 2), 16);
		const g = parseInt(hex.substring(2, 2 + 2), 16);
		const b = parseInt(hex.substring(4, 4 + 2), 16);
		const brightness = ((r * 299) + (g * 587) + (b * 114)) / 1000;
		return brightness <= 155;
	}
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
		<div class="fixed bottom-4 left-1/2 -translate-x-1/2">
			<div class="flex w-full gap-2 rounded-lg bg-white p-4 drop-shadow-md">
				{#each colors as color, i}
					<button
						class={`flex h-8 w-8 cursor-pointer items-center justify-center rounded-md ${
							i === active ? 'outline-4 outline-purple-400' : 'outline-4 outline-white'
						} ${tooDark(color) ? 'text-white' : 'text-black'}`}
						onclick={() => (active = i)}
						style="background-color: {color}"
					>
						{i + 1}
					</button>
				{/each}
			</div>
		</div>
	</div>
{/if}

<style>
	:global(path) {
		background-color: #ffffff;
	}

	:global(.unfilled) {
		cursor: pointer;
	}

	:global(text) {
		fill: oklch(0.714 0.203 305.504);
		font-weight: bold;
		font-family: 'Inter', sans-serif;
		cursor: pointer;

		user-select: none;
		-moz-user-select: -moz-none;
		-khtml-user-select: none;
		-webkit-user-select: none;
	}
</style>
