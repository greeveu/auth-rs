<script lang="ts">
	import { apiUrl } from '$lib/store/config';
	import '../app.css';
	let { data, children } = $props<{
		data: { apiUrl: string };
		children: any;
	}>();

	apiUrl.set(data.apiUrl);
</script>

{@render children()}
