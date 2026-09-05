import { svelte } from "@sveltejs/vite-plugin-svelte";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { configDefaults, defineConfig } from "vitest/config";

const repoRoot = dirname(fileURLToPath(import.meta.url));
// g16.111: the Svelte accessibility extractor reads names and roles through
// dom-accessibility-api, which this repository already carries as a declared
// dependency of @testing-library/dom. Resolve it through that graph rather
// than adding a root manifest entry, which the package-install certification
// gate forbids on a worker branch.
const domAccessibilityApi = createRequire(
  createRequire(import.meta.url).resolve("@testing-library/dom/package.json"),
).resolve("dom-accessibility-api");
const workspaceAliases = {
  "@inflatable-cookie/poodle-svelte/markdown": join(
    repoRoot,
    "packages/svelte/components/src/markdown.ts",
  ),
  "@inflatable-cookie/poodle-svelte/types": join(
    repoRoot,
    "packages/svelte/components/src/types.ts",
  ),
  "@inflatable-cookie/poodle-svelte": join(
    repoRoot,
    "packages/svelte/components/src/index.ts",
  ),
  "@inflatable-cookie/poodle-react/markdown": join(
    repoRoot,
    "packages/react/components/src/markdown.ts",
  ),
  "@inflatable-cookie/poodle-react/types": join(
    repoRoot,
    "packages/react/components/src/types.ts",
  ),
  "@inflatable-cookie/poodle-react": join(
    repoRoot,
    "packages/react/components/src/index.ts",
  ),
};

// Component-shell smoke tests for the Svelte and React implementations.
// Verifies the machine -> DOM wiring per framework: components mount, contract
// anatomy/classes are present, and stateful controls react to interaction.
// Pure state-machine logic lives in @inflatable-cookie/poodle-core (its own suite); these
// tests cover the framework binding those machines to real DOM.
export default defineConfig({
  resolve: { alias: workspaceAliases },
  test: {
    projects: [
      {
        plugins: [svelte()],
        resolve: { alias: workspaceAliases, conditions: ["browser"] },
        test: {
          name: "svelte-components",
          environment: "happy-dom",
          globals: true,
          include: ["packages/svelte/components/test/**/*.test.ts"],
          // The SSR suite needs server-compiled components; running it under
          // this client-compiled project crashes `svelte/server`'s render.
          exclude: [...configDefaults.exclude, "packages/svelte/components/test/ssr/**"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        // Server-render evidence for the Svelte adapter (g15.041): no browser
        // resolve condition, so vite-plugin-svelte compiles .svelte for the
        // server and `render` from `svelte/server` works.
        plugins: [svelte()],
        test: {
          name: "svelte-components-ssr",
          environment: "node",
          include: ["packages/svelte/components/test/ssr/**/*.test.ts"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        plugins: [svelte()],
        resolve: { alias: workspaceAliases, conditions: ["browser"] },
        test: {
          name: "svelte-preview",
          environment: "happy-dom",
          globals: true,
          include: ["packages/svelte/preview/test/**/*.test.ts"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        // Headless DOM geometry: the parts of @inflatable-cookie/poodle-core that read layout
        // and computed style. The rest of the core suite is pure and runs under
        // `bun test`, which has no DOM at all.
        test: {
          name: "headless-dom",
          environment: "happy-dom",
          globals: true,
          include: ["test/headless-dom/**/*.test.ts"],
        },
      },
      {
        resolve: { alias: workspaceAliases },
        test: {
          name: "react-components",
          environment: "happy-dom",
          globals: true,
          include: ["packages/react/components/test/**/*.test.tsx"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        resolve: { alias: workspaceAliases },
        test: {
          name: "react-preview",
          environment: "happy-dom",
          globals: true,
          include: ["packages/react/preview/test/**/*.test.tsx"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        // Runtime accessibility sweep (axe-core) over the Svelte components.
        plugins: [svelte()],
        resolve: { alias: workspaceAliases, conditions: ["browser"] },
        test: {
          name: "a11y",
          environment: "happy-dom",
          globals: true,
          include: ["test/a11y/**/*.test.ts"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        // g16.111 Nucleus A1: the Svelte half of the paired accessibility
        // receipt. Mounts each shared scenario, replays its actions through
        // DOM events, and emits the accessibility snapshot the GPUI headless
        // run is compared against.
        plugins: [svelte()],
        resolve: {
          alias: { ...workspaceAliases, "dom-accessibility-api": domAccessibilityApi },
          conditions: ["browser"],
        },
        test: {
          name: "nucleus-a11y",
          environment: "happy-dom",
          globals: true,
          include: ["test/nucleus-a11y/**/*.test.ts"],
          setupFiles: ["./test/vitest.setup.ts"],
        },
      },
      {
        // Svelte <-> React parity: renders both implementations of a component in
        // one happy-dom process and diffs their emitted poodle-* anatomy classes.
        // Needs the Svelte plugin and Vitest's JSX transform in the same project.
        plugins: [svelte()],
        resolve: { alias: workspaceAliases, conditions: ["browser"] },
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
