import {
  svelte,
  vitePreprocess,
} from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [svelte({ preprocess: vitePreprocess() })],
  esbuild: {
    jsx: "automatic",
  },
  resolve: {
    conditions: ["browser"],
  },
  ssr: {
    noExternal: [/^@inflatable-cookie\/poodle-/],
  },
  test: {
    environment: "happy-dom",
  },
});
