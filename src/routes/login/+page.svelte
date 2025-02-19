<script lang="ts">
    import AuthRsApi from '$lib/api';
	import AuthStateManager from '$lib/auth';
	import { onMount, tick } from "svelte";

    const authStateManager = new AuthStateManager();
    const api = new AuthRsApi();
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
            .then((data) => {
                window.location.href = redirect ?? '/';
            })
            .catch((error) => {
                verifyText = 'Verify';
                step = 2;
                totp = [null, null, null, null, null, null];
                console.error(error);
            });
    }

    onMount(() => {
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
                class="border border-gray-300 rounded-md opacity-75"
                style="padding: 5px 10px; width: 300px; margin: 15px;"
                autofocus={true}
                bind:value={email}
            >
            <input
                type="password"
                placeholder="Password"
                class="border border-gray-300 rounded-md opacity-75"
                style="padding: 5px 10px; width: 300px; margin-bottom: 15px;"
                bind:value={password}
            >
        {:else if step == 2}
            <div class="flex flex-row w-[250px] justify-between">
                {#each [0, 1, 2, 3, 4, 5] as index}
                    <!-- svelte-ignore a11y_autofocus -->
                    <input
                        type="number"
                        class="border border-gray-300 rounded-md opacity-75"
                        style="padding: 5px; width: 30px; height: 40px;"
                        maxlength="1"
                        max="9"
                        id="totp-{index}"
                        disabled={step != 2}
                        bind:value={totp[index]}
                        on:input={() => {
                            if (totp[index] != null && totp[index] > 9) {
                                const code = String(totp[index]);
                                if (code.length == 6) {
                                    totp[0] = parseInt(code.charAt(0));
                                    totp[1] = parseInt(code.charAt(1));
                                    totp[2] = parseInt(code.charAt(2));
                                    totp[3] = parseInt(code.charAt(3));
                                    totp[4] = parseInt(code.charAt(4));
                                    totp[5] = parseInt(code.charAt(5));
                                    completeTotp();
                                } else {
                                    totp[index] = parseInt(code.charAt(index));
                                }
                            }
                            
                            if (totp[index] == null && index > 0) {
                                document.getElementById(`totp-${index - 1}`)?.focus();
                            } else if (totp[index] != null && index < 5) {
                                document.getElementById(`totp-${index + 1}`)?.focus();
                            } else if (totp[index] != null && index == 5) {
                                document.getElementById(`totp-${index}`)?.blur();
                                completeTotp();
                            }
                        }}
                    >
                {/each}
            </div>
        {/if}
        <button
            type="submit"
            class="bg-blue-500 text-white rounded-md cursor-pointer text-[17px] button"
            style="padding: 7.5px; width: {step < 2 ? 300 : 250}px; margin-top: {step < 2 ? 5 : 20}px;"
            class:disabled={step == 0 ? email == '' || password == '' : totp.map(c => c?.toString()).join('').length < 6 || step == 1 || step == 3}
            on:click={step == 0 ? login : completeTotp}
        >{step < 2 ? loginText : verifyText}</button>
    </form>
    {#if step < 2}
        <p class="text-[14px]" style="margin-top: 15px;">or</p>
        <a href="/register" class="text-[13px]" style="margin-top: 10px;">Don't have an account? <i>Register here!</i></a>
    {/if}
</div>

<style>
    /* Hide spin buttons in Chrome */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    /* Hide spin buttons in Firefox */
    input[type="number"] {
        -moz-appearance: textfield;
    }

    input:focus {
        outline: none;
        border: solid 1px rgb(59, 130, 246);
    }

    .button.disabled {
        opacity: 0.5;
        cursor: default;
    }
</style>