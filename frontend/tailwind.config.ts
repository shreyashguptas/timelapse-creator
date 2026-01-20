import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        cream: {
          DEFAULT: "#F5F3ED",
          light: "#FAF9F7",
          dark: "#EBE8E0",
        },
        charcoal: {
          DEFAULT: "#1A1A1A",
          light: "#4A4A4A",
          muted: "#8A8A8A",
        },
        success: "#2D5016",
        error: "#8B2942",
      },
      fontFamily: {
        sans: ["var(--font-sans)", "system-ui", "sans-serif"],
        serif: ["var(--font-serif)", "Georgia", "serif"],
      },
    },
  },
  plugins: [],
};
export default config;
