<script lang="ts">
	import RoleList from '../../lib/components/dashboard/RoleList.svelte';
	import TextField from "$lib/components/global/TextField.svelte";
	import { onMount } from 'svelte';
	import type AuthRsApi from '$lib/api';
	import User from '$lib/models/User';
	import type Role from '$lib/models/Role';
	import DateUtils from '$lib/dateUtils';

    export let api: AuthRsApi;
    export let user: User;
    export let roles: Role[];
    
    onMount(async () => {
        if (roles.length < 1) {
            api.getAllRoles()
                .then(r => roles = r)
                .catch(e => console.error(e));
        }
    });
</script>

<div class="flex flex-col items-start justify-start h-[100%] w-full gap-[10px]">
    <TextField label="Full Name" value={`${user.firstName} ${user.lastName}`} readonly={User.isSystemAdmin(user)} />
    <TextField label="Email" value={user.email} readonly={User.isSystemAdmin(user)} />
    <RoleList label="Roles" roles={roles.filter(r => user.roles.includes(r._id))} onAdd={() => {}} onRemove={() => {}} readOnly isSystemAdmin={User.isSystemAdmin(user)} />
    <TextField label="Creation Date" value={DateUtils.getFullDateString(User.getCreatedAt(user))} readonly />
</div>