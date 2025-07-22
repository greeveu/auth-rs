<script lang="ts">
	import { onMount } from 'svelte';
	import { version, latestVersion, updateAvailable, updateCheckLoading, updateCheckError, checkForUpdates } from '$lib/store/version';
	import { ExternalLink, Download } from 'lucide-svelte';

	// Check for updates when component mounts, but don't retry on errors
	onMount(() => {
		checkForUpdates();
	});
</script>

<div class="flex flex-col items-center gap-1">
	<div class="text-[11px] text-gray-400 opacity-70">
		{$version}
	</div>
	
	{#if $updateCheckLoading}
		<div class="text-[9px] text-gray-500 opacity-60">
			Checking for updates...
		</div>
	{:else if $updateAvailable && $latestVersion}
		<a 
			href="https://github.com/TimLohrer/auth-rs/tags"
			target="_blank"
			rel="noopener noreferrer"
			class="flex items-center gap-1 text-[9px] text-blue-400 hover:text-blue-300 transition-colors cursor-pointer"
			title="New version available: {$latestVersion}"
		>
			<Download size={8} />
			Update to {$latestVersion}
			<ExternalLink size={6} />
		</a>
	{:else if $updateCheckError}
		<div class="text-[9px] text-red-400 opacity-60" title="Error: {$updateCheckError}">
			Update check failed
		</div>
	{/if}
</div>
