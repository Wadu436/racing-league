import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { nodePolyfills } from 'vite-plugin-node-polyfills';

export default defineConfig({
	plugins: [sveltekit(), nodePolyfills()],
	server: {
		host: '127.0.0.1',
		port: 5173
	}
});
