import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

const config = {
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

export default config;
