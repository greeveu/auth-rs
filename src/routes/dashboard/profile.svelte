<script lang="ts">
	import { Pencil } from "lucide-svelte";

    export let user: UserMinimal;
    export let roles: Role[];
</script>

<div class="flex flex-col items-start justify-start h-[90%] w-full gap-[20px]" style="padding-top: 0%;">
    <div>
        <p class="text-[14px]">> Dashboard > Your Profile</p>
    </div>
    <div
        class="name-input flex justify-between items-center outline-none border-[2px] border-[#222] hover:text-blue-500 rounded-md min-w-[275px] cursor-pointer transition-all"
        style="padding: 3.5px 10px;"
    >
        <p style="color: white !important;">{user.firstName} {user.lastName}</p>
        <Pencil size="17.5" />
    </div>
    <div
        class="name-input flex justify-between items-center outline-none border-[2px] border-[#222] hover:text-blue-500 rounded-md min-w-[275px] cursor-pointer transition-all"
        style="padding: 3.5px 10px;"
    >
        <p style="color: white !important;">{user.email}</p>
        <Pencil size="17.5" />
    </div>
    <div
        class="name-input flex justify-between items-center outline-none border-[2px] border-[#222] rounded-md min-w-[275px]"
        style="padding: 3.5px 10px;"
    >
        <p>{`${user.createdAt.split(' ')[0].split('-').reverse().join('.')} ${user.createdAt.split(' ')[1].split('.')[0]}`}</p>
    </div>
</div>