<script lang="ts">
	import TextField from "$lib/components/dashboard/TextField.svelte";
import { Pencil } from "lucide-svelte";

    export let user: UserMinimal;
    export let roles: Role[];
</script>

<div class="flex flex-col items-start justify-start h-[100%] w-full gap-[10px]" style="padding-top: 0%;">
    <TextField label="Full Name" value={`${user.firstName} ${user.lastName}`} />
    <TextField label="Email" value={user.email} />
    <TextField label="Creation Date" value={`${user.createdAt.split(' ')[0].split('-').reverse().join('.')} ${user.createdAt.split(' ')[1].split('.')[0]}`} readonly />
</div>