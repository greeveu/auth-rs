<script lang="ts">
	import { X } from "lucide-svelte";

    export let title;
    export let onClose: () => void;
</script>
<!-- svelte-ignore element_invalid_self_closing_tag -->
<div class="absolute flex items-center justify-center top-0 left-0 w-full h-full bg-black opacity-[85%] z-[999]" />
<div class="absolute flex items-center justify-center top-0 left-0 w-full h-full z-[1000]">
    <div class="flex flex-col gap-[7.5px] bg-black border-[2px] border-[#222] rounded-md" style="padding: 10px;">
        <div class="flex flex-row min-w-[300px] items-center justify-between">
            <p>{title}</p>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div class="flex items-center justify-center" on:click={() => onClose()}>
                <X size="24" class="cursor-pointer hover:text-red-600 transition-all" />
            </div>
        </div>
        <!-- svelte-ignore element_invalid_self_closing_tag -->
        <div class="w-full h-[2px] rounded-md bg-[#111]" />
        <slot />
    </div>
</div>