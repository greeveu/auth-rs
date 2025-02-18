<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
	import type AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import type OAuthApplication from "$lib/models/OAuthApplication";
    import { Bot, Link, User, UserPen, UserMinus, UserCog, SquareArrowOutUpRight, Clock } from "lucide-svelte";

    let api: AuthRsApi | null = null;
    let user: UserMinimal | null = null;

    let oAuthData: {
        clientId: string;
        state: string;
        scopes: string[];
        redirect: string,
        redirectBase: string,
        activeSince: string,
    } = {
        clientId: '',
        state: '',
        scopes: [],
        redirect: '',
        redirectBase: '',
        activeSince: ''
    };
    let oAuthApplication: OAuthApplication | null = null;

    let step = 0;

    const SCOPES: Record<string, { icon: string; description: string }> = {
        'user:read': { icon: 'user', description: 'Read your profile data (e.g. name or email)' },
        'user:update': { icon: 'user-pen', description: 'Change your profile data (e.g. name)' },
        'user:delete': { icon: 'user-minus', description: 'Delete your profile' },
        'user:*': { icon: 'user-cog', description: 'Read and manage your entire profile' },
    };

    async function authorize() {
        step = 1;
        api?.authorizeOAuthApplication(oAuthData.clientId, oAuthData.redirect, oAuthData.scopes)
            .then((data) => {
                window.location.href = `${oAuthData.redirect}?code=${data.code}&state=${oAuthData.state}`;
            })
            .catch((err) => {
                step = 0;
                console.error('Failed to authorize OAuth application!', err);
            });
    }

    function cancel() {
        window.location.href = oAuthData.redirectBase;
    }

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
            redirectBase: `${redirect.split('//')[0]}//${redirect.split('//')[1].split('/')[0]}`,
            activeSince: ''
        };

        api.getOAuthApplication(clientId)
            .then((app) => {
                oAuthApplication = app;
                const createdAt = app.createdAt.split(' ')[0].split('-')
                oAuthData.activeSince = `${createdAt[2]}.${createdAt[1]}.${createdAt[0]}`;
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
            <div class="flex flex-row items-center justify-center gap-[10px]" style="margin-bottom: 20px;">
                <Bot class="size-[80px]" />
                <p class="opacity-50" style="letter-spacing: 3.5px;">....</p>
                <Link class="size-[20px] opacity-75" style="margin-top: 5px;" />
                <p class="opacity-50" style="letter-spacing: 3.5px;">....</p>
                <User class="size-[80px]" />
            </div>
            <h1 class="font-extrabold text-2xl" style="margin-bottom: 5px;">{oAuthApplication?.name}</h1>
            <h2 class="opacity-50 text-[14px]">wants to access your Account.</h2>
            <div class="flex flex-row gap-[10px] text-[12px]">
                <p class="opacity-50">Signed in as</p>
                <p class="opacity-85">{user?.firstName} {user?.lastName}</p>
                <a class="text-blue-400" style="margin-left: 7.5px;" href="/logout?redirect_uri={oAuthData.redirectBase}">Not you?</a>
            </div>
        </div>
        <hr class="h-[2px] w-full bg-white opacity-25" style="margin: 15px;" />
        <div class="flex flex-col items-center gap-[10px]" style="padding: 7.5px;">
            {#each oAuthData.scopes as scope}
                <div class="flex flex-row items-start w-full gap-[15px]">
                    {#if SCOPES[scope].icon == 'user'}
                        <User class="w-[20px] h-[20px]"  />
                    {:else if SCOPES[scope].icon == 'user-pen'}
                        <UserPen class="w-[20px] h-[20px]" />
                    {:else if SCOPES[scope].icon == 'user-minus'}
                        <UserMinus class="w-[20px] h-[20px]" />
                    {:else if SCOPES[scope].icon == 'user-cog'}
                        <UserCog class="w-[20px] h-[20px]" />
                    {/if}
                    <p class="text-[14px]">{SCOPES[scope].description}</p>
                </div>
            {/each}
        </div>
        <hr class="h-[2px] w-full bg-white opacity-25" style="margin: 10px;" />
        <div class="flex flex-col items-start justify-center w-full max-w-[450px] gap-[10px]" style="padding: 2.5px 17.5px;">
            <div class="flex flex-row items-center gap-[15px]">
                <SquareArrowOutUpRight class="w-[20px] h-[20px] opacity-50" />
                <p class="text-[11px]">
                    <span class="opacity-50">Once you authorize you will be redirected to:</span>
                    <span class="font-bold opacity-85">{oAuthData.redirectBase}</span>
                </p>
            </div>
            <div class="flex flex-row items-center gap-[12.5px]">
                <Clock class="w-[17.5px] h-[17.5px] opacity-50" />
                <p class="text-[11px] opacity-85">{oAuthApplication?.name} <span class="opacity-70">is active since</span> {oAuthData.activeSince}</p>
            </div>
        </div>
        <hr class="h-[2px] w-full bg-white opacity-25" style="margin: 10px;" />
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <div class="flex flex-row items-center justify-between w-full" style="margin-top: 10px;">
            <h1
                class="text-red-500 cursor-pointer btn cancel"
                class:opacity-50={step != 0}
                on:click={step == 0 ? cancel : null}
            >Cancel</h1>
            <h1
                class="text-green-500 font-bold cursor-pointer btn authorize"
                class:opacity-50={step != 0}
                on:click={step == 0 ? authorize : null}
            >{step == 0 ? 'Authorize' : 'Authorizing...'}</h1>
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

