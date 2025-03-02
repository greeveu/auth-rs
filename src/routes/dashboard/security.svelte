<script lang="ts">
	import { ShieldCheck, ShieldX } from 'lucide-svelte';
	import type AuthRsApi from "$lib/api";
	import type UserMinimal from "$lib/models/User";

    export let api: AuthRsApi;
    export let user: UserMinimal;
</script>

<div class="flex flex-col items-center justify-start h-full" style="padding-top: 10%;">
    {#if user.mfa}
        <ShieldCheck size="120" class="text-green-600" />
    {:else}
        <ShieldX size="120" class="text-red-600" />
    {/if}
    <p class="text-[24px]" style="margin-top: 10px;">MFA is {user.mfa ? 'enabled' : 'disabled'}.</p>
</div>