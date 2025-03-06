<script lang="ts">
	import Popup from './../../lib/components/global/Popup.svelte';
	import RedirectUriList from './../../lib/components/dashboard/RedirectUriList.svelte';
	import type AuthRsApi from "$lib/api";
	import type OAuthApplication from "$lib/models/OAuthApplication";
	import type UserMinimal from "$lib/models/User";
	import { BotOff, Pen, Trash } from "lucide-svelte";
	import { onMount } from "svelte";
	import OAuthApplicationUpdates from '$lib/models/OAuthApplicationUpdates';

    export let api: AuthRsApi;
    export let user: UserMinimal;
    export let applications: OAuthApplication[];

    let addRedirectUriPopup: boolean = false;
    let newRedirectUri: string = '';
    let newRedirectUriApplication: OAuthApplication | null = null;

    function openAddRedirectUriPopup(application: OAuthApplication) {
        newRedirectUri = '';
        newRedirectUriApplication = application;
    }

    function addRedirectUri(application: OAuthApplication) {
        addRedirectUriPopup = false;
        application?.redirectUris.push(newRedirectUri);
        api.updateOAuthApplication(application, new OAuthApplicationUpdates({ name: null, description: null, redirectUris: application.redirectUris }))
            .then(newApplication => {
                applications[applications.map(app => app._id).indexOf(application._id)] = newApplication;
            })
    }

    function removeRedirectUri(application: OAuthApplication, redirectUri: string) {
        application.redirectUris = application.redirectUris.filter(uri => uri != redirectUri);
    }

    onMount(() => {
        api.getOAuthApplications()
            .then(apps => applications = apps)
            .catch(e => console.error(e));
    });
</script>

{#if addRedirectUriPopup}
    <Popup title="Add Redirect URI" onClose={() => addRedirectUriPopup = false}>
        <div class="flex flex-col items-center justify-center w-full" style="margin-top: 20px; margin-bottom: 20px;">
            <input
                type="text"
                placeholder="https://example.com/callback"
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px;"
                bind:value={newRedirectUri}
            >
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-green-600 rounded-md text-[18px] button green-button"
                style="margin-top: 25px;"
                class:enabled={newRedirectUri.length > 0 && newRedirectUriApplication != null && newRedirectUri.includes('http') && newRedirectUri.includes('://') && newRedirectUri.includes('.')}
                on:click={newRedirectUri.length > 0 ? () => addRedirectUri(newRedirectUriApplication!) : null}
            >Confirm</p>
        </div>
    </Popup>
{/if}

{#if applications.length < 1}
    <div class="flex flex-col items-center justify-center gap-[25px] h-full w-full">
        <BotOff size="75" class="opacity-40" />
        <p class="text-[20px] opacity-50">You don't have any OAuth apps.</p>
    </div>
{:else}
    <div class="flex flex-wrap w-full overflow-y-scroll gap-[25px]">
        {#each applications as application}
            <div class="flex flex-col items-start justify start gap-[10px] min-w-[350px] min-h-[200px] border-[2px] border-[#333] rounded-md" style="padding: 15px;">
                <p class="text-[20px] font-bold h-[20px]">{application.name}</p>
                <p class="text-[12px] opacity-35 h-[20px]">Created at {application.createdAt.split(' ')[0].replaceAll('-', ' ').split(' ').reverse().join('.')}</p>
                <p class="text-[12px] opacity-50">{@html application.description?.length > 1 ? application.description?.substring(0, 200) + (application.description?.length > 200 ? '...' : '') : '<i>This application does not have a description.</i>'}</p>
                <RedirectUriList bind:redirectUris={application.redirectUris} onAdd={() => openAddRedirectUriPopup} onRemove={(redirectUri) => removeRedirectUri(application, redirectUri)} />
                <Trash
                    class="absolute self-end cursor-pointer hover:text-red-600 transition-all"
                    size=20
                    on:click={() => api.deleteOAuthApplication(application).then(() => applications = applications.filter(a => a._id != application._id))}
                />
                <Pen
                    class="absolute self-end cursor-pointer hover:text-blue-500 transition-all"
                    style="margin-right: 35px;"
                    size=20
                    on:click={() => null}
                />
            </div>
        {/each}
    </div>
{/if}

<style>
    ::-webkit-scrollbar {
        width: 0px;
    }
</style>