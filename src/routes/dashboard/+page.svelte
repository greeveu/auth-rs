<script lang="ts">
	import { goto } from "$app/navigation";
	import AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import { ClipboardList, CodeXml, Link, User } from "lucide-svelte";
	import { onMount } from "svelte";

    const authStateManager = new AuthStateManager();
    const api = new AuthRsApi();

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
    
    onMount(() => {
        const token = authStateManager.getToken();
        if (token) {
            api.setToken(token);
            api.getCurrentUser()
                .then((data) => {
                    user = data;
                })
                .catch((error) => {
                    console.error(error);
                    api.setToken(null);
                    authStateManager.clearToken();
                    goto('/login?redirect_uri=/dashboard');
                });
        } else {
            goto('/login?redirect_uri=/dashboard');
        }
    })
</script>

<div class="flex w-screen h-screen items-center justify-center">
    <div class="flex flex-row items-center h-[80%] w-[70%] border-[2.5px] rounded-md" style="border-color: #333; padding: 10px;">
        <div class="flex flex-col gap-[15px] h-[90%]">
            {#each TABS.filter(t => t.requiredRoleId ? user?.roles.includes(t.requiredRoleId) : true) as tab, index}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div
                    class="flex flex-row items-center justify-start gap-[15px] w-[275px] bg-red-200 h-[40px] rounded-md"
                    class:active={currentTabIndex == index}
                    style="padding: 5px 10px;"
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
        <hr class="w-[2px] h-[95%] bg-[#333] rounded-[2px]" style="margin: 0 15px;">
    </div>
</div>