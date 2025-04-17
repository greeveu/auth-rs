<script lang="ts">
    import AuthStateManager from "$lib/auth";
	import { apiUrl } from "$lib/store/config";
    import { onMount } from "svelte";

    let redirectCooldown = 3;
    let redirect: string = '/login';

    onMount(() => {
        new AuthStateManager($apiUrl).clearToken();

        redirect =  new URL(window.location.href).searchParams.get('redirect_uri') ?? '/login';
        setInterval(() => {
            Math.max(0, redirectCooldown--);
            if (redirectCooldown == 0) {
                window.location.href = redirect;
            }
        }, 1000);
    })
</script>

<div class="flex flex-col gap-[20px] items-center justify-center h-screen w-screen">
    <h1 class="text-3xl">Successfully logged out!</h1>
    <p>Redirecting to <i>{redirect}</i> in {redirectCooldown > 0 ? redirectCooldown : 'now'}...</p>
</div>