<script lang="ts">
	import TotpInput from '../../lib/components/auth/TotpInput.svelte';
	import Popup from '../../lib/components/global/Popup.svelte';
	import { ClockFading, KeyRound, Pen, Search, ShieldCheck, ShieldX, Trash } from 'lucide-svelte';
	import type AuthRsApi from "$lib/api";
	import type User from "$lib/models/User";
	import { goto } from '$app/navigation';
	import TextInput from '$lib/components/global/TextInput.svelte';
	import Passkey from '$lib/models/Passkey';
	import { onMount } from 'svelte';
	import Tooltip from 'sv-tooltip';
	import DateUtils from '$lib/dateUtils';
	import PasskeyUpdates from '$lib/models/PasskeyUpdates';

    export let api: AuthRsApi;
    export let user: User;
    export let passkeys: Passkey[];

    let activeTabIndex = 0;
    let tabs = [
        { name: '2FA', icon: ClockFading },
        { name: 'Passkeys', icon: KeyRound }
    ];

    let startEnable2FAPopup = false;
    let completeEnable2FAPopup = false;
    let disable2FAPopup = false;
    
    let enablePassword = '';
    let showEnablePassword = false;
    
    let enableTotpQR: string | null = null;
    let enableTotp: (number | null)[] = [null, null, null, null, null, null];
    
    let disablePassword = '';
    let showDisablePassword = false;

    let editPasskeyPopup: boolean = false;
    let editPasskey: Passkey | null = null;
    let editPasskeyName: string = '';

    let deletePasskeyPopup: boolean = false;
    let deletePasskey: Passkey | null = null;

    async function showEnableMFAPopup() {
        enablePassword = '';
        showEnablePassword = false;
        startEnable2FAPopup = true;
    }

    async function enableMFA() {
        if (enableTotpQR) {
            const totp = enableTotp.map(n => n === null ? 0 : n).join('');
            api.mfa(totp).then((newUser: User) => {
                completeEnable2FAPopup = false;
                user = newUser;
                goto('/logout');
            });
        }
    }

    async function createPasskey() {
        const passkey = await api.registerPasskey(user._id);
        if (passkey) {
            console.log('Passkey created:', passkey);
            passkeys = [...passkeys, passkey];
            editPasskey = passkey;
            editPasskeyName = "";
            editPasskeyPopup = true;
        }
    }

    async function showDisableMFAPopup() {
        disablePassword = '';
        showDisablePassword = false;
        disable2FAPopup = true;
    }

    onMount(() => {
        if (user.passkeys || passkeys.length > 0) {
            api.getUserPasskeys(user._id).then((newPasskeys: Passkey[]) => {
                passkeys = newPasskeys;
            });
        }
    })
</script>

{#if editPasskeyPopup}
    <Popup title="Edit Passkey" onClose={() => editPasskeyPopup = false}>
        <div class="flex flex-col items-center justify-center min-w-[350px]">
            <TextInput label="Name" bind:value={editPasskeyName} autofocus />
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-green-600 rounded-md {editPasskeyName.length > 3 ? 'cursor-pointer' : 'cursor-default opacity-50'} text-[18px] green-button"
                style="margin-top: 20px; margin-bottom: 10px;"
                on:click={editPasskeyName.length > 3 ? () => {
                    editPasskeyPopup = false;
                    api.updatePasskey(user._id, editPasskey?.id!, new PasskeyUpdates({ name: editPasskeyName }))
                        .then(newPasskey => {
                            passkeys = passkeys.map(p => p.id === newPasskey.id ? newPasskey : p);
                        }).catch(e => console.error(e));
                } : null}
            >Save</p>
        </div>
    </Popup>
{/if}

{#if deletePasskeyPopup}
    <Popup title="Delete Passkey" onClose={() => deletePasskeyPopup = false}>
        <div class="flex flex-col items-center justify-center max-w-[350px]" style="margin-top: 20px; margin-bottom: 20px;">
            <p class="text-[14px] text-center opacity-50">Are you sure you want to delete the passkey "{deletePasskey?.name}"?</p>
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-red-600 cursor-pointer rounded-md text-[18px] red-button"
                style="margin-top: 25px;"
                on:click={() => {
                    deletePasskeyPopup = false;
                    api.deletePasskey(user._id, deletePasskey?.id!)
                        .then(() => {
                            passkeys = passkeys.filter(p => p.id !== deletePasskey?.id);
                        }).catch(e => console.error(e));
                }}
            >Confirm</p>
        </div>
    </Popup>
{/if}

<div class="flex flex-col items-center justify-start h-full w-full">
    <div class="flex flex-row gap-25px outline-[2px] outline-[#333] rounded-md" style="padding: 5px;">
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        {#each tabs as tab, index}
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
                class="flex flex-row items-center justify-center gap-[10px] rounded-md w-[140px]"
                class:bg-blue-500={activeTabIndex == index}
                class:cursor-default={activeTabIndex == index}
                class:cursor-pointer={activeTabIndex != index}
                on:click={() => activeTabIndex = index}
                style="padding: 10px;"
            >
                <svelte:component this={tab.icon} height="10px" width="10px" />
                <p class="text-[16px]">{tab.name}</p>
            </div>
        {/each}
    </div>
    {#if activeTabIndex == 0}
        <div class="flex flex-col items-center justify-center h-full">
            {#if user.mfa}
                <ShieldCheck size="120" class="text-green-600" />
            {:else}
                <ShieldX size="120" class="text-red-600" />
            {/if}
            <p class="text-[24px]" style="margin-top: 10px;">2FA is {user.mfa ? 'enabled' : 'disabled'}.</p>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
                class="flex flex-row items-center justify-center gap-[15px] w-[275px] border-[2px] border-[#222] rounded-md cursor-pointer transition-all {user.mfa ? 'hover:text-red-600 hover:border-red-600' : 'hover:text-green-600 hover:border-green-600'}"
                style="padding: 10px 15px; margin-top: 20%;"
                on:click={user.mfa ? showDisableMFAPopup : showEnableMFAPopup}
            >
                {#if user.mfa}
                    Disable MFA
                {:else}
                    Enable MFA
                {/if}
            </div>
        </div>
    {:else if activeTabIndex == 1}
        <div class="flex flex-col h-full w-full" style="margin-top: 25px;">
            {#if passkeys.length < 1}
                <div class="flex flex-col items-center justify-center gap-[25px] h-full w-full">
                    <Search size="75" class="opacity-40" />
                        <p class="text-[20px] text-center opacity-50">You dont have any passkeys registered.</p>
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                        <p
                            class="border-blue-500 text-blue-500 hover:bg-blue-500 hover:text-white transition-all border-[1.5px] cursor-pointer rounded-md"
                            style="padding: 10px; margin-top: 25px;"
                            on:click={createPasskey}
                        >Create Passkey</p>
                </div>
            {:else}
                <div class="absolute flex flex-col min-h-[70px] items-center justify-center self-end" style="margin-right: 50px;">
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                    <p
                        class="border-blue-500 text-blue-500 hover:bg-blue-500 hover:text-white transition-all border-[1.5px] cursor-pointer rounded-md"
                        style="padding: 10px;"
                        on:click={createPasskey}
                    >Create Passkey</p>
                </div>
                <div class="flex flex-wrap overflow-y-scroll gap-[25px]">
                    {#each passkeys as passkey}
                        <div class="flex flex-col items-start justify start gap-[25px] min-w-[250px] max-w-[200px] min-h-[100px] border-[2px] border-[#333] rounded-md" style="padding: 15px;">
                            <div class="flex flex-row justify-between w-full">
                                <p class="text-[20px] font-bold h-[20px]">{passkey.name}</p>
                                <div class="flex flex-row">
                                    <Tooltip tip="Edit Passkey" bottom>
                                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                                        <div class="flex self-end" style="margin-right: 12.5px;" on:click={() => {
                                            editPasskey = passkey;
                                            editPasskeyName = passkey.name;
                                            editPasskeyPopup = true;
                                        }}>
                                            <Pen
                                                class="cursor-pointer hover:text-blue-500 transition-all"
                                                size=20
                                            />
                                        </div>
                                    </Tooltip>
                                    <Tooltip tip="Delete Passkey" bottom color="var(--color-red-600)">
                                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                                        <div class="flex self-end" on:click={() => {
                                            deletePasskey = passkey;
                                            deletePasskeyPopup = true;
                                        }}>
                                            <Trash
                                                class="cursor-pointer hover:text-red-600 transition-all"
                                                size=20
                                            />
                                        </div>
                                    </Tooltip>
                                </div>
                            </div>
                            <p class="text-[14px] opacity-35 h-[20px]">Created at {DateUtils.getDateString(Passkey.getCreatedAt(passkey))}</p>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}
</div>

{#if startEnable2FAPopup}
    <Popup title="Enable MFA" onClose={() => startEnable2FAPopup = false}>
        <div class="flex flex-col items-center justify-center w-full" style="margin-top: 10px; margin-bottom: 10px;">
            <TextInput type="password" label="" placeholder="Confirm Password" bind:value={enablePassword} autocomplete="current-password" autofocus />
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-green-600 rounded-md text-[18px] button green-button"
                style="margin-top: 25px;"
                class:enabled={enablePassword.length > 0}
                on:click={enablePassword.length > 0 ? () => {
                    startEnable2FAPopup = false;
                    enableTotp = [null, null, null, null, null, null];
                    completeEnable2FAPopup = true;
                    api.enableMfa(user, enablePassword)
                        .then((enableData: { token: string; }) => {
                            startEnable2FAPopup = false;
                            enableTotpQR = enableData.token;
                        })
                } : null}
            >Confirm</p>
        </div>
    </Popup>
{/if}

{#if completeEnable2FAPopup}
    <Popup title="Complete MFA activation" onClose={() => completeEnable2FAPopup = false}>
        <div class="flex flex-col items-center justify-center max-w-[500px]" style="margin-top: 20px; margin-bottom: 20px;">
            <img src="data:image/png;base64,{enableTotpQR}" alt="MFA QR Code" class="w-[200px] h-[200px] rounded-md" />
            <p class="text-[14px] opacity-50 text-center" style="margin-top: 15px; margin-bottom: 15px;">Scan the QR code with your authenticator app and enter the 6 digit code below to complete the activation.</p>
            <TotpInput bind:totp={enableTotp} completeTotp={enableMFA} disabled={false} />
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-green-600 rounded-md text-[18px] button green-button"
                style="margin-top: 25px;"
                class:enabled={enableTotp.filter(c => c != null).length === 6}
                on:click={enableMFA}
            >Confirm</p>
        </div>
    </Popup>
{/if}

{#if disable2FAPopup}
    <Popup title="Disable MFA" onClose={() => disable2FAPopup = false}>
        <div class="flex flex-col items-center justify-center w-full" style="margin-top: 10px; margin-bottom: 10px;">
            <TextInput type="password" label="" placeholder="Confirm Password" bind:value={disablePassword} autocomplete="current-password" autofocus />
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-red-600 rounded-md text-[18px] button red-button"
                style="margin-top: 25px;"
                class:enabled={disablePassword.length > 0}
                on:click={disablePassword.length > 0 ? () => api.disableMfa(user, null, disablePassword).then(newUser => {disable2FAPopup = false; user = newUser; goto('/logout')}) : null}
            >Confirm</p>
        </div>
    </Popup>
{/if}

<style>
    .button {
        transition-duration: .2s;
        opacity: 0.5;
        cursor: default;
    }

    .button.enabled {
        opacity: 1;
        cursor: pointer;
    }

    .button.red-button.enabled:hover {
        background-color: transparent;
        color: var(--color-red-900);
    }

    .button.green-button.enabled:hover {
        background-color: transparent;
        color: var(--color-green-900);
    }
</style>