import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import preprocess from 'svelte-preprocess';

const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		alias: {
			$lib: 'src/lib'
		},
		adapter: adapter({
			// Prerendering turned off. Turn it on if you know what you're doing.
			prerender: { entries: [] },
			fallback: 'index.html', // enable SPA mode
			pages: '../server/dist'
		})
	}
};

// /** @type {import('@sveltejs/kit').Config} */
// const config = {
// 	preprocess: vitePreprocess(),

// 	kit: {
// 		adapter: adapter({
// 			fallback: undefined,
// 		}),
// 		outDir: "../server/dist",

// 	},
// };

export default config;
