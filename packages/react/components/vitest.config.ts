import { fileURLToPath } from "node:url";

import { defineConfig } from "vitest/config";

import repositoryConfig from "../../../vitest.config.ts";

// Focused React component tests are documented to run from this package
// directory. Vite stops its config search at this `package.json` boundary, so
// without a local config `bunx vitest` here misses the repository project and
// its `packages/react/components/test/**` include resolves under the package
// instead of the checkout — reporting "No test files found". Re-export the
// repository config with the checkout as root so the same project, aliases and
// happy-dom environment apply wherever the command starts.
const repoRoot = fileURLToPath(new URL("../../..", import.meta.url));

export default defineConfig({
  ...repositoryConfig,
  root: repoRoot,
});
