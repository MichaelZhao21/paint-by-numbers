<script lang="ts">
	interface TextFieldProps {
		label: string;
		value: string;
		placeholder?: string;
		className?: string;
	}
	let { label, value = $bindable(), placeholder, className }: TextFieldProps = $props();
</script>

<div class="flex">
	<label for="colors" class="pr-1 font-bold">{label}:</label>
	<input
		id="colors"
		type="text"
		bind:value
        placeholder="{placeholder}"
		class={`${className} rounded-sm border-2 px-1 duration-150 outline-none hover:border-purple-300 focus:border-purple-300`}
	/>
</div>
