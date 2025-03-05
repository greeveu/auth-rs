<script lang="ts">
	import type AuthRsApi from "$lib/api";
	import type OAuthApplication from "$lib/models/OAuthApplication";
	import type UserMinimal from "$lib/models/User";
	import { Unlink } from "lucide-svelte";
	import { onMount } from "svelte";

    export let api: AuthRsApi;
    export let user: UserMinimal;
    export let connections: OAuthApplication[];

    onMount(() => {
        api.getConnections(user)
            .then(apps => connections = apps)
            .catch(e => console.error(e));
    });
</script>

{#if connections.length < 1}
    <div class="flex flex-col items-center justify-center gap-[25px] h-full w-full">
        <Unlink size="75" class="opacity-40" />
        <p class="text-[20px] opacity-50">You don't have any connected Apps.</p>
    </div>
{:else}
    <div class="flex flex-wrap w-full overflow-y-scroll gap-[25px]">
        {#each connections as connection}
            <div class="flex flex-col items-start justify start gap-[10px] w-[350px] h-[200px] bg-[#111] rounded-md" style="padding: 15px;">
                <p class="text-[20px] font-bold">{connection.name}</p>
                <p class="text-[12px] opacity-50">{@html connection.description.length > 1 ? connection.description.substring(0, 205) + (connection.description.length > 205 ? '...' : '') : '<i>This App does not have a description.</i>'}</p>
                <div class="flex flex-col items-end justify-end min-h-[30px] h-full w-full">
                    <button
                        class="flex items-center justify-center w-[100px] h-[30px] bg-red-800 cursor-pointer hover:bg-transparent hover:text-red-800 border-red-800 border-[2px] transition-all rounded-md"
                        style="padding: 5px;"
                        on:click={() => api.disconnectConnection(connection).then(() => connections = connections.filter(c => c._id != connection._id))}
                    >
                        <p class="text-[13px]">Disconnect</p>
                    </button>
                </div>
            </div>
        {/each}
    </div>
{/if}

<style>
    ::-webkit-scrollbar {
        width: 0px;
    }
</style>