<script lang="ts">
	import { KeyRound } from 'lucide-svelte';
    import { Circle } from 'svelte-loading-spinners';
	import TotpInput from './../../lib/components/auth/TotpInput.svelte';
    import AuthRsApi from '$lib/api';
	import AuthStateManager from '$lib/auth';
	import { onMount, tick } from "svelte";
	import type Settings from '$lib/models/Settings';
	import TextInput from '$lib/components/global/TextInput.svelte';

    const authStateManager = new AuthStateManager();
    const api = new AuthRsApi();
    let settings: Settings | null = null;
    let step = 0;
    let supportsPasskeys = false;

    let email = '';
    let password = '';
    let isUsingPasskey = false;
    /**
	 * @type {number[] | null[]}
	 */
    let totp: (number | null)[] = [null, null, null, null, null, null];
    let isLoading = false;

    let redirect: string | null = null;

    async function login() {
        if (email == '' || password == '') {
            console.error('Email and password are required');
            return;
        }

        step = 1;
        isLoading = true;

        api.login(email, password)
            .then(async (data) => {
                if (data.mfaRequired) {
                    isLoading = false;
                    step = 2;
                    await tick();
                    document.getElementById('totp-0')?.focus();
                    return;
                }
                step = 4;
                window.location.href = redirect ?? '/';
            })
            .catch((error) => {
                step = 0;
                isLoading = false;
                password = '';
                console.error(error);
            });
    }

    function usePasskey() {
        isUsingPasskey = true;
        api.startPasskeyAuth().then(() => {
            window.location.href = redirect ?? '/';
        }).catch(error => {
            isUsingPasskey = false;
            console.error(error);
        });
    }

    async function completeTotp() {
        const code = totp.map((value) => `${value}`).join('');
        if (code.length != 6) {
            console.error('Invalid TOTP code');
            return;
        }
        step = 3;
        isLoading = true;

        api.mfa(code)
            .then(() => {
                window.location.href = redirect ?? '/';
            })
            .catch(async (error) => {
                isLoading = false;
                step = 2;
                totp = [null, null, null, null, null, null];
                await tick();
                document.getElementById('totp-0')?.focus();
                console.error(error);
            });
    }

    onMount(async () => {
        redirect = new URL(window.location.href).searchParams.get('redirect_uri');
        supportsPasskeys = window.PublicKeyCredential != null;

        const token = authStateManager.getToken();
        if (token) {
            api.setToken(token);
            api.getCurrentUser()
                .then(() => {
                    window.location.href = redirect ?? '/';
                    return;
                })
                .catch(() => {
                    api.setToken(null);
                    authStateManager.clearToken();
                });
        }
        settings = await api.getSettings();
        
        document.getElementById('form')?.addEventListener('submit', e => {
            e.preventDefault();
        });
    });
</script>

<div class="flex flex-col items-center justify-center h-screen">
    <h1
        class="text-4xl font-bold"
        style="margin-bottom: 35px;"
    >{step < 2 ? 'Login' : 'Verify Login'}</h1>
    <form id="form" class="flex flex-col items-center justify-center mt-4">
        {#if step < 2}
            <TextInput type="email" label="Email" bind:value={email} autocomplete={"email webauthn"} autofocus />
            <TextInput type="password" label="Password" bind:value={password} autocomplete={"current-password"} />
        {:else if step == 2}
            <TotpInput bind:totp disabled={step != 2} completeTotp={completeTotp} />
        {/if}
        <button
            type="submit"
            class="border-[1.5px] border-blue-500 bg-blue-500 text-white rounded-md login-button"
            class:bg-transparent={isLoading}
            class:text-blue-500={isLoading}
            style="padding: 7.5px; width: {step < 2 ? 300 : 250}px; margin-top: {step < 2 ? 5 : 20}px;"
            class:enabled={step == 0 ? email != '' && password != '' : totp.map(c => c?.toString()).join('').length >= 6 && (step == 0 || step == 2)}
            on:click={step == 0 ? login : completeTotp}
        >
            <div class="flex flex-row items-center justify-center gap-[10px]">
                {#if step < 2 && !isLoading}
                    <p class="text-[17px]">Login</p>
                {:else if step < 2 && isLoading}
                    <Circle color="var(--color-blue-500)" size=15 />
                    <p class="text-[17px]">Logging in</p>
                    {:else if step >= 2 && !isLoading}
                    <p class="text-[17px]">Verify</p>
                {:else if step >= 2 && isLoading}
                    <Circle color="var(--color-blue-500)" size=15 />
                    <p class="text-[17px]">Verifying</p>
                {/if}
            </div>
        </button>
    </form>
    {#if step < 2 && supportsPasskeys}
        <hr class="w-[200px] text-[#333] border-[1.5px] rounded-md" style="margin-top: 15px;">
        <button
            type="submit"
            class="border-[1.5px] border-blue-500 text-blue-500 hover:text-white hover:bg-blue-500 rounded-md text-[15px] transition-all"
            class:text-white={isUsingPasskey}
            class:bg-blue-500={isUsingPasskey}
            class:cursor-pointer={!isUsingPasskey}
            style="padding: 5px; width: 300px; margin-top: 15px;"
            class:enabled={true}
            on:click={usePasskey}
        >
            <div class="flex flex-row items-center justify-center gap-[10px]">
                {#if isUsingPasskey}
                    <Circle color="white" size=15 />
                    <p class="text-[14px]">Waiting for passkey</p>
                {:else}
                    <KeyRound size=17.5 />
                    <p class="text-[14px]">Use a passkey</p>
                {/if}
            </div>
        </button>
    {/if}
    {#if settings?.openRegistration && step < 2}
        <p class="text-[14px]" style="margin-top: 15px;">or</p>
        <a href="/register" class="text-[13px]" style="margin-top: 10px;">Don't have an account? <i>Register here!</i></a>
    {/if}
</div>

<style>
    .login-button {
        transition-duration: .2s;
        opacity: 0.5;
        cursor: default;
    }

    .login-button.enabled {
        opacity: 1;
        cursor: pointer;
    }

    .login-button.enabled:hover {
        background-color: transparent;
        color: var(--color-blue-500);
    }
</style>