import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [svelte()],
  build: {
    chunkSizeWarningLimit: 2200,
  },
  resolve: {
    alias: {
      "@inflatable-cookie/poodle-svelte/markdown": fileURLToPath(
        new URL("../components/src/markdown.ts", import.meta.url),
      ),
      "@inflatable-cookie/poodle-svelte/types": fileURLToPath(
        new URL("../components/src/types.ts", import.meta.url),
      ),
      "@inflatable-cookie/poodle-svelte": fileURLToPath(new URL("../components/src/index.ts", import.meta.url)),
    },
  },
  server: {
    host: "0.0.0.0",
    port: 4173,
  },
});
