#!/usr/bin/env bun
/**
 * Fast `docs:check` preflight: a worktree with no `node_modules` used to fail
 * deep in the board on a missing `@inflatable-cookie/poodle-core/tokens`
 * import. Fail in seconds with the one-line install hint instead.
 */

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

export const DOCS_DEPS_HINT =
  "Cannot resolve @inflatable-cookie/poodle-core/tokens (no node_modules). Run bun install.";

const CORE_PACKAGE = join("node_modules", "@inflatable-cookie", "poodle-core");
const PREVIEW_CORE_PACKAGE = join(
  "packages",
  "svelte",
  "preview",
  "node_modules",
  "@inflatable-cookie",
  "poodle-core",
);

export function docsDepsInstalled(root: string): boolean {
  if (!existsSync(join(root, "node_modules"))) return false;
  return existsSync(join(root, CORE_PACKAGE)) || existsSync(join(root, PREVIEW_CORE_PACKAGE));
}

export function checkoutRoot(cwd: string = process.cwd()): string {
  const result = spawnSync("git", ["rev-parse", "--show-toplevel"], {
    cwd,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `docs:deps-preflight: git rev-parse --show-toplevel failed\n${result.stderr.trim()}`,
    );
  }
  const toplevel = result.stdout.trim();
  if (toplevel === "") {
    throw new Error("docs:deps-preflight: git rev-parse --show-toplevel returned an empty path");
  }
  return toplevel;
}

export function missingDocsDepsMessage(): string {
  return DOCS_DEPS_HINT;
}

if (import.meta.main) {
  try {
    const root = checkoutRoot();
    if (!docsDepsInstalled(root)) {
      console.error(missingDocsDepsMessage());
      process.exit(1);
    }
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
