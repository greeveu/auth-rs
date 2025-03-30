<script lang="ts">
	import type AuthRsApi from "$lib/api";
	import AuditLogEntry from "$lib/components/dashboard/AuditLogEntry.svelte";
	import { AuditLog } from "$lib/models/AuditLog";
	import type OAuthApplication from "$lib/models/OAuthApplication";
	import type Role from "$lib/models/Role";
	import User from "$lib/models/User";
	import { onMount } from "svelte";

    export let api: AuthRsApi;
    export let user: User;
    export let users: User[];
    export let roles: Role[];
    export let applications: OAuthApplication[];
    export let auditLogs: AuditLog[];
    export let isGlobalLogs: boolean = false;

    onMount(async () => {
        api.getAuditLogs(isGlobalLogs ? null : user).then((newAuditLogs) => {
            auditLogs = newAuditLogs;
            if (isGlobalLogs) {
                // TODO: request users individually when making audit logs paginated
                api.getUsers().then((newUsers) => {
                    users = newUsers;
                }).catch((err) => {
                    console.error(err);
                });
            }
            api.getOAuthApplications().then((newApplications) => {
                applications = newApplications;
            }).catch((err) => {
                console.error(err);
            });
        }).catch((err) => {
            console.error(err);
        });
    })
</script>

<div class="flex flex-col overflow-y-scroll gap-[15px]">
    {#each auditLogs.reverse() as auditLog}
        <AuditLogEntry user={user} auditLog={auditLog} users={users} roles={roles} applications={applications} />
    {/each}
</div>