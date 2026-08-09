import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react()],
  // The catalogue intentionally loads every component and default icon so
  // operators can inspect the full surface from one offline bundle.
  build: { chunkSizeWarningLimit: 2200 },
  server: { port: 4180 },
});
