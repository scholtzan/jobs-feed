/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {}
	},
	daisyui: {
		themes: ['light', 'dark', 'cupcake', 'cmyk'],
		styled: true,
		base: true
	},
	plugins: [require('daisyui')]
};
