#!/usr/bin/env bun
/**
 * Workspace slice of `bun.lock`: versions and intra-repo ranges.
 *
 * Bun 1.4.2 (`packageManager`) leaves `workspaces` stale after a version bump:
 * `bun install`, `--force`, and `--lockfile-only` still pass `--frozen-lockfile`
 * with the old versions. This check fails on that drift. `--refresh` patches
 * only the `workspaces` object; the `packages` resolutions stay byte-identical.
 */

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export const LOCK_PATH = "bun.lock";

type DepMap = Record<string, string>;

export type WorkspaceManifest = {
  path: string;
  name: string;
  version?: string;
  dependencies?: DepMap;
  devDependencies?: DepMap;
  peerDependencies?: DepMap;
};

export type WorkspaceLockEntry = {
  name?: string;
  version?: string;
  dependencies?: DepMap;
  devDependencies?: DepMap;
  peerDependencies?: DepMap;
};

export type BunLockFile = {
  workspaces?: Record<string, WorkspaceLockEntry>;
};

const DEP_FIELDS = ["dependencies", "devDependencies", "peerDependencies"] as const;

export function parseJsonc(text: string): unknown {
  try {
    return JSON.parse(text);
  } catch {
    return JSON.parse(text.replace(/,(\s*[}\]])/g, "$1"));
  }
}

export function workspaceManifests(root: string): WorkspaceManifest[] {
  const pkg = JSON.parse(readFileSync(join(root, "package.json"), "utf8")) as {
    workspaces?: string[];
  };
  const paths = pkg.workspaces ?? [];
  return paths.map((workspacePath) => {
    const manifest = JSON.parse(readFileSync(join(root, workspacePath, "package.json"), "utf8")) as {
      name: string;
      version?: string;
      dependencies?: DepMap;
      devDependencies?: DepMap;
      peerDependencies?: DepMap;
    };
    return {
      path: workspacePath,
      name: manifest.name,
      version: manifest.version,
      dependencies: manifest.dependencies,
      devDependencies: manifest.devDependencies,
      peerDependencies: manifest.peerDependencies,
    };
  });
}

export function lockWorkspaces(lockText: string): Record<string, WorkspaceLockEntry> {
  const parsed = parseJsonc(lockText) as BunLockFile;
  return parsed.workspaces ?? {};
}

export function workspaceLockDrift(root: string, lockText?: string): string[] {
  const text = lockText ?? readFileSync(join(root, LOCK_PATH), "utf8");
  const locked = lockWorkspaces(text);
  const findings: string[] = [];

  for (const manifest of workspaceManifests(root)) {
    const entry = locked[manifest.path];
    if (entry === undefined) {
      findings.push(`${LOCK_PATH} workspaces is missing ${manifest.path}`);
      continue;
    }
    if (entry.name !== manifest.name) {
      findings.push(
        `${LOCK_PATH} workspaces[${manifest.path}].name is ${JSON.stringify(entry.name)}, package.json has ${JSON.stringify(manifest.name)}`,
      );
    }
    if (manifest.version !== undefined && entry.version !== manifest.version) {
      findings.push(
        `${LOCK_PATH} workspaces[${manifest.path}].version is ${JSON.stringify(entry.version)}, package.json has ${JSON.stringify(manifest.version)}`,
      );
    }
    for (const field of DEP_FIELDS) {
      const wanted = manifest[field] ?? {};
      const got = entry[field] ?? {};
      for (const [name, specifier] of Object.entries(wanted)) {
        if (got[name] !== specifier) {
          findings.push(
            `${LOCK_PATH} workspaces[${manifest.path}].${field}[${name}] is ${JSON.stringify(got[name])}, package.json has ${JSON.stringify(specifier)}`,
          );
        }
      }
    }
  }

  return findings;
}

function matchBrace(text: string, open: number): number {
  let depth = 0;
  let inString = false;
  let escape = false;
  for (let i = open; i < text.length; i++) {
    const ch = text[i];
    if (inString) {
      if (escape) escape = false;
      else if (ch === "\\") escape = true;
      else if (ch === '"') inString = false;
      continue;
    }
    if (ch === '"') inString = true;
    else if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) return i;
    }
  }
  throw new Error("unbalanced braces in bun.lock");
}

function workspaceBlockSpan(
  lockText: string,
  workspacePath: string,
): { start: number; end: number } {
  const workspacesAt = lockText.indexOf('"workspaces"');
  if (workspacesAt < 0) {
    throw new Error(`${LOCK_PATH} has no workspaces object`);
  }
  const sectionOpen = lockText.indexOf("{", workspacesAt);
  const sectionClose = matchBrace(lockText, sectionOpen);
  const section = lockText.slice(sectionOpen, sectionClose + 1);
  const key = `"${workspacePath}":`;
  const keyAt = section.indexOf(key);
  if (keyAt < 0) {
    throw new Error(`${LOCK_PATH} workspaces is missing ${workspacePath}`);
  }
  const blockOpen = section.indexOf("{", keyAt);
  const blockClose = matchBrace(section, blockOpen);
  return {
    start: sectionOpen + blockOpen,
    end: sectionOpen + blockClose,
  };
}

function replaceQuoted(
  block: string,
  key: string,
  from: string,
  to: string,
): string {
  if (from === to) return block;
  const needle = `"${key}": "${from}"`;
  const next = `"${key}": "${to}"`;
  if (!block.includes(needle)) {
    throw new Error(`cannot find ${needle} in a workspace block`);
  }
  return block.replace(needle, next);
}

/**
 * Update only `workspaces` versions and specifiers. Returns the new lock
 * text; callers write it. The `packages` section is copied verbatim.
 */
export function refreshWorkspaceLockText(root: string, lockText: string): string {
  const locked = lockWorkspaces(lockText);
  let next = lockText;

  for (const manifest of workspaceManifests(root)) {
    const entry = locked[manifest.path];
    if (entry === undefined) {
      throw new Error(
        `${LOCK_PATH} workspaces is missing ${manifest.path}; run bun install once to add it`,
      );
    }
    const span = workspaceBlockSpan(next, manifest.path);
    let block = next.slice(span.start, span.end + 1);

    if (manifest.version !== undefined && entry.version !== undefined && entry.version !== manifest.version) {
      block = replaceQuoted(block, "version", entry.version, manifest.version);
    }
    if (entry.name !== undefined && entry.name !== manifest.name) {
      block = replaceQuoted(block, "name", entry.name, manifest.name);
    }
    for (const field of DEP_FIELDS) {
      const wanted = manifest[field] ?? {};
      const got = entry[field] ?? {};
      for (const [name, specifier] of Object.entries(wanted)) {
        if (got[name] !== undefined && got[name] !== specifier) {
          block = replaceQuoted(block, name, got[name], specifier);
        }
      }
    }

    next = next.slice(0, span.start) + block + next.slice(span.end + 1);
  }

  return next;
}

export function refreshWorkspaceLock(root: string): { changed: boolean } {
  const path = join(root, LOCK_PATH);
  const before = readFileSync(path, "utf8");
  const after = refreshWorkspaceLockText(root, before);
  if (after === before) return { changed: false };
  writeFileSync(path, after);
  return { changed: true };
}

function packagesSection(lockText: string): string {
  const at = lockText.indexOf('\n  "packages"');
  if (at < 0) return lockText.slice(lockText.indexOf('"packages"'));
  return lockText.slice(at);
}

export function packagesSectionUnchanged(before: string, after: string): boolean {
  return packagesSection(before) === packagesSection(after);
}

if (import.meta.main) {
  const mode = process.argv.includes("--refresh") ? "refresh" : "check";
  const root = process.cwd();
  try {
    if (mode === "refresh") {
      const changed = refreshWorkspaceLock(root);
      if (changed.changed) {
        console.log(`${LOCK_PATH} workspaces updated from package.json`);
      } else {
        console.log(`${LOCK_PATH} workspaces already match package.json`);
      }
    }
    const findings = workspaceLockDrift(root);
    if (findings.length > 0) {
      console.error(`${LOCK_PATH} workspaces are stale:`);
      for (const finding of findings) console.error(`  ${finding}`);
      console.error(`Refresh with: bun scripts/workspace-lock.ts --refresh`);
      process.exit(1);
    }
    if (mode === "check") {
      console.log(`${LOCK_PATH} workspaces match package.json`);
    }
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
