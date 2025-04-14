<script lang="ts">
    import AuthRsApi from '$lib/api';
	import AuthStateManager from '$lib/auth';
	import { onMount } from "svelte";
	import type Settings from '$lib/models/Settings';
	import TextInput from '$lib/components/global/TextInput.svelte';
	import { goto } from '$app/navigation';
	import { Circle } from 'svelte-loading-spinners';
    import { Info, TicketCheck } from 'lucide-svelte';

    const authStateManager = new AuthStateManager();
    const api = new AuthRsApi();
    let settings: Settings | null = null;
    let step = 0;

    let firstName = '';
    let lastName = '';
    let email = '';
    let password = '';
    let confirmPassword = '';

    let isLoading = false;

    let redirect: string | null = null;
    let registrationCode: string | null = null;

    $: dataIsValid = email != '' && password != '' && confirmPassword != '' && firstName != '' && lastName != '' && email.includes('@') && email.includes('.') && password.length >= 8 && password == confirmPassword;

    async function register() {
        if (!dataIsValid) {
            console.error('Incomplete registration form.');
            return;
        }

        step = 1;
        isLoading = true;

        api.createUser(email, password, firstName, lastName, registrationCode)
            .then(async (data) => {
                window.location.href = `/login${redirect ? `?redirect_uri=${redirect}` : ''}`;
            })
            .catch((error) => {
                step = 0;
                isLoading = false;
                password = '';
                confirmPassword = '';
                console.error(error);
            });
    }

    onMount(async () => {
        redirect = new URL(window.location.href).searchParams.get('redirect_uri');
        registrationCode = new URL(window.location.href).searchParams.get('registration_code');

        settings = await api.getSettings();

        if (!settings!.openRegistration && !registrationCode) {
            goto(`/login${redirect ? `?redirect_uri=${redirect}` : ''}`);
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
    >Register</h1>
    <form id="form" class="flex flex-col items-center justify-center mt-4">
        <TextInput type="email" label="Email" bind:value={email} autocomplete="email" />
        <TextInput type="text" label="First Name" bind:value={firstName} autocomplete="name" autofocus />
        <TextInput type="text" label="Last Name" bind:value={lastName} autocomplete="family-name" />
        <TextInput type="password" label="Password" bind:value={password} autocomplete="new-password" />
        <TextInput type="password" label="Confirm Password" bind:value={confirmPassword} autocomplete="new-password" />
        {#if registrationCode != null}
            <div class="flex flex-row items-center justity-start w-full gap-[10px]" style="margin-bottom: 10px;">
                <TicketCheck size="17.5" class="opacity-50" />
                <p class="text-[14px] opacity-50">Registration code: <i>{registrationCode}</i></p>
            </div>
        {/if}
        <button
            type="submit"
            class="border-[1.5px] border-blue-500 bg-blue-500 text-white rounded-md text-[17px] opacity-50 cursor-default transition-all"
            style="padding: 7.5px; width: 300px; margin-top: 5px;"
            class:opacity-100={step == 0 && dataIsValid}
            class:cursor-pointer={step == 0 && dataIsValid}
            class:hover:bg-transparent={step == 0 && dataIsValid}
            class:hover:text-blue-500={step == 0 && dataIsValid}
            on:click={register}
        >
            <div class="flex flex-row items-center justify-center gap-[10px]">
                {#if !isLoading}
                    <p class="text-[17px]">Register</p>
                {:else}
                    <Circle color="var(--color-blue-500)" size=15 />
                    <p class="text-[17px]">Registering</p>
                {/if}
            </div>
        </button>
    </form>
    <p class="text-[14px]" style="margin-top: 15px;">or</p>
    <a href={`/login${redirect ? `?redirect_uri=${redirect}` : ''}`} class="text-[13px]" style="margin-top: 10px;">Already have an account? <i>Log In here!</i></a>
</div>