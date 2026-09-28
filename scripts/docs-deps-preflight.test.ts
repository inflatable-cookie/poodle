import { afterAll, describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  checkoutRoot,
  DOCS_DEPS_HINT,
  docsDepsInstalled,
} from "./docs-deps-preflight.ts";

const repoRoot = checkoutRoot();
const script = join(repoRoot, "scripts/docs-deps-preflight.ts");
const plantRoots: string[] = [];

afterAll(() => {
  for (const root of plantRoots) {
    rmSync(root, { recursive: true, force: true });
  }
});

function initPlant(): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-docs-deps-"));
  plantRoots.push(root);
  const git = spawnSync("git", ["init", "--quiet"], { cwd: root, encoding: "utf8" });
  if (git.status !== 0) {
    throw new Error(`git init failed: ${git.stderr}`);
  }
  writeFileSync(join(root, "bun.lock"), "{}\n");
  return root;
}

function runPreflight(cwd: string): ReturnType<typeof spawnSync> {
  return spawnSync("bun", [script], { cwd, encoding: "utf8" });
}

describe("docs:check missing-deps preflight", () => {
  test("a checkout with the core package linked passes", () => {
    const root = initPlant();
    mkdirSync(join(root, "node_modules", "@inflatable-cookie", "poodle-core"), {
      recursive: true,
    });
    writeFileSync(
      join(root, "node_modules", "@inflatable-cookie", "poodle-core", "package.json"),
      '{"name":"@inflatable-cookie/poodle-core"}\n',
    );
    expect(docsDepsInstalled(root)).toBe(true);
    const result = runPreflight(root);
    expect(result.status).toBe(0);
    expect(result.stderr).not.toContain(DOCS_DEPS_HINT);
  });

  test("a worktree without node_modules fails in seconds with the install hint", () => {
    const root = initPlant();
    expect(docsDepsInstalled(root)).toBe(false);

    const started = Date.now();
    const result = runPreflight(root);
    const elapsed = Date.now() - started;

    expect(elapsed).toBeLessThan(5_000);
    expect(result.status).toBe(1);
    expect(result.stderr).toContain(DOCS_DEPS_HINT);
    expect(result.stderr).toContain("bun install");
  });
});
