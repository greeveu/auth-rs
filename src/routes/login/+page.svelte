<script lang="ts">
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

    let email = '';
    let password = '';
    /**
	 * @type {number[] | null[]}
	 */
    let totp: (number | null)[] = [null, null, null, null, null, null];
    let loginText = 'Login';
    let verifyText = 'Verify';

    let redirect: string | null = null;

    async function login() {
        if (email == '' || password == '') {
            console.error('Email and password are required');
            return;
        }

        step = 1;
        loginText = 'Logging in...';

        api.login(email, password)
            .then(async (data) => {
                if (data.mfaRequired) {
                    loginText = 'Login';
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
                loginText = 'Login';
                password = '';
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
        verifyText = 'Verifying...';

        api.mfa(code)
            .then(() => {
                window.location.href = redirect ?? '/';
            })
            .catch(async (error) => {
                verifyText = 'Verify';
                step = 2;
                totp = [null, null, null, null, null, null];
                await tick();
                document.getElementById('totp-0')?.focus();
                console.error(error);
            });
    }

    onMount(async () => {
        redirect = new URL(window.location.href).searchParams.get('redirect_uri');

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
            <TextInput type="email" label="Email" bind:value={email} autofocus />
            <TextInput type="password" label="Password" bind:value={password} />
        {:else if step == 2}
            <TotpInput bind:totp disabled={step != 2} completeTotp={completeTotp} />
        {/if}
        <button
            type="submit"
            class="border-[1.5px] border-blue-500 bg-blue-500 text-white rounded-md text-[17px] button"
            style="padding: 7.5px; width: {step < 2 ? 300 : 250}px; margin-top: {step < 2 ? 5 : 20}px;"
            class:enabled={step == 0 ? email != '' && password != '' : totp.map(c => c?.toString()).join('').length >= 6 && (step == 0 || step == 2)}
            on:click={step == 0 ? login : completeTotp}
        >{step < 2 || step >= 4 ? loginText : verifyText}</button>
    </form>
    {#if settings?.openRegistration && step < 2}
        <p class="text-[14px]" style="margin-top: 15px;">or</p>
        <a href="/register" class="text-[13px]" style="margin-top: 10px;">Don't have an account? <i>Register here!</i></a>
    {/if}
</div>

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

    .button.enabled:hover {
        background-color: transparent;
        color: var(--color-blue-500);
    }
</style>