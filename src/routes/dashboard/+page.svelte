<script lang="ts">
	import Security from './Security.svelte';
	import SidebarButton from '$lib/components/dashboard/SidebarButton.svelte';
	import Profile from './Profile.svelte';
	import AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import { onMount } from "svelte";
	import UserMinimal from '$lib/models/User';
	import type OAuthApplication from '$lib/models/OAuthApplication';
	import Connections from './Connections.svelte';
	import type OAuthConnection from '$lib/models/OAuthConnection';
	import Applications from './Applications.svelte';
	import Logs from './Logs.svelte';
	import type { AuditLog } from '$lib/models/AuditLog';
	import Roles from './Roles.svelte';
	import type Role from '$lib/models/Role';
	import Users from './Users.svelte';
	import type Settings from '$lib/models/Settings';

    const authStateManager = new AuthStateManager();
    let api = new AuthRsApi();

    let currentTabIndex = 0;

    let settings: Settings | null = null;
    let user: UserMinimal | null = null;
    let users: UserMinimal[] = [];
    let roles: Role[] = [];
    let connections: OAuthConnection[] = [];
    let applications: OAuthApplication[] = [];
    let auditLogs: AuditLog[] = [];

    const TABS: {
        name: string;
        icon: string;
        shouldShow: (user: UserMinimal, settings: Settings) => boolean;
    }[] = [
        { name: 'Your Profile', icon: 'user', shouldShow: () => true },
        { name: 'Security', icon: 'shield', shouldShow: () => true },
        { name: 'Connections', icon: 'link', shouldShow: (user) => !UserMinimal.isSystemAdmin(user) },
        { name: 'OAuth Applications', icon: 'code-xml', shouldShow: (user, settings) => settings.allowOauthAppsForUsers || UserMinimal.isAdmin(user) },
        { name: 'Logs', icon: 'clipboard-list', shouldShow: () => true },
        { name: 'SPACER', icon: '', shouldShow: (user) => UserMinimal.isAdmin(user) },
        { name: 'Users', icon: 'users', shouldShow: (user) => UserMinimal.isAdmin(user) },
        { name: 'Roles', icon: 'crown', shouldShow: (user) => UserMinimal.isAdmin(user) },
        { name: 'All OAuth Apps', icon: 'code-xml', shouldShow: (user) => UserMinimal.isAdmin(user) },
        { name: 'Registration Codes', icon: 'ticket-check', shouldShow: (user, settings) => UserMinimal.isAdmin(user) && !settings.allowRegistration },
        { name: 'Global Logs', icon: 'scroll-text', shouldShow: (user) => UserMinimal.isAdmin(user) },
        { name: 'System Settings', icon: 'settings', shouldShow: (user) => UserMinimal.isSystemAdmin(user) },
    ];
    
    onMount(async () => {
        const loadData = await authStateManager.handlePageLoad(['redirect_uri=/dashboard']);
        if (loadData) {
            api = loadData[0];
            user = loadData[1];
            settings = await api.getSettings();
        }
    })
</script>

<div class="flex w-screen h-screen items-center justify-center">
    <div class="flex flex-row items-center h-[80%] w-[70%] border-[2.5px] border-[#333] rounded-md" style="padding: 10px;">
        <div class="flex flex-col justify-between h-[90%]">
            <div class="flex flex-col gap-[15px]">
                {#each TABS as tab, index}
                    {#if user && settings && tab.shouldShow(user, settings)}
                        {#if tab.name == 'SPACER'}
                            <!-- svelte-ignore element_invalid_self_closing_tag -->
                            <div class="flex items-center justify-center w-[275px] h-[2px] bg-[#333]" style="margin-top: 20px;">
                                <p class="flex absolute text-center text-[14px] bg-black" style="padding: 0 10px;">Admin</p>
                            </div>
                        {:else}
                            <SidebarButton tab={tab} active={currentTabIndex == index} selectTab={() => currentTabIndex = index} />
                        {/if}
                    {/if}
                {/each}
            </div>
            <SidebarButton tab={{ name: 'Logout', icon: 'log-out' }} active={false} selectTab={() => authStateManager.logout()} isLogout={true} />
        </div>
        <!-- svelte-ignore element_invalid_self_closing_tag -->
        <div class="w-[4px] h-[90%] bg-[#333] rounded-[2px]" style="margin: 0 15px;" />
        <div class="flex flex-col h-[90%] w-full">
            {#if user && roles}
                <div class="flex items-center min-h-[75px]">
                    <p class="text-[14px]">>{currentTabIndex > 4 ? ' Admin' : ''} Dashboard > {TABS[currentTabIndex].name}</p>
                </div>
                {#if currentTabIndex == 0}
                    <Profile bind:api bind:user bind:roles />
                {:else if currentTabIndex == 1}
                    <Security bind:api bind:user />
                {:else if currentTabIndex == 2}
                    <Connections bind:api bind:user bind:connections />
                {:else if currentTabIndex == 3}
                    <Applications bind:api bind:user bind:applications onlyShowOwned={true} />
                {:else if currentTabIndex == 4}
                    <Logs bind:api bind:user bind:users bind:roles bind:applications bind:auditLogs />
                {:else if currentTabIndex == 6}
                    <Users bind:api bind:currentUser={user} bind:users bind:roles />
                {:else if currentTabIndex == 7}
                    <Roles bind:api bind:roles />
                {:else if currentTabIndex == 8}
                    <Applications bind:api bind:user bind:applications onlyShowOwned={false} />
                {:else if currentTabIndex == 9}
                    <Logs bind:api bind:user bind:users bind:roles bind:applications bind:auditLogs />
                {/if}
            {/if}
        </div>
    </div>
</div>