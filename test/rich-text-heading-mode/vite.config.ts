import { fileURLToPath } from "node:url";
import { svelte } from "@sveltejs/vite-plugin-svelte";

const core = fileURLToPath(new URL("../../packages/core/src/index.ts", import.meta.url));
const coreStyles = fileURLToPath(new URL("../../packages/core/src/styles", import.meta.url));

export default {
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [svelte()],
  esbuild: { jsx: "automatic" },
  resolve: {
    alias: [
      { find: "@inflatable-cookie/poodle-core/styles", replacement: coreStyles },
      { find: /^@inflatable-cookie\/poodle-core$/, replacement: core },
    ],
  },
  server: {
    host: "127.0.0.1",
    strictPort: true,
  },
};
