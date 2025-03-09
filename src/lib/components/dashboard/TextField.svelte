<script lang="ts">
	import { Pencil } from "lucide-svelte";

    export let label: string;
    export let value: string;
    export let readonly: boolean = false;
    export let fullWidth: boolean = false;
    export let onClick: () => void = () => {};
</script>

<div class="flex flex-col">
    <p class="text-[14px]" style="padding: 2px 6px;">{label}</p>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="name-input flex justify-between items-center outline-none border-[2px] border-[#222] hover:text-blue-500 rounded-md {fullWidth ? 'w-full' : 'min-w-[275px]'} transition-all {readonly ? 'cursor-default' : 'cursor-pointer'}"
        style="padding: 3.5px 10px;"
        on:click={onClick}
    >
        <p style="color: white !important;">{value}</p>
        {#if !readonly}
            <Pencil size="17.5" on:click={onClick} />
        {/if}
    </div>
</div>