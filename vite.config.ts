import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import mdx from "@mdx-js/rollup";
import { fileURLToPath, URL } from "node:url";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  resolve: {
    alias: {
      "~root": fileURLToPath(new URL("./src", import.meta.url)),
      "~components": fileURLToPath(new URL("./src/components", import.meta.url)),
      "~features": fileURLToPath(new URL("./src/features", import.meta.url)),
      "~wails": fileURLToPath(new URL("./wailsjs", import.meta.url)),
    },
  },
  plugins: [
    { enforce: "pre", ...mdx({/* jsxImportSource: …, otherOptions… */ }) },
    react({ include: /\.(jsx|js|mdx|md|tsx|ts)$/ }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
