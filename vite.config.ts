import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
	server: {
		allowedHosts: true
	},
	define: {
		__VERSION__: JSON.stringify(process.env.VITE_VERSION || 'dev')
	}
});
