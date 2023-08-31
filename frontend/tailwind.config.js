import colors from 'tailwindcss/colors';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {},

		colors: {
			transparent: 'transparent',
			current: 'currentColor',
			primary: colors.violet,
			secondary: colors.blue,

			black: colors.black,
			white: colors.white,
			gray: colors.slate,

			green: colors.emerald,
			red: colors.rose,
			purple: colors.violet,
			yellow: colors.amber,
			pink: colors.fuchsia
		}
	},
	plugins: []
};
