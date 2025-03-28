<script lang="ts">
    export let label: string;
    export let value: boolean;
    export let onToggle = (value: boolean) => {};
    export let disabled = false;
</script>

<div class="flex flex-col">
    <p class="text-[14px]" style="padding: 2px 6px;">{label}</p>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="flex items-center justify-center w-[200px] h-[75px] rounded-md {disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'} {value ? 'bg-green-500' : 'bg-red-500'}" on:click={disabled ? () => {} : () => { value = !value; onToggle(value); }}>
        {value ? 'ON' : 'OFF'}
    </div>
</div>