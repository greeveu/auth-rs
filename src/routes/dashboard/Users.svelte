<script lang="ts">
	import Popup from '../../lib/components/global/Popup.svelte';
	import type AuthRsApi from "$lib/api";
	import { PackageOpen, Pen, Trash } from "lucide-svelte";
	import { onMount } from "svelte";
	import UserMinimalUpdates from '$lib/models/UserUpdates';
	import UserMinimal from '$lib/models/User';
	import TextField from '$lib/components/dashboard/TextField.svelte';
	import RoleList from '$lib/components/dashboard/RoleList.svelte';
	import type Role from '$lib/models/Role';

    export let api: AuthRsApi;
    export let users: UserMinimal[];
    export let roles: Role[];

    let showNewUserPopup: boolean = false;
    let newUser: UserMinimal | null = null;
    let newUserEmail: string = '';
    let newUserFirstName: string = '';
    let newUserLastName: string = '';
    let newUserPassword: string = '';
    let newUserPasswordConfirm: string = '';

    let editUserPopup: boolean = false;
    let editUser: UserMinimal | null = null;
    let editUserEmail: string = '';
    let editUserFirstName: string = '';
    let editUserLastName: string = '';
    let editUserPassword: string = '';

    let deleteUserPopup: boolean = false;
    let deleteUser: UserMinimal | null = null;

    function openCreateUserPopup() {
        newUser = null;
        newUserEmail = '';
        newUserFirstName = '';
        newUserLastName = '';
        newUserPassword = '';
        newUserPasswordConfirm = '';
        showNewUserPopup = true;
    }

    function userDataIsValid(): boolean {
        const emailValid = newUserEmail.length > 3 && newUserEmail.includes('@') && newUserEmail.includes('.');
        const nameValid = newUserFirstName.length > 3 && newUserLastName.length > 3;
        const passwordValid = newUserPassword.length > 7 && newUserPassword === newUserPasswordConfirm;

        return emailValid && nameValid && passwordValid;
    }

    onMount(() => {
        api.getAllUsers()
            .then(allUsers => users = allUsers)
            .catch(e => console.error(e));
    });
</script>

{#if showNewUserPopup}
    <Popup title="Create User" onClose={() => showNewUserPopup = false}>
        <div class="flex flex-col items-center justify-center min-w-[350px] max-w-[400px]">
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Email</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Name"
                bind:value={newUserEmail}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">First Name</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="First Name"
                bind:value={newUserFirstName}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Last Name</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Last Name"
                bind:value={newUserLastName}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Password</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Password"
                bind:value={newUserPassword}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Confirm Password</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Confirm Password"
                bind:value={newUserPasswordConfirm}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-green-600 rounded-md {userDataIsValid() ? 'cursor-pointer' : 'cursor-default opacity-50'} text-[18px] button green-button"
                style="margin-top: 25px; margin-bottom: 10px;"
                on:click={userDataIsValid() ? () => {
                    showNewUserPopup = false;
                    api.createUser(newUserEmail, newUserFirstName, newUserLastName, newUserPassword)
                        .then(createdUser => {
                            newUser = createdUser;
                            users = [...users, createdUser]
                        })
                        .catch(console.error);
                } : null}
            >Create</p>
        </div>
    </Popup>
{/if}

{#if editUserPopup}
    <Popup title="Edit User" onClose={() => editUserPopup = false}>
        <div class="flex flex-col items-center justify-center min-w-[350px]">
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Email</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Email"
                bind:value={editUserEmail}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">First Name</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="First Name"
                bind:value={editUserFirstName}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Last Name</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Last Name"
                bind:value={editUserLastName}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <p class="text-[14px] self-start h-[17.5px] opacity-50">Password</p>
            <!-- svelte-ignore a11y_autofocus -->
            <input
                type="text"
                placeholder="Password"
                bind:value={editUserPassword}
                class="border-[1.5px] border-gray-300 rounded-md opacity-75 w-full"
                style="padding: 5px 10px; margin-top: 5px; margin-bottom: 10px;"
                autofocus
            >
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-green-600 rounded-md {userDataIsValid() ? 'cursor-pointer' : 'cursor-default opacity-50'} text-[18px] button green-button"
                style="margin-top: 20px; margin-bottom: 10px;"
                on:click={userDataIsValid() ? () => {
                    editUserPopup = false;
                    api.updateUser(editUser!, new UserMinimalUpdates({ email: editUserEmail, password: editUserPassword, firstName: editUserFirstName, lastName: editUserLastName, roles: null, disabled: null }))
                        .then(newUser => {
                            users[users.map(user => user._id).indexOf(editUser!._id)] = newUser;
                        })
                        .catch(e => console.error(e));
                } : null}
            >Save</p>
        </div>
    </Popup>
{/if}

{#if deleteUserPopup}
    <Popup title="Delete User" onClose={() => deleteUserPopup = false}>
        <div class="flex flex-col items-center justify-center max-w-[350px]" style="margin-top: 20px; margin-bottom: 20px;">
            <p class="text-[14px] text-center opacity-50">Are you sure you want to delete the user "{deleteUser?.firstName} {deleteUser?.lastName}"?</p>
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <p
                class="text-red-600 cursor-pointer rounded-md text-[18px] button red-button"
                style="margin-top: 25px;"
                on:click={() => {
                    deleteUserPopup = false;
                    api.deleteUser(deleteUser!)
                        .then(() => users = users.filter(user => user._id != deleteUser!._id))
                        .catch(e => console.error(e));
                }}
            >Confirm</p>
        </div>
    </Popup>
{/if}

{#if users.length < 1}
    <div class="flex flex-col items-center justify-center gap-[25px] h-full w-full">
        <PackageOpen size="75" class="opacity-40" />
            <p class="text-[20px] opacity-50">There are currently no users set up. (? LOL)</p>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <p
                class="border-blue-500 text-blue-500 hover:bg-blue-500 hover:text-white transition-all border-[1.5px] cursor-pointer rounded-md button"
                style="padding: 10px; margin-top: 25px;"
                on:click={openCreateUserPopup}
            >Create User</p>
    </div>
{:else}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="absolute flex flex-col min-h-[70px] items-center justify-center self-end" style="margin-right: 50px;">
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <p
            class="border-blue-500 text-blue-500 hover:bg-blue-500 hover:text-white transition-all border-[1.5px] cursor-pointer rounded-md button"
            style="padding: 10px;"
            on:click={openCreateUserPopup}
        >Create User</p>
    </div>
    <div class="flex flex-wrap overflow-y-scroll gap-[25px]">
        {#each users.filter(u => u._id != UserMinimal.DEFAULT_USER_ID) as user}
            <div class="flex flex-col items-start justify start gap-[10px] min-w-[300px] border-[2px] border-[#333] rounded-md" style="padding: 15px;">
                <div class="flex flex-row justify-between w-full">
                    <p class="text-[20px] font-bold h-[20px]">{user.firstName} {user.lastName}</p>
                    <div class="flex flex-row">
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div class="flex self-end" style="margin-right: 12.5px;" on:click={() => {
                            editUser = user;
                            editUserFirstName = user.firstName;
                            editUserLastName = user.lastName;
                            editUserPopup = true;
                        }}>
                            <Pen
                                class="cursor-pointer hover:text-blue-500 transition-all"
                                size=20
                            />
                        </div>
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div class="flex self-end" on:click={() => {
                            deleteUser = user;
                            deleteUserPopup = true;
                        }}>
                            <Trash
                                class="cursor-pointer hover:text-red-600 transition-all"
                                size=20
                            />
                        </div>
                    </div>
                </div>
                <p class="text-[12px] opacity-35 h-[20px]">Created at {UserMinimal.getCreatedAt(user).getDate()}.{UserMinimal.getCreatedAt(user).getMonth()}.{UserMinimal.getCreatedAt(user).getFullYear()}</p>
                <TextField label="Email" value={user.email} readonly />
                <RoleList label="Roles" roles={roles.filter(r => user.roles.includes(r._id))} onAdd={() => {}} onRemove={() => {}} readOnly={false} disableOutline />
            </div>
        {/each}
    </div>
{/if}

<style>
    input:focus {
        outline: none;
        border: solid 1.5px var(--color-blue-500);
    }

    ::-webkit-scrollbar {
        width: 5px;
    }

    ::-webkit-scrollbar-thumb {
        background-color: var(--color-blue-500);
        border-radius: 10px;
    }
</style>