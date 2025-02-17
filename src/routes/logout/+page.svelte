<script lang="ts">
    import AuthStateManager from "$lib/auth";
    import { onMount } from "svelte";

    const authStateManager = new AuthStateManager();

    onMount(() => {
        authStateManager.clearToken();
    })
</script>

<div class="flex items-center justify-center">
    <h1>Successfully logged out!</h1>
</div>