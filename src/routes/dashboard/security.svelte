<script lang="ts">
	import TotpInput from './../../lib/components/auth/TotpInput.svelte';
	import Popup from './../../lib/components/global/Popup.svelte';
	import { ShieldCheck, ShieldX } from 'lucide-svelte';
	import type AuthRsApi from "$lib/api";
	import type UserMinimal from "$lib/models/User";

    export let api: AuthRsApi;
    export let user: UserMinimal;

    let enable2FAPopup = false;
    let disable2FAPopup = false;

    let enableTotp: (number | null)[] = [null, null, null, null, null, null];
    let disableTotp: (number | null)[] = [null, null, null, null, null, null];
    let disablePassword = '';

    async function enableMFA() {
        enable2FAPopup = true;
    }
    async function disableMFA() {
        disable2FAPopup = true;
    }
</script>

<div class="flex flex-col items-center justify-start h-full" style="padding-top: 10%;">
    {#if user.mfa}
    <ShieldCheck size="120" class="text-green-600" />
    {:else}
    <ShieldX size="120" class="text-red-600" />
    {/if}
    <p class="text-[24px]" style="margin-top: 10px;">MFA is {user.mfa ? 'enabled' : 'disabled'}.</p>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="flex flex-row items-center justify-center gap-[15px] w-[275px] border-[2px] border-[#222] rounded-md cursor-pointer transition-all {user.mfa ? 'hover:text-red-600 hover:border-red-600' : 'hover:text-green-600 hover:border-green-600'}"
        style="padding: 10px 15px; margin-top: 20%;"
        on:click={user.mfa ? disableMFA : enableMFA}
    >
        {#if user.mfa}
            Disable MFA
        {:else}
            Enable MFA
        {/if}
    </div>
</div>

{#if enable2FAPopup}
    <Popup title="Enable MFA" onClose={() => enable2FAPopup = false}>
        <div class="flex items-center justify-center w-full" style="margin-top: 20px; margin-bottom: 20px;">
            <TotpInput bind:totp={enableTotp} completeTotp={enableMFA} disabled={false} />
        </div>
        <button>Submit</button>
    </Popup>
{/if}

{#if disable2FAPopup}
    <Popup title="Disable MFA" onClose={() => disable2FAPopup = false}>
        <div class="flex items-center justify-center w-full" style="margin-top: 20px; margin-bottom: 20px;">
            <TotpInput bind:totp={disableTotp} completeTotp={disableMFA} disabled={false} />
        </div>
        <button>Submit</button>
    </Popup>
{/if}