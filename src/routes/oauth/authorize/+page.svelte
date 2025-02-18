<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
	import type AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import type OAuthApplication from "$lib/models/OAuthApplication";
    import { Bot, Link, User, UserPen, UserMinus, UserCog } from "lucide-svelte";

    let api: AuthRsApi | null = null;
    let user: UserMinimal | null = null;

    let oAuthData: {
        clientId: string;
        state: string;
        scopes: string[];
        redirect: string
    } = {
        clientId: '',
        state: '',
        scopes: [],
        redirect: '',
    };
    let oAuthApplication: OAuthApplication | null = null;

    const SCOPES: Record<string, { icon: string; description: string }> = {
        'user:read': { icon: 'user', description: 'Read your profile data. (e.g. name or email)' },
        'user:update': { icon: 'user-pen', description: 'Change your profile data. (e.g. name)' },
        'user:delete': { icon: 'user-minus', description: 'Delete your profile.' },
        'user:*': { icon: 'user-cog', description: 'Read and manage your entire profile.' },
    };

    onMount(async () => {
        const url = new URL(window.location.href);
        const clientId = url.searchParams.get('client_id');
        const state = url.searchParams.get('state');
        const scope = url.searchParams.get('scope');
        const redirect = url.searchParams.get('redirect_uri');

        if (!clientId || !state || !scope || !redirect) {
            console.error('Missing parameters!');
            goto(redirect ?? '/');
            return;
        }

        const pageData = await new AuthStateManager().handlePageLoad([`redirect_uri=${url}`]);
        api = pageData?.[0] ?? null;
        user = pageData?.[1] ?? null;

        if (!api || !user) {
            console.error('Failed to load page data!');
            goto(redirect);
            return;
        }

        oAuthData = {
            clientId,
            state,
            scopes: scope.split(','),
            redirect,
        };

        api.getOAuthApplication(clientId)
            .then((app) => {
                oAuthApplication = app;
            })
            .catch((err) => {
                console.error('Failed to load OAuth application data!', err);
                goto(redirect);
            });
    });
</script>

<div class="flex items-center justify-center h-screen w-screen">
    <div class="flex flex-col items-center border-white border-1 rounded-md" style="padding: 30px;">
        <div class="flex flex-col gap-2 items-center">
            <div class="flex flex-row items-center justify-center gap-[15px]" style="margin-bottom: 30px;">
                <Bot class="size-[80px]" />
                <p class="opacity-50" style="letter-spacing: 5px;">...</p>
                <Link class="size-[25px] opacity-75" />
                <p class="opacity-50" style="letter-spacing: 5px;">...</p>
                <User class="size-[80px]" />
            </div>
            <h1 class="font-bold text-2xl" style="margin-bottom: 10px;">{oAuthApplication?.name}</h1>
            <h2 class="opacity-50 text-[14px]">wants to access your Account</h2>
            <div class="flex flex-row gap-[10px] text-[12px]">
                <p class="opacity-50">Signed in as</p>
                <p class="opacity-85">{user?.firstName} {user?.lastName}</p>
                <a class="text-blue-400" style="margin-left: 7.5px;" href="/logout">Not you?</a>
            </div>
        </div>
        <hr class="h-[2px] w-full bg-white opacity-25" style="margin: 15px;" />
        <div class="flex flex-col items-center gap-[10px]" style="padding: 5px;">
            {#each oAuthData.scopes as scope}
            <div class="flex flex-row items-start w-full gap-[15px]">
                {#if SCOPES[scope].icon === 'user'}
                    <User class="w-[20px] h-[20px]"  />
                {:else if SCOPES[scope].icon === 'user-pen'}
                    <UserPen class="w-[20px] h-[20px]" />
                {:else if SCOPES[scope].icon === 'user-minus'}
                    <UserMinus class="w-[20px] h-[20px]" />
                {:else if SCOPES[scope].icon === 'user-cog'}
                    <UserCog class="w-[20px] h-[20px]" />
                {/if}
                <p class="text-[14px]">{SCOPES[scope].description}</p>
            </div>
            {/each}
        </div>
        <hr class="h-[2px] w-full bg-white opacity-25" style="margin: 15px;" />
        <div class="flex flex-row items-center justify-between w-full" style="margin-top: 5px;">
            <h1 class="text-red-500 cursor-pointer btn cancel">Cancel</h1>
            <h1 class="text-green-500 font-bold cursor-pointer btn authorize">Authorize</h1>
        </div>
    </div>
</div>

<style>
    .btn {
        transition: all 0.2s;
    }
    
    .btn:hover {
        opacity: 75%;
    }
</style>

