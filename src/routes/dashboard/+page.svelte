<script lang="ts">
	import SidebarButton from '$lib/components/dashboard/SidebarButton.svelte';
	import Profile from './profile.svelte';
	import AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import { onMount } from "svelte";
	import type UserMinimal from '$lib/models/User';

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
        { name: 'Users', icon: 'users', requiredRoleId: '00000000-0000-0000-0000-00000000000' },
        { name: 'Roles', icon: 'crown', requiredRoleId: '00000000-0000-0000-0000-00000000000' },
        { name: 'Global Logs', icon: 'scroll-text', requiredRoleId: '00000000-0000-0000-0000-00000000000' },
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
                    <SidebarButton tab={tab} active={currentTabIndex == index} selectTab={() => currentTabIndex = index} />
                {/each}
            </div>
            <SidebarButton tab={{ name: 'Logout', icon: 'log-out', requiredRoleId: null }} active={false} selectTab={() => authStateManager.logout()} isLogout={true} />
        </div>
        <!-- svelte-ignore element_invalid_self_closing_tag -->
        <div class="w-[4px] h-[90%] bg-[#333] rounded-[2px]" style="margin: 0 15px;" />
        <div class="flex flex-col h-[90%]">
            {#if user && roles}
                <div class="flex items-center h-[75px]">
                    <p class="text-[14px]">> Dashboard > {TABS[currentTabIndex].name}</p>
                </div>
                {#if currentTabIndex == 0}
                    <Profile bind:api bind:user={user!} bind:roles />
                {/if}
            {/if}
        </div>
    </div>
</div>