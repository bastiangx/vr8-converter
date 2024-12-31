/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Work Sans"', '"-apple-system"', '"system-ui"', "sans-serif"],
      },
    },
  },
  daisyui: {
    themes: [
      {
        night: {
          ...require("daisyui/src/theming/themes")["night"],
          "base-100": "#11111b",
        },
      },
      "nord",
    ],
  },
  plugins: [require("daisyui")],
};
