<script lang="ts">
	import type AuthRsApi from "$lib/api";
	import AuditLogEntry from "$lib/components/dashboard/AuditLogEntry.svelte";
	import { AuditLog } from "$lib/models/AuditLog";
	import type OAuthApplication from "$lib/models/OAuthApplication";
	import type RegistrationToken from "$lib/models/RegistrationToken";
	import type Role from "$lib/models/Role";
	import User from "$lib/models/User";
	import { onMount } from "svelte";

    export let api: AuthRsApi;
    export let user: User;
    export let users: User[];
    export let roles: Role[];
    export let applications: OAuthApplication[];
    export let registrationTokens: RegistrationToken[];
    export let auditLogs: AuditLog[];
    export let isGlobalLogs: boolean = false;

    onMount(async () => {
        api.getAuditLogs(isGlobalLogs ? null : user).then((newAuditLogs) => {
            auditLogs = newAuditLogs;
            if (isGlobalLogs && users.length <= 0) {
                // TODO: request users individually when making audit logs paginated
                api.getUsers().then((newUsers) => {
                    users = newUsers;
                }).catch((err) => {
                    console.error(err);
                });
            }
            if (roles.length <= 0) {
                api.getAllRoles().then((newRoles) => {
                    roles = newRoles;
                }).catch((err) => {
                    console.error(err);
                });
            }
            if (registrationTokens.length <= 0) {
                api.getAllRegistrationTokens().then((newRegistrationTokens) => {
                    registrationTokens = newRegistrationTokens;
                }).catch((err) => {
                    console.error(err);
                });
            }
            if (applications.length <= 0) {
                api.getOAuthApplications().then((newApplications) => {
                    applications = newApplications;
                }).catch((err) => {
                    console.error(err);
                });
            }
        }).catch((err) => {
            console.error(err);
        });
    })
</script>

<div class="flex flex-col overflow-y-scroll gap-[15px]">
    {#each auditLogs.reverse() as auditLog}
        <AuditLogEntry {user} {auditLog} {users} {roles} {applications} {registrationTokens} />
    {/each}
</div>