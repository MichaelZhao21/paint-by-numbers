<script lang="ts">
	import init, { flat_to_svg } from 'pbn';
	import { onMount, untrack } from 'svelte';
	import { page } from '$app/state';
	import Loading from '../../components/Loading.svelte';
	import { goto } from '$app/navigation';
	import HamburgerIcon from '../../components/HamburgerIcon.svelte';
	import CloseIcon from '../../components/CloseIcon.svelte';
	import Button from '../../components/Button.svelte';

	let loading = $state<boolean>(true);
	let shape = $state<string | null>(null);
	let colors = $state<string[]>([]);
	let active = $state<number | null>(0);
	let info = $state<number | null>(null);
	let infoTimeout = $state<number | null>(null);
	let painted = $state<boolean[]>([]);
	let name = $state<string | null>(null);
	let count = $state<number>(0);
	let menuOpen = $state<boolean>(false);
	let touches = 0;
	let transX = 0;
	let transY = 0;
	let zoom = 1;

	onMount(async () => {
		// Load web assembly module
		await init();

		// Get image url
		name = page.url.searchParams.get('name');
		if (!name) return;

		// Check to see if image in local storage
		const rawSavedList = localStorage.getItem('files');
		const savedList = rawSavedList ? rawSavedList.split(',') : [];
		if (!savedList.includes(name)) {
			alert('Image not found in local storage');
			goto('/');
			return;
		}

		// Load image from OPFS
		const dir = await navigator.storage.getDirectory();
		const fileHandler = await dir.getFileHandle(name, { create: false });
		const file = await fileHandler.getFile();
		const blob = await file.arrayBuffer();

		// Convert image to SVG
		const u8s = new Uint8Array(blob);
		const data = flat_to_svg(u8s);
		shape = data.svg;
		colors = data.colors;

		count = shape.match(/<path/g)?.length || 0;

		// Load in painted paths
		const rawPaintedList = localStorage.getItem(`painted-${name}`);
		if (rawPaintedList) {
			painted = rawPaintedList.split('').map((v) => v === '1');
		} else {
			painted = new Array(count).fill(false);
		}

		loading = false;
	});

	$effect(() => {
		if (!shape) return;

		untrack(() => {
			// Create click event listeners for each path
			for (let i = 0; i < count; i++) {
				function paint(ignoreActive = false) {
					// Check label
					const label = document.getElementById(`label-${i}`);
					if (!label) return;
					const el = document.getElementById(`shape-${i}`);
					if (!el) return;
					const numLabel = Number(label?.textContent);
					if (active === null) return;
					if (!ignoreActive && numLabel !== active + 1) {
						if (infoTimeout) {
							clearTimeout(infoTimeout);
							infoTimeout = null;
						}
						info = numLabel;
						infoTimeout = setTimeout(() => {
							info = null;
							infoTimeout = null;
						}, 2000);
						return;
					}
					const color = colors[numLabel - 1];

					el.setAttribute('fill', color);
					el.setAttribute('stroke', color);
					el.classList.remove('unfilled');
					el.removeEventListener('click', paint as any);
					label.removeEventListener('click', paint as any);
					label.remove();
					painted[i] = true;
					localStorage.setItem(`painted-${name}`, painted.map((p) => (p ? '1' : '0')).join(''));
				}
				document.getElementById(`shape-${i}`)?.addEventListener('click', () => {
					paint();
				});
				document.getElementById(`label-${i}`)?.addEventListener('click', () => {
					paint();
				});
				if (painted[i]) {
					paint(true);
				}
			}

			// Transform SVG to center
			const svg = document.querySelector('svg');
			if (!svg) return;
			const { width, height } = svg.getBBox();
			if (Math.abs(height) < 3000 && Math.abs(width) < 3000) {
				transX = window.innerWidth / 2 - width / 2;
				transY = window.innerHeight / 2 - height / 2;
				svg.style.transform = `translate(${transX}px, ${transY}px) scale(${zoom})`;
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

					svg.style.transform = `translate(${transX}px, ${transY}px) scale(${zoom})`;
				}

				function drop() {
					document.removeEventListener('mousemove', drag);
					document.removeEventListener('mouseup', drop);
				}

				document.addEventListener('mousemove', drag);
				document.addEventListener('mouseup', drop);
			});

			// Create zoom event listener
			document.addEventListener('wheel', (e) => {
				zoom += e.deltaY * -0.001;
				zoom = Math.max(0.1, zoom);
				zoom = Math.min(10, zoom);

				const svg = document.querySelector('svg');
				if (!svg) return;

				svg.style.transform = `translate(${transX}px, ${transY}px) scale(${zoom})`;
			});

			// Create number key listener
			document.addEventListener('keydown', (e) => {
				if (e.key >= '0' && e.key <= '9') {
					let num = Number(e.key);
					if (num === 0) num = 10;
					if (num > 0 && num <= colors.length) {
						active = num - 1;
					}
				}
			});

			// Create touch event listener
			// document.addEventListener('touchstart', (e) => {
			// 	if (e.touches.length === 1) {
			// 		touches == 1;
			// 		let startX = e.touches[0].clientX;
			// 		let startY = e.touches[0].clientY;

			// 		function drag(e: TouchEvent) {
			// 			const dx = e.touches[0].clientX - startX;
			// 			const dy = e.touches[0].clientY - startY;
			// 			transX += dx;
			// 			transY += dy;
			// 			startX = e.touches[0].clientX;
			// 			startY = e.touches[0].clientY;

			// 			const svg = document.querySelector('svg');
			// 			if (!svg) return;

			// 			svg.style.transform = `translate(${transX}px, ${transY}px) scale(${zoom})`;
			// 		}

			// 		function drop() {
			// 			document.removeEventListener('touchmove', drag);
			// 			document.removeEventListener('touchend', drop);
			// 		}

			// 		document.addEventListener('touchmove', drag);
			// 		document.addEventListener('touchend', drop);
			// 	} else if (e.touches.length === 2) {
			// 	}
			// });
		});
	});

	function tooDark(color: string) {
		const hex = color.replace('#', '');
		const r = parseInt(hex.substring(0, 0 + 2), 16);
		const g = parseInt(hex.substring(2, 2 + 2), 16);
		const b = parseInt(hex.substring(4, 4 + 2), 16);
		const brightness = (r * 299 + g * 587 + b * 114) / 1000;
		return brightness <= 155;
	}

	function downloadToPng() {
		const svg = document.querySelector('svg');
		if (!svg) return;

		// Serialize the SVG to a string, removing the style tag
		const serializer = new XMLSerializer();
		const svgString = serializer.serializeToString(svg).replace(/style=".*"/, '');

		// Create a canvas element
		const canvas = document.createElement('canvas');
		const ctx = canvas.getContext('2d');

		// Draw the SVG to the canvas using an image
		const img = new Image();
		img.width = svg.clientWidth;
		img.height = svg.clientHeight;
		img.onload = () => {
			canvas.width = img.width;
			canvas.height = img.height;
			ctx?.drawImage(img, 0, 0, img.width, img.height);
			const png = canvas.toDataURL('image/png');
			const a = document.createElement('a');
			a.href = png;
			a.download = `${name}.png`;
			a.click();
		};
		img.src = 'data:image/svg+xml;base64,' + btoa(svgString);
	}
</script>

{#if !loading}
	<div class="absolute h-screen w-screen overflow-hidden">
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
		<div class="fixed top-4 left-4">
			<div class="flex w-full flex-col gap-2 rounded-lg bg-white p-2 drop-shadow-md">
				{#if menuOpen}
					<button
						onclick={() => {
							menuOpen = false;
						}}
					>
						<CloseIcon />
					</button>
					<Button
						text="Download"
						handleClick={downloadToPng}
						tooltip="Download the current painting as a PNG image"
					/>
					<Button text="Home" handleClick={() => goto('/')} tooltip="Go back to the home page" />
				{:else}
					<button
						onclick={() => {
							menuOpen = true;
						}}
					>
						<HamburgerIcon />
					</button>
				{/if}
			</div>
		</div>
		{#if info}
			<div class="fixed top-4 right-4">
				<div
					class={`flex w-full gap-2 rounded-lg bg-white p-2 drop-shadow-md ${tooDark(colors[info - 1]) ? 'text-white' : 'text-black'}`}
				>
					<div
						class="flex h-8 w-8 items-center justify-center rounded-md"
						style="background-color: {colors[info - 1]}"
					>
						{info}
					</div>
				</div>
			</div>
		{/if}
	</div>
{/if}

<Loading bind:loading />

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
