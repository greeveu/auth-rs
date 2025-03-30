<script lang="ts">
	import OnOffToggle from '../../lib/components/global/OnOffToggle.svelte';
	import type Settings from "$lib/models/Settings";
	import type AuthRsApi from '$lib/api';
	import SettingsUpdates from '$lib/models/SettingsUpdates';

    export let api: AuthRsApi;
    export let settings: Settings;

    let disabled = false;

    function updateSettings() {
        disabled = true;
        api.updateSettings(new SettingsUpdates({
            openRegistration: settings.openRegistration,
            allowOauthAppsForUsers: settings.allowOauthAppsForUsers
        })).then((newSettings) => {
            settings = newSettings;
            disabled = false;
        }).catch(() => {
            disabled = false;
        });
    }
</script>

<div class="flex flex-col items-start justify-start h-[100%] w-full gap-[20px]">
    <OnOffToggle label="Allow Registrations" bind:value={settings.openRegistration} onToggle={updateSettings} disabled={disabled} />
    <OnOffToggle label="Allow OAuth Applications for Users" bind:value={settings.allowOauthAppsForUsers} onToggle={updateSettings} disabled={disabled} />
</div>