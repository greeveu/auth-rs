<script lang="ts">
	import TotpInput from './../../lib/components/auth/TotpInput.svelte';
    import AuthRsApi from '$lib/api';
	import AuthStateManager from '$lib/auth';
	import { onMount, tick } from "svelte";
    import { Eye, EyeOff } from 'lucide-svelte';
	import type Settings from '$lib/models/Settings';

    const authStateManager = new AuthStateManager();
    const api = new AuthRsApi();
    let settings: Settings | null = null;
    let step = 0;

    let email = '';
    let password = '';
    let showPassword = false;
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
                loginText = 'Login';
                if (data.mfaRequired) {
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
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="email"
                placeholder="Email"
                class="border-[1.5px] border-gray-300 rounded-md opacity-75"
                style="padding: 5px 10px; width: 300px; margin: 15px;"
                autofocus={true}
                bind:value={email}
            >
            <input
                type={showPassword ? "text" : "password"}
                placeholder="Password"
                class="border-[1.5px] border-gray-300 rounded-md opacity-75"
                style="padding: 5px 10px; width: 300px; margin-bottom: 15px;"
                bind:value={password}
            >
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
                class="absolute cursor-pointer"
                style="margin-left: 260px; margin-top: 5px;"
                on:click={() => showPassword = !showPassword}
            >
                {#if showPassword}
                    <Eye class="size-[18px]" />
                {:else}
                    <EyeOff class="size-[18px]" />
                {/if}
            </span>
        {:else if step == 2}
            <TotpInput bind:totp disabled={step != 2} completeTotp={completeTotp} />
        {/if}
        <button
            type="submit"
            class="border-[1.5px] border-blue-500 bg-blue-500 text-white rounded-md text-[17px] button"
            style="padding: 7.5px; width: {step < 2 ? 300 : 250}px; margin-top: {step < 2 ? 5 : 20}px;"
            class:enabled={step == 0 ? email != '' && password != '' : totp.map(c => c?.toString()).join('').length >= 6 && (step == 0 || step == 2)}
            on:click={step == 0 ? login : completeTotp}
        >{step < 2 ? loginText : verifyText}</button>
    </form>
    {#if settings?.allowRegistration && step < 2}
        <p class="text-[14px]" style="margin-top: 15px;">or</p>
        <a href="/register" class="text-[13px]" style="margin-top: 10px;">Don't have an account? <i>Register here!</i></a>
    {/if}
</div>

<style>
    input:focus {
        outline: none;
        border: solid 1.5px var(--color-blue-500);
    }

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