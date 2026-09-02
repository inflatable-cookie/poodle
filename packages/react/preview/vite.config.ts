import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react()],
  // The catalogue intentionally loads every component and default icon so
  // operators can inspect the full surface from one offline bundle.
  build: { chunkSizeWarningLimit: 2200 },
  resolve: {
    alias: {
      "@inflatable-cookie/poodle-react/markdown": fileURLToPath(
        new URL("../components/src/markdown.ts", import.meta.url),
      ),
      "@inflatable-cookie/poodle-react/types": fileURLToPath(
        new URL("../components/src/types.ts", import.meta.url),
      ),
      "@inflatable-cookie/poodle-react": fileURLToPath(
        new URL("../components/src/index.ts", import.meta.url),
      ),
    },
  },
  server: { port: 4180 },
});
