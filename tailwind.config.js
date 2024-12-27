/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            fontFamily: {
                sans: ['"Work Sans"','"-apple-system"', '"system-ui"', 'sans-serif'],
            },
        },
    },
    daisyui: {
        themes: [
            {
                dark: {
                    "primary": "#c4a7e7",        // love
                    "secondary": "#31748f",      // pine
                    "accent": "#907aa9",         // gold
                    "neutral": "#191724",        // base
                    "base-100": "#1f1d2e",       // surface
                    "base-200": "#26233a",       // overlay
                    "base-300": "#6e6a86",       // muted
                    "base-content": "#e0def4",   // text
                    "foam": "#9ccfd8",           // foam
                    "pine": "#31748f",           // pine
                    "gold": "#f6c177",           // gold
                    "love": "#eb6f92",           // love
                },
                light: {
                    "primary": "#907aa9",        // love
                    "secondary": "#286983",      // pine
                    "accent": "#907aa9",         // gold
                    "neutral": "#faf4ed",        // base
                    "base-100": "#fffaf3",       // surface
                    "base-200": "#f2e9e1",       // overlay
                    "base-300": "#9893a5",       // muted
                    "base-content": "#575279",   // text
                    "foam": "#56949f",           // foam
                    "pine": "#286983",           // pine
                    "gold": "#ea9d34",           // gold
                    "love": "#b4637a",           // love
                }
            }
        ]
    },
    plugins: [require("daisyui")],
}