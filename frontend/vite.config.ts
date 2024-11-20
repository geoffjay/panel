import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import mdx from "@mdx-js/rollup";
import { fileURLToPath, URL } from "node:url";

// https://vitejs.dev/config/
export default defineConfig({
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
});
