<script lang="ts">
	import AuthRsApi from "$lib/api";
	import AuthStateManager from "$lib/auth";
	import { onMount } from "svelte";

    const authStateManager = new AuthStateManager();
    const api = new AuthRsApi();

    let user: UserMinimal | null = null;
    let roles: Role[] = [];

    async function loadData() {
        api.getAllRoles()
            .then((data) => {
                roles = data;
            })
            .catch((error) => {
                console.error(error);
            });
    }

    onMount(() => {
        const token = authStateManager.getToken();
        if (token) {
            api.setToken(token);
            api.getCurrentUser()
                .then((data) => {
                    user = data;
                    loadData();
                })
                .catch((error) => {
                    console.error(error);
                    api.setToken(null);
                    authStateManager.clearToken();
                    window.location.href = '/login';
                });
        } else {
            window.location.href = '/login';
        }
    })
</script>

<h1>Welcome, {user?.firstName} {user?.lastName}!</h1>
<p>You have the following roles: {roles.filter(r => user?.roles.includes(r._id)).map(r => r.name).join(', ')}</p>