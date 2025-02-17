<script lang="ts">
	import AuthStateManager from "$lib/auth";
	import { onMount } from "svelte";

    const authStateManager = new AuthStateManager();

    onMount(() => {
        const token = authStateManager.getToken();
        if (token) {
            window.location.href = '/users/@me';
        } else {
            window.location.href = '/login';
        }
    })
</script>