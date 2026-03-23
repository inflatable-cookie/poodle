import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      "@poodle/svelte-primitives": fileURLToPath(new URL("../primitives/src/index.ts", import.meta.url)),
      "@poodle/svelte-composites": fileURLToPath(new URL("../composites/src/index.ts", import.meta.url)),
    },
  },
  server: {
    host: "0.0.0.0",
    port: 4173,
  },
});
