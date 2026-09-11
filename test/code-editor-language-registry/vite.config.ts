import { fileURLToPath } from "node:url";
import { svelte } from "@sveltejs/vite-plugin-svelte";

const core = fileURLToPath(new URL("../../packages/core/src/index.ts", import.meta.url));
const coreStyles = fileURLToPath(new URL("../../packages/core/src/styles", import.meta.url));
// The fixture consumer — like the specimen previews — owns its grammar
// packages. Poodle itself declares none, so the fixture resolves the two
// grammars its registry names from the preview app that installs them.
const previewModules = fileURLToPath(new URL("../../packages/svelte/preview/node_modules", import.meta.url));

export default {
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [svelte()],
  esbuild: { jsx: "automatic" },
  resolve: {
    alias: [
      { find: "@inflatable-cookie/poodle-core/styles", replacement: coreStyles },
      { find: /^@inflatable-cookie\/poodle-core$/, replacement: core },
      { find: "@codemirror/lang-json", replacement: `${previewModules}/@codemirror/lang-json` },
      {
        find: "@codemirror/lang-javascript",
        replacement: `${previewModules}/@codemirror/lang-javascript`,
      },
    ],
  },
  server: {
    host: "127.0.0.1",
    strictPort: true,
  },
};
