import { svelte } from "@sveltejs/vite-plugin-svelte"
import { defineConfig } from 'vite';

export default defineConfig({
	base: ".",
	plugins: [svelte()],
	build: {
        outDir: "../server/dist",
        emptyOutDir: true,
		rollupOptions: {
			// https://rollupjs.org/configuration-options/
		},
    }
});

