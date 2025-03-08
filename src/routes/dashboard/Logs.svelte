<script lang="ts">
	import type AuthRsApi from "$lib/api";
	import type UserMinimal from "$lib/models/User";
	import { onMount } from "svelte";

    export let api: AuthRsApi;
    export let user: UserMinimal;
    export let auditLogs: AuditLog[];

    onMount(async () => {
        api.getAuditLogs(user).then((newAuditLogs) => {
            console.log(auditLogs);
            auditLogs = newAuditLogs;
        }).catch((err) => {
            console.error(err);
        });
    })
</script>