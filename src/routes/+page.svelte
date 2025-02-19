<script lang="ts">
	import { goto } from "$app/navigation";
	import AuthStateManager from "$lib/auth";
	import { onMount } from "svelte";

    onMount(async () => {
        const loadData = await new AuthStateManager().handlePageLoad();
        if (loadData && loadData.length > 1) {
            goto('/dashboard');
        }
    })
</script>