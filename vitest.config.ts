import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vitest/config";

// Component-shell smoke tests for the Svelte and React implementations.
// Verifies the machine -> DOM wiring per framework: components mount, contract
// anatomy/classes are present, and stateful controls react to interaction.
// Pure state-machine logic lives in @poodle/headless (its own suite); these
// tests cover the framework binding those machines to real DOM.
export default defineConfig({
  test: {
    projects: [
      {
        plugins: [svelte()],
        resolve: { conditions: ["browser"] },
        test: {
          name: "svelte-components",
          environment: "happy-dom",
          globals: true,
          include: ["packages/svelte/components/test/**/*.test.ts"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        esbuild: { jsx: "automatic" },
        test: {
          name: "react-components",
          environment: "happy-dom",
          globals: true,
          include: ["packages/react/components/test/**/*.test.tsx"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        // Svelte <-> React parity: renders both implementations of a component in
        // one happy-dom process and diffs their emitted poodle-* anatomy classes.
        // Needs both toolchains (svelte plugin + react jsx) in the same project.
        plugins: [svelte()],
        esbuild: { jsx: "automatic" },
        resolve: { conditions: ["browser"] },
        test: {
          name: "parity",
          environment: "happy-dom",
          globals: true,
          include: ["test/parity/**/*.test.tsx"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
    ],
  },
});
