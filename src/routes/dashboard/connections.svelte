<script lang="ts">
	import ScopeList from '../../lib/components/global/ScopeList.svelte';
	import type AuthRsApi from "$lib/api";
	import type OAuthConnection from "$lib/models/OAuthConnection";
	import type UserMinimal from "$lib/models/User";
	import { Unlink } from "lucide-svelte";
	import { onMount } from "svelte";

    export let api: AuthRsApi;
    export let user: UserMinimal;
    export let connections: OAuthConnection[];

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
            <div class="flex flex-col items-start justify start gap-[10px] w-[350px] min-h-[200px] bg-[#111] rounded-md" style="padding: 15px;">
                <p class="text-[20px] font-bold h-[20px]">{connection.application.name}</p>
                <p class="text-[12px] opacity-35 h-[20px]">Authorized at {connection.createdAt.split(' ')[0].replaceAll('-', ' ').split(' ').reverse().join('.')}</p>
                <p class="text-[12px] opacity-50">{@html (connection.application.description?.length ?? 0) > 1 ? connection.application.description?.substring(0, 200) + ((connection.application.description?.length ?? 0) > 200 ? '...' : '') : '<i>This app does not have a description.</i>'}</p>
                <p class="text-[12px] h-[10px]">Permissions:</p>
                <div class="flex flex-col w-full">
                    <ScopeList scopes={connection.scope} iconSize={17.5} textSize="12px" gap="10px" />
                </div>
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