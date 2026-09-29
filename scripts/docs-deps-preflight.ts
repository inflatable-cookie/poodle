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

/**
 * Decide the preflight for `root` and return the CLI's exit status and stderr.
 * Exported so the planted test can exercise the real logic in-process: one
 * `bun <script>` spawn per fixture cost ~11s on a host whose temp root had
 * grown to ~237k entries (bun walks the entry-heavy ancestor), which tripped
 * the test's explicit 5s bound and bun's 5s default.
 */
export function runDocsDepsPreflight(root: string): { status: number; stderr: string } {
  if (!docsDepsInstalled(root)) {
    return { status: 1, stderr: `${missingDocsDepsMessage()}\n` };
  }
  return { status: 0, stderr: "" };
}

if (import.meta.main) {
  try {
    const root = checkoutRoot();
    const result = runDocsDepsPreflight(root);
    if (result.stderr !== "") process.stderr.write(result.stderr);
    process.exit(result.status);
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
