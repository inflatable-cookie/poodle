#!/usr/bin/env bun
/**
 * `jetstream:build` preflight: cargo used to stop on a missing
 * `crates/jetstream-input/Cargo.toml` with a path that did not say the sibling
 * checkout was absent or stale. Fail at once with that message. Poodle does
 * not vendor Jetstream.
 */

import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

export const PREVIEW_MANIFEST = "packages/jetstream/preview/Cargo.toml";
const ENGINE_CRATE_PATH = /path\s*=\s*"\.\.\/\.\.\/\.\.\/\.\.\/jetstream\/crates\/([^"]+)"/g;

export function checkoutRoot(cwd: string = process.cwd()): string {
  const result = spawnSync("git", ["rev-parse", "--show-toplevel"], {
    cwd,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(
      `jetstream:build: git rev-parse --show-toplevel failed\n${result.stderr.trim()}`,
    );
  }
  const toplevel = result.stdout.trim();
  if (toplevel === "") {
    throw new Error("jetstream:build: git rev-parse --show-toplevel returned an empty path");
  }
  return toplevel;
}

export function siblingJetstreamRoot(repoRoot: string): string {
  return resolve(repoRoot, "..", "jetstream");
}

export function requiredEngineCrates(manifestSource: string): string[] {
  const crates = new Set<string>();
  for (const match of manifestSource.matchAll(ENGINE_CRATE_PATH)) {
    crates.add(match[1]);
  }
  return [...crates].sort();
}

export function missingSiblingMessage(engineRoot: string): string {
  return [
    `sibling Jetstream checkout not found at ${engineRoot}.`,
    "jetstream:build needs a paired checkout next to this repository",
    "(../jetstream from the Poodle root, or the Paseo worktree sibling link).",
    "Poodle does not vendor Jetstream.",
  ].join(" ");
}

export function staleSiblingMessage(engineRoot: string, crateName: string): string {
  return [
    `sibling Jetstream checkout at ${engineRoot} is out of date:`,
    `missing crates/${crateName}/Cargo.toml.`,
    "Update that checkout; Poodle does not vendor Jetstream.",
  ].join(" ");
}

export function jetstreamBuildPreflight(
  repoRoot: string,
  manifestSource: string = readFileSync(join(repoRoot, PREVIEW_MANIFEST), "utf8"),
): { status: number; stderr: string } {
  const engineRoot = siblingJetstreamRoot(repoRoot);
  if (!existsSync(join(engineRoot, "Cargo.toml"))) {
    return { status: 1, stderr: `${missingSiblingMessage(engineRoot)}\n` };
  }
  for (const crateName of requiredEngineCrates(manifestSource)) {
    if (!existsSync(join(engineRoot, "crates", crateName, "Cargo.toml"))) {
      return { status: 1, stderr: `${staleSiblingMessage(engineRoot, crateName)}\n` };
    }
  }
  return { status: 0, stderr: "" };
}

if (import.meta.main) {
  try {
    const root = checkoutRoot();
    const result = jetstreamBuildPreflight(root);
    if (result.status !== 0) {
      process.stderr.write(result.stderr);
      process.exit(result.status);
    }
    const cargo = spawnSync(
      "cargo",
      ["build", "-p", "poodle-jetstream-preview", "--manifest-path", join(root, PREVIEW_MANIFEST)],
      { cwd: root, stdio: "inherit" },
    );
    if (cargo.status !== 0) {
      process.exit(cargo.status ?? 1);
    }
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
