<script lang="ts">
	import { PlusCircle, X } from "lucide-svelte";

    export let redirectUris: string[];
    export let onAdd: () => void;
    export let onRemove: (role: string) => void;
</script>

<div class="flex flex-col">
    <div class="flex flex-row items-center gap-[5px]">
        <p class="text-[14px]" style="padding: 2px;">Redirect URI's</p>
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div on:click={onAdd}>
            <PlusCircle size="15" class="hover:text-green-500 cursor-pointer transition-all" />
        </div>
    </div>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="name-input flex flex-col items-start gap-[10px]">
        {#each redirectUris as redirectUri}
            <div class="flex flex-row items-center justify-between gap-[10px] text-[13px] bg-[#111] rounded-md" style="padding: 7.5px;">
                <p style="color: white !important;">{redirectUri.split('?')[0]}</p>
                {#if redirectUris.length > 1}
                    <div on:click={() => onRemove(redirectUri)}>
                        <X size="15" class="hover:text-red-500 cursor-pointer transition-all" />
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>