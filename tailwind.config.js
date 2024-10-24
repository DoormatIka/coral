import daisyui from "daisyui"

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
			fontFamily: {
        kanit: ['"Kanit"', "serif"],
      },
		},
  },
  plugins: [daisyui],
	daisyui: {
		themes: ["luxury"],
	},
}

