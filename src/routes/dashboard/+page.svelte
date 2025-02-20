<script lang="ts">
	import Profile from './profile.svelte';
	import AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import { ClipboardList, CodeXml, Link, User } from "lucide-svelte";
	import { onMount } from "svelte";

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
        { name: 'Connections', icon: 'link', requiredRoleId: null },
        { name: 'OAuth Applications', icon: 'code-xml', requiredRoleId: null },
        { name: 'Logs', icon: 'clipboard-list', requiredRoleId: null },
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
        <div class="flex flex-col gap-[15px] h-[90%]">
            {#each TABS.filter(t => t.requiredRoleId ? user?.roles.includes(t.requiredRoleId) : true) as tab, index}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div
                    class="flex flex-row items-center justify-start gap-[15px] w-[275px] border-[2px] border-[#222] rounded-md cursor-pointer hover:text-blue-500 hover:border-blue-500 transition-all"
                    class:active={currentTabIndex == index}
                    style="padding: 10px 15px;"
                    on:click={() => currentTabIndex = index}
                >
                    {#if tab.icon == 'user'}
                        <User height="30" width="30" />
                    {:else if tab.icon == 'link'}
                        <Link height="30" width="30" />
                    {:else if tab.icon == 'code-xml'}
                        <CodeXml height="30" width="30" />
                    {:else if tab.icon == 'clipboard-list'}
                        <ClipboardList height="30" width="30" />
                    {/if}
                    <p>{tab.name}</p>
                </div>
            {/each}
        </div>
        <!-- svelte-ignore element_invalid_self_closing_tag -->
        <div class="w-[4px] h-[90%] bg-[#333] rounded-[2px]" style="margin: 0 15px;" />
        {#if user && roles}
            {#if currentTabIndex == 0}
                <Profile bind:user={user!} bind:roles />
            {/if}
        {/if}
    </div>
</div>

<style>
    .active {
        background-color: var(--color-blue-500);
        border-color: var(--color-blue-500);
    }

    .active:hover {
        color: white;
        cursor: default;
    }
</style>