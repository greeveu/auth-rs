<script lang="ts">
	import Security from './security.svelte';
	import SidebarButton from '$lib/components/dashboard/SidebarButton.svelte';
	import Profile from './profile.svelte';
	import AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import { onMount } from "svelte";
	import UserMinimal from '$lib/models/User';

    const authStateManager = new AuthStateManager();
    let api = new AuthRsApi();

    let currentTabIndex = 0;

    let user: UserMinimal | null = null;
    let roles: Role[] = [];
    let connections: OAuthApplication[] = [];
    let OAuthApplications: OAuthApplication[] = [];
    let auditLogs: AuditLog[] = [];

    const TABS: {
        name: string;
        icon: string;
        requiredRoleId: string | null;
    }[] = [
        { name: 'Your Profile', icon: 'user', requiredRoleId: null },
        { name: 'Security', icon: 'shield', requiredRoleId: null },
        { name: 'Connections', icon: 'link', requiredRoleId: null },
        { name: 'OAuth Applications', icon: 'code-xml', requiredRoleId: null },
        { name: 'Logs', icon: 'clipboard-list', requiredRoleId: null },
        { name: 'SPACER', icon: '', requiredRoleId: UserMinimal.ADMIN_ROLE_ID },
        { name: 'Users', icon: 'users', requiredRoleId: UserMinimal.ADMIN_ROLE_ID },
        { name: 'Roles', icon: 'crown', requiredRoleId: UserMinimal.ADMIN_ROLE_ID },
        { name: 'Global Logs', icon: 'scroll-text', requiredRoleId: UserMinimal.ADMIN_ROLE_ID },
    ];
    
    onMount(async () => {
        const loadData = await authStateManager.handlePageLoad(['redirect_uri=/dashboard']);
        if (loadData) {
            api = loadData[0];
            user = loadData[1];
        }
    })
</script>

<div class="flex w-screen h-screen items-center justify-center">
    <div class="flex flex-row items-center h-[80%] w-[70%] border-[2.5px] border-[#333] rounded-md" style="padding: 10px;">
        <div class="flex flex-col justify-between h-[90%]">
            <div class="flex flex-col gap-[15px]">
                {#each TABS.filter(t => t.requiredRoleId ? user?.roles.includes(t.requiredRoleId) : true) as tab, index}
                    {#if tab.name == 'SPACER'}
                        <!-- svelte-ignore element_invalid_self_closing_tag -->
                        <div class="flex items-center justify-center w-[275px] h-[2px] bg-[#333]" style="margin-top: 20px;">
                            <p class="flex absolute text-center text-[14px] bg-black" style="padding: 0 10px;">Admin</p>
                        </div>
                    {:else}
                        <SidebarButton tab={tab} active={currentTabIndex == index} selectTab={() => currentTabIndex = index} />
                    {/if}
                {/each}
            </div>
            <SidebarButton tab={{ name: 'Logout', icon: 'log-out', requiredRoleId: null }} active={false} selectTab={() => authStateManager.logout()} isLogout={true} />
        </div>
        <!-- svelte-ignore element_invalid_self_closing_tag -->
        <div class="w-[4px] h-[90%] bg-[#333] rounded-[2px]" style="margin: 0 15px;" />
        <div class="flex flex-col h-[90%] w-full">
            {#if user && roles}
                <div class="flex items-center h-[75px]">
                    <p class="text-[14px]">> {TABS[currentTabIndex].requiredRoleId == UserMinimal.ADMIN_ROLE_ID ? 'Admin' : ''} Dashboard > {TABS[currentTabIndex].name}</p>
                </div>
                {#if currentTabIndex == 0}
                    <Profile bind:api bind:user={user!} bind:roles />
                {:else if currentTabIndex == 1}
                    <Security bind:api bind:user={user} />
                {/if}
            {/if}
        </div>
    </div>
</div>