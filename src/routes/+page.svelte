<script lang="ts">
	import { goto } from "$app/navigation";
	import AuthStateManager from "$lib/auth";
	import { apiUrl } from "$lib/store/config";
	import { onMount } from "svelte";

    onMount(async () => {
        const loadData = await new AuthStateManager($apiUrl).handlePageLoad();
        if (loadData && loadData.length > 1) {
            goto('/dashboard');
        }
    })
</script>
