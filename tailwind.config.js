/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"SF-Pro"', '"-apple-system"', '"system-ui"', "sans-serif"],
      },
    },
  },
  daisyui: {
    themes: ["nord", "night"],
  },
  plugins: [require("daisyui")],
};
