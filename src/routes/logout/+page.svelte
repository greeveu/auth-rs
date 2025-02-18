<script lang="ts">
    import AuthStateManager from "$lib/auth";
    import { onMount } from "svelte";

    let redirectCooldown = 5;
    let redirect: string | null = null;

    onMount(() => {
        new AuthStateManager().clearToken();

        redirect =  new URL(window.location.href).searchParams.get('redirect_uri');
        if (redirect) {
            setInterval(() => {
                redirectCooldown--;
                if (redirectCooldown <= 0) {
                    window.location.href = redirect!;
                }
            }, 1000);
        }
    })
</script>

<div class="flex flex-col gap-[20px] items-center justify-center h-screen w-screen">
    <h1 class="text-3xl">Successfully logged out!</h1>
    {#if redirect}
        <p>Redirecting to <i>{redirect}</i> in {redirectCooldown}...</p>
    {/if}
</div>