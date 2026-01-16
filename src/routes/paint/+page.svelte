<script lang="ts">
	import init, { flat_to_svg } from 'pbn';
	import { onMount, untrack } from 'svelte';
	import { page } from '$app/state';
	import Loading from '../../components/Loading.svelte';
	import { goto } from '$app/navigation';
	import HamburgerIcon from '../../components/HamburgerIcon.svelte';
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
	let textColor = $state<string>('#c27aff');
	let fontSize = $state<number>(10);
	let touches = 0;
	let transX = 0;
	let transY = 0;
	let zoom = 1;
	let centerX = 0;
	let centerY = 0;
	const EXTRA_KEYMAP = { q: 11, w: 12, e: 13, r: 14, t: 15, y: 16, u: 17, i: 18, o: 19, p: 20 };

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
			centerX = width / 2;
			centerY = height / 2;
			if (Math.abs(height) < 3000 && Math.abs(width) < 3000) {
				transX = window.innerWidth / 2 - width / 2;
				transY = window.innerHeight / 2 - height / 2;
				setPos();
			}

			// Create drag event listener
			document.addEventListener('mousedown', (e) => {
				let startX = e.x;
				let startY = e.y;

				function drag(e: MouseEvent) {
					const dx = e.x - startX;
					const dy = e.y - startY;
					transX += dx;
					transY += dy;
					startX = e.x;
					startY = e.y;

					setPos();
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
				const prevZoom = zoom;
				zoom += e.deltaY * -0.001;
				zoom = Math.max(0.1, zoom);
				zoom = Math.min(10, zoom);

				// Scale to keep cursor in same place
				const svgX = (e.x - transX - centerX) / prevZoom + centerX;
				const svgY = (e.y - transY - centerY) / prevZoom + centerY;
				transX = e.x - (svgX - centerX) * zoom - centerX;
				transY = e.y - (svgY - centerY) * zoom - centerY;

				setPos();
			});

			// Create number key listener
			document.addEventListener('keydown', (e) => {
				let num;
				if (e.key >= '0' && e.key <= '9') {
					num = Number(e.key);
					if (num === 0) num = 10;
				}
				if (e.key in EXTRA_KEYMAP) {
					num = (EXTRA_KEYMAP as any)[e.key]; // We know e.key is a key... (stupid typescript)
				}

				if (num && num > 0 && num <= colors.length) {
					active = num - 1;
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

			// Click listener to close menu
			document.addEventListener('click', () => {
				if (menuOpen) menuOpen = false;
			});
		});
	});

	function setPos() {
		const svg = document.querySelector('svg');
		if (!svg) return;
		svg.style.transform = `translate(${transX}px, ${transY}px) scale(${zoom})`;
	}

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

	async function downloadFull() {
		const dir = await navigator.storage.getDirectory();
		const fileHandler = await dir.getFileHandle(name!, { create: false });
		const file = await fileHandler.getFile();

		const a = document.createElement('a');
		a.href = URL.createObjectURL(file);
		a.download = name + '.png' || 'download.png';
		a.click();
	}

	function checkCompletion() {
		const total = document.querySelectorAll('path').length;
		const numIncomplete = document.querySelectorAll('path.unfilled').length;
		if (numIncomplete === total) {
			alert('You finished the painting!');
		} else {
			alert(`There are ${numIncomplete}/${total} unfilled areas!`);
		}
	}
</script>

{#if !loading}
	<div class="absolute h-screen w-screen overflow-hidden" style="--text-color: {textColor}; --font-size: {fontSize}px">
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
			{#if menuOpen}
				<div class="flex w-full flex-col gap-2 rounded-lg bg-white p-2 drop-shadow-md">
					<Button text="Home" handleClick={() => goto('/')} tooltip="Go back to the home page" />
					<Button
						text="Download Current"
						handleClick={downloadToPng}
						tooltip="Download the current painting as a PNG image"
					/>
					<Button
						text="Download Finished"
						handleClick={downloadFull}
						tooltip="Download the full painting as a PNG image"
					/>
					<Button
						text="Check Completion"
						handleClick={checkCompletion}
						tooltip="Check the completion of the puzzle"
					/>
					<div class="flex flex-col w-full">
						<p>Set Text Color</p>
						<input
							type="color"
							bind:value={textColor}
							class="w-full cursor-pointer rounded-md border-2 border-white duration-200 hover:border-purple-500"
							onclick={(e) => e.stopPropagation()}
						/>
						<p class="mt-1">Set Text Size</p>
						<input
							type="number"
							bind:value={fontSize}
							class="w-full cursor-pointer rounded-md border-2 border-white duration-200 hover:border-purple-500"
							onclick={(e) => e.stopPropagation()}
						/>
					</div>
				</div>
			{:else}
				<button
					class="flex w-full cursor-pointer flex-col gap-2 rounded-lg bg-white p-2 drop-shadow-md"
					onclick={(e) => {
						e.stopPropagation();
						menuOpen = true;
					}}
				>
					<HamburgerIcon />
				</button>
			{/if}
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
		fill: var(--text-color);
		font-weight: bold;
		font-family: 'Inter', sans-serif;
		font-size: var(--font-size);
		cursor: pointer;

		user-select: none;
		-moz-user-select: -moz-none;
		-khtml-user-select: none;
		-webkit-user-select: none;
	}
</style>
