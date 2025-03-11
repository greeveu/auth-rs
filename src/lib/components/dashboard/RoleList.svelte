<script lang="ts">
	import type Role from "$lib/models/Role";
	import UserMinimal from "$lib/models/User";
	import { PlusCircle, X } from "lucide-svelte";

    export let label: string;
    export let roles: Role[];
    export let readOnly: boolean = true;
    export let disableOutline: boolean = false;
    export let isSystemAdmin: boolean = false;
    export let onAdd: () => void;
    export let onRemove: (role: Role) => void;
</script>

<div class="flex flex-col">
    <div class="flex flex-row items-center">
        <p class="text-[14px]" style="padding: 2px 6px;">{label}</p>
        {#if !readOnly}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div on:click={onAdd}>
                <PlusCircle size="15" class="hover:text-green-500 cursor-pointer transition-all" />
            </div>
        {/if}
    </div>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="name-input flex items-center flex-wrap outline-none gap-[10px] rounded-md w-[275px] {disableOutline ? '' : 'border-[2px] border-[#333]'}" style={disableOutline ? "" : "padding: 10px;"}>
        {#each roles as role}
            <div class="flex flex-row items-center justify-between gap-[10px] text-[13px] h-[40px] bg-[#111] rounded-md" style="padding: 10px; {disableOutline ? 'margin: 0 2.5px;' : ''}">
                <p style="color: white !important;">{role.name}</p>
                {#if role._id != UserMinimal.DEFAULT_ROLE_ID && !(role._id == UserMinimal.ADMIN_ROLE_ID && !isSystemAdmin) && !readOnly}
                    <div on:click={() => onRemove(role)}>
                        <X size="15" class="hover:text-red-500 cursor-pointer transition-all" />
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>