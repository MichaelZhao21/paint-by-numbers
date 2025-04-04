<script lang="ts">
    export let text;
    export let handleClick;
    export let className = '';
    export let disabled = false;
</script>

<button
    onclick={handleClick}
    class={`${className} cursor-pointer rounded-md bg-purple-300 px-6 py-1 font-bold text-black drop-shadow-md duration-200 outline-none hover:bg-purple-400 focus:bg-purple-400 disabled:bg-gray-200 disabled:text-slate-400 disabled:cursor-not-allowed`}
    disabled={disabled}
>
    {text}
</button>