import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          "Inter",
          "ui-sans-serif",
          "system-ui",
          "-apple-system",
          "Segoe UI",
          "Roboto",
          "sans-serif",
        ],
        mono: [
          "JetBrains Mono",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "monospace",
        ],
      },
      colors: {
        bg: {
          base: "#0a0a0f",
          surface: "#14141c",
          elevated: "#1c1c26",
        },
        accent: {
          cyan: "#22d3ee",
          emerald: "#10b981",
          rose: "#f43f5e",
          amber: "#f59e0b",
          violet: "#a78bfa",
        },
      },
      backdropBlur: {
        xs: "2px",
      },
      animation: {
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        shimmer: "shimmer 2s linear infinite",
      },
      keyframes: {
        shimmer: {
          "0%": { backgroundPosition: "-200% 0" },
          "100%": { backgroundPosition: "200% 0" },
        },
      },
      boxShadow: {
        glow: "0 0 16px -2px var(--tw-shadow-color)",
        "glow-lg": "0 0 32px -4px var(--tw-shadow-color)",
      },
    },
  },
  plugins: [],
} satisfies Config;
