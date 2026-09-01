import { fileURLToPath } from "node:url";
import { svelte } from "@sveltejs/vite-plugin-svelte";

const core = fileURLToPath(new URL("../../packages/core/src/index.ts", import.meta.url));
const coreStyles = fileURLToPath(new URL("../../packages/core/src/styles", import.meta.url));

export default {
  root: fileURLToPath(new URL(".", import.meta.url)),
  // The component fixture mounts the real Svelte and React composites, so the
  // page proves what a consumer actually renders rather than a hand-built
  // stand-in. React needs no plugin here: the fixture never hot-reloads, so
  // esbuild's automatic JSX runtime is the whole requirement.
  plugins: [svelte()],
  esbuild: { jsx: "automatic" },
  resolve: {
    // Exact-match only: the component packages resolve their own sub-path
    // imports (`/icons`, `/tokens`) through their workspace links, and a
    // prefix alias here would shadow them with a path that has no exports map.
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
