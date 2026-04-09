import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig, type PluginOption } from "vite";

// Cast needed: bun hoists vite types to a different path than the local copy,
// making TypeScript treat identical Plugin types as incompatible.
export default defineConfig({
  plugins: [svelte() as unknown as PluginOption],
});
