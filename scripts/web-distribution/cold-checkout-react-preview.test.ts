import { afterAll, describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

import { findRepoRoot } from "./core-build";

const repoRoot = findRepoRoot();

const COLD_SUITES = [
  "packages/react/preview/test/catalogue-nav.test.tsx",
  "packages/react/preview/test/g15-031-foundation-content-status.test.tsx",
  "packages/react/preview/test/g15-033-composition-forms-data-media.test.tsx",
] as const;

const REACT_PREVIEW_ALIAS =
  /resolve:\s*\{\s*alias:\s*workspaceAliases\s*\},\s*(?=test:\s*\{\s*name:\s*"react-preview")/;

// Whole-line plain alias assignments: the root config's resolve.alias and the
// projects that restate it without extra conditions.
const WORKSPACE_ALIAS_LINE =
  /^[^\S\n]*resolve:\s*\{\s*alias:\s*workspaceAliases\s*\},[^\S\n]*$/gm;
const WORKSPACE_ALIAS_LINE_PRESENT =
  /^[^\S\n]*resolve:\s*\{\s*alias:\s*workspaceAliases\s*\},[^\S\n]*$/m;

const RESOLVE_FAILURE = 'Failed to resolve import "@inflatable-cookie/poodle-react"';

const VITEST_TIMEOUT_MS = 120_000;

const childEnv = { ...process.env };
delete childEnv.FORCE_COLOR;

let coldRoot: string | undefined;

function run(
  command: string,
  args: string[],
  cwd: string,
  timeout = VITEST_TIMEOUT_MS,
): ReturnType<typeof spawnSync> {
  return spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    env: childEnv,
    timeout,
  });
}

function copyPackageNodeModules(fromRoot: string, toRoot: string): void {
  const found = run("find", [fromRoot, "-name", "node_modules", "-type", "d", "-prune"], fromRoot, 30_000);
  if (found.status !== 0) {
    throw new Error(`find node_modules failed: ${found.stderr}`);
  }
  for (const dir of found.stdout.split("\n").filter(Boolean)) {
    const rel = dir.slice(fromRoot.length + 1);
    if (rel === "node_modules" || rel.startsWith("node_modules/") || rel.includes("/node_modules/")) {
      continue;
    }
    const dest = join(toRoot, rel);
    mkdirSync(dirname(dest), { recursive: true });
    const copied = run("cp", ["-a", dir, dest], fromRoot, 30_000);
    if (copied.status !== 0) {
      throw new Error(`cp ${rel} failed: ${copied.stderr}`);
    }
  }
}

function createColdCheckout(): string {
  const parent = mkdtempSync(join(tmpdir(), "poodle-cold-web-"));
  const root = join(parent, "checkout");
  const added = run("git", ["worktree", "add", "--detach", root, "HEAD"], repoRoot, 60_000);
  if (added.status !== 0) {
    throw new Error(`git worktree add failed: ${added.stderr}${added.stdout}`);
  }
  copyFileSync(join(repoRoot, "vitest.config.ts"), join(root, "vitest.config.ts"));
  symlinkSync(join(repoRoot, "node_modules"), join(root, "node_modules"));
  copyPackageNodeModules(repoRoot, root);
  const coreDist = join(repoRoot, "packages/core/dist");
  if (!existsSync(coreDist)) {
    throw new Error("packages/core/dist missing; run core:build before this proof");
  }
  const copiedCore = run("cp", ["-a", coreDist, join(root, "packages/core/dist")], repoRoot, 30_000);
  if (copiedCore.status !== 0) {
    throw new Error(`cp core dist failed: ${copiedCore.stderr}`);
  }
  return root;
}

function removeColdCheckout(root: string): void {
  run("git", ["worktree", "remove", "--force", root], repoRoot, 60_000);
  run("git", ["worktree", "prune"], repoRoot, 30_000);
  rmSync(dirname(root), { recursive: true, force: true });
}

function stripWorkspaceAliases(source: string): string {
  if (!/name:\s*"react-preview"/.test(source)) {
    throw new Error("vitest.config.ts has no react-preview project");
  }
  // vitest 5 merges the root config's resolve.alias into every project, so
  // stripping only the react-preview project's own alias leaves the import
  // resolving through the inherited root alias and the planted regression
  // passes for the wrong reason. The negative control strips every plain
  // workspaceAliases alias line (root and projects) so the checkout is
  // genuinely alias-less.
  return source.replace(WORKSPACE_ALIAS_LINE, "");
}

function ciWebSequence(toml: string): string[] {
  const match = toml.match(/"ci:web"\s*=\s*\[([\s\S]*?)\]/);
  if (!match) {
    throw new Error("ci:web sequence missing");
  }
  const names: string[] = [];
  for (const line of match[1].split("\n")) {
    const task = line.match(/task\s*=\s*"([^"]+)"/);
    if (task) names.push(task[1]);
  }
  return names;
}

function runColdSuites(cwd: string): { status: number | null; output: string } {
  // Call the checkout's vitest binary. `bunx vitest … -- <files>` lets bunx
  // swallow the `--` and drop the file filters, so the whole react-preview
  // include runs — including suites that import `lucide-static/icon-nodes.json`
  // through the root `node_modules` symlink.
  const vitest = join(cwd, "node_modules", ".bin", "vitest");
  const result = run(vitest, ["run", "--project", "react-preview", ...COLD_SUITES], cwd);
  return {
    status: result.status,
    output: `${result.stdout}\n${result.stderr}`,
  };
}

afterAll(() => {
  if (coldRoot) {
    removeColdCheckout(coldRoot);
    coldRoot = undefined;
  }
}, 60_000);

describe("g16.098 cold-checkout react-preview", () => {
  test("ci:web builds shell packages before test:components and keeps pack-install after them", () => {
    const sequence = ciWebSequence(readFileSync(join(repoRoot, "tasks/effigy.tasks.toml"), "utf8"));
    const svelte = sequence.indexOf("svelte:package");
    const react = sequence.indexOf("react:package");
    const components = sequence.indexOf("test:components");
    const pack = sequence.indexOf("test:web-pack-install");
    expect(svelte).toBeGreaterThan(-1);
    expect(react).toBeGreaterThan(-1);
    expect(components).toBeGreaterThan(-1);
    expect(pack).toBeGreaterThan(-1);
    expect(svelte).toBeLessThan(components);
    expect(react).toBeLessThan(components);
    expect(pack).toBeGreaterThan(Math.max(svelte, react));
  });

  test(
    "the three react-preview suites pass in a detached worktree with no shell dist",
    () => {
      coldRoot = createColdCheckout();
      expect(existsSync(join(coldRoot, "packages/react/components/dist"))).toBe(false);
      expect(existsSync(join(coldRoot, "packages/svelte/components/dist"))).toBe(false);
      expect(readFileSync(join(coldRoot, "vitest.config.ts"), "utf8")).toMatch(REACT_PREVIEW_ALIAS);
      const result = runColdSuites(coldRoot);
      expect(result.output, result.output).not.toContain(RESOLVE_FAILURE);
      expect(result.output, result.output).not.toContain("g18-019-markdown-renderer.test.tsx");
      expect(result.status, result.output).toBe(0);
    },
    VITEST_TIMEOUT_MS,
  );

  test(
    "removing the react-preview alias fails the same three suites with Failed to resolve import",
    () => {
      if (!coldRoot) {
        coldRoot = createColdCheckout();
      }
      const configPath = join(coldRoot, "vitest.config.ts");
      const original = readFileSync(configPath, "utf8");
      const planted = stripWorkspaceAliases(original);
      expect(planted).not.toMatch(REACT_PREVIEW_ALIAS);
      expect(planted).not.toMatch(WORKSPACE_ALIAS_LINE_PRESENT);
      expect(planted).toMatch(/name:\s*"react-preview"/);
      writeFileSync(configPath, planted);
      try {
        const result = runColdSuites(coldRoot);
        expect(result.status).not.toBe(0);
        expect(result.output).toContain(RESOLVE_FAILURE);
        for (const suite of COLD_SUITES) {
          expect(result.output).toContain(suite);
        }
      } finally {
        writeFileSync(configPath, original);
      }
    },
    VITEST_TIMEOUT_MS,
  );
});
