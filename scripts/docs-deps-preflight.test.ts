import { afterAll, describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  DOCS_DEPS_HINT,
  docsDepsInstalled,
  runDocsDepsPreflight,
} from "./docs-deps-preflight.ts";

const plantRoots: string[] = [];

afterAll(() => {
  for (const root of plantRoots) {
    rmSync(root, { recursive: true, force: true });
  }
});

function initPlant(): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-docs-deps-"));
  plantRoots.push(root);
  writeFileSync(join(root, "bun.lock"), "{}\n");
  return root;
}

// In-process: each case used to spawn `bun <script>`, which cost ~11s on a
// host whose OS temp root had grown to ~237k entries (bun walks the
// entry-heavy ancestor) and broke both the 5s default and the explicit 5s
// bound below. Measured 2026-09-29 at load ~50: each case now runs in <1ms.
function runPreflight(cwd: string): { status: number; stderr: string } {
  return runDocsDepsPreflight(cwd);
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
