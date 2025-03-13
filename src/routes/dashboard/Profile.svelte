<script lang="ts">
	import RoleList from '../../lib/components/dashboard/RoleList.svelte';
	import TextField from "$lib/components/dashboard/TextField.svelte";
	import { onMount } from 'svelte';
	import type AuthRsApi from '$lib/api';
	import UserMinimal from '$lib/models/User';
	import UserUpdates from '$lib/models/UserUpdates';
	import type Role from '$lib/models/Role';

    export let api: AuthRsApi;
    export let user: UserMinimal;
    export let roles: Role[];
    
    function addRole() {
        // FIXME: This is a temporary solution. The user should be able to select a role from a dropdown.
        const id = prompt('Enter the role ID:');
        if (id) {
            const role = roles.find(r => r._id == id);
            if (role) {
                const updates = new UserUpdates({ 
                    email: null, 
                    password: null, 
                    firstName: null, 
                    lastName: null, 
                    roles: [...user.roles, role._id], 
                    disabled: null 
                });
                api.updateUser(user, updates)
                    .then(u => user = u)
                    .catch(e => console.error(e));
            }
        }
    }

    function removeRole(role: Role) {
        const updates = new UserUpdates({ 
            email: null, 
            password: null, 
            firstName: null, 
            lastName: null, 
            roles: roles.filter(r => r._id != role._id).map(r => r._id), 
            disabled: null 
        });
        api.updateUser(user, updates)
            .then(u => user = u)
            .catch(e => console.error(e));
    }

    onMount(async () => {
        if (roles.length < 1) {
            api.getAllRoles()
                .then(r => roles = r)
                .catch(e => console.error(e));
        }
    });
</script>

<div class="flex flex-col items-start justify-start h-[100%] w-full gap-[10px]">
    <TextField label="Full Name" value={`${user.firstName} ${user.lastName}`} readonly={UserMinimal.isSystemAdmin(user)} />
    <TextField label="Email" value={user.email} readonly={UserMinimal.isSystemAdmin(user)} />
    <RoleList label="Roles" roles={roles.filter(r => user.roles.includes(r._id))} onAdd={addRole} onRemove={removeRole} readOnly={!UserMinimal.isAdmin(user)} isSystemAdmin={UserMinimal.isSystemAdmin(user)} />
    <TextField label="Creation Date" value={`${UserMinimal.getCreatedAt(user).getDate()}.${UserMinimal.getCreatedAt(user).getMonth() + 1}.${UserMinimal.getCreatedAt(user).getFullYear()} ${UserMinimal.getCreatedAt(user).getHours()}:${UserMinimal.getCreatedAt(user).getMinutes()}:${UserMinimal.getCreatedAt(user).getSeconds()}`} readonly />
</div>