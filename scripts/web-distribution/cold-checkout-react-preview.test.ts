import { afterAll, describe, expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
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

// Checkout is a detached worktree, workspace node_modules links, and a core
// dist copy — not a recursive `find`+`cp -a` of every package store. Vitest
// runs in its own process group so a timeout SIGKILLs workers instead of
// spending minutes unwinding them.
// Measured 2026-09-28 at load 141/143/130: checkout 2.9s, passing suites
// 15.5s, planted-alias rerun 6.3s, file 27s. The vitest child cap is 120s
// (~8x the loaded passing run). Each bun:test wrapper is also 120s: checkout
// is now ~3s, so that budget is almost entirely vitest. A genuine hang still
// dies inside ci:web's 5-minute child bound.
const CHECKOUT_STEP_MS = 60_000;
const CORE_DIST_COPY_MS = 30_000;
const VITEST_CHILD_MS = 120_000;
const COLD_PASS_TEST_MS = 120_000;
const COLD_FAIL_TEST_MS = 120_000;
const CLEANUP_MS = 30_000;

const childEnv = { ...process.env };
delete childEnv.FORCE_COLOR;

let coldRoot: string | undefined;

function run(
  command: string,
  args: string[],
  cwd: string,
  timeout = CHECKOUT_STEP_MS,
): ReturnType<typeof spawnSync> {
  return spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    env: childEnv,
    timeout,
  });
}

function runProcessGroup(
  command: string,
  args: string[],
  cwd: string,
  timeout: number,
): Promise<{ status: number | null; stdout: string; stderr: string; timedOut: boolean }> {
  return new Promise((resolve) => {
    let settled = false;
    let timedOut = false;
    let stdout = "";
    let stderr = "";
    let timer: ReturnType<typeof setTimeout> | undefined;
    const child = spawn(command, args, {
      cwd,
      env: childEnv,
      stdio: ["ignore", "pipe", "pipe"],
      detached: true,
    });
    const finish = (status: number | null) => {
      if (settled) return;
      settled = true;
      if (timer !== undefined) clearTimeout(timer);
      resolve({ status, stdout, stderr, timedOut });
    };
    timer = setTimeout(() => {
      timedOut = true;
      if (child.pid !== undefined) {
        try {
          process.kill(-child.pid, "SIGKILL");
        } catch {
          try {
            child.kill("SIGKILL");
          } catch {
            /* already gone */
          }
        }
      }
      setTimeout(() => finish(null), 2_000);
    }, timeout);
    child.stdout?.setEncoding("utf8");
    child.stderr?.setEncoding("utf8");
    child.stdout?.on("data", (chunk: string) => {
      stdout += chunk;
    });
    child.stderr?.on("data", (chunk: string) => {
      stderr += chunk;
    });
    child.on("close", (status) => finish(status));
    child.on("error", (error) => {
      stderr += `${error.message}\n`;
      finish(null);
    });
  });
}

function linkWorkspaceNodeModules(fromRoot: string, toRoot: string): void {
  const manifest = JSON.parse(readFileSync(join(fromRoot, "package.json"), "utf8")) as {
    workspaces?: string[];
  };
  for (const rel of manifest.workspaces ?? []) {
    const source = join(fromRoot, rel, "node_modules");
    if (!existsSync(source)) continue;
    const dest = join(toRoot, rel, "node_modules");
    mkdirSync(dirname(dest), { recursive: true });
    symlinkSync(source, dest);
  }
}

function createColdCheckout(): string {
  const parent = mkdtempSync(join(tmpdir(), "poodle-cold-web-"));
  const root = join(parent, "checkout");
  const added = run("git", ["worktree", "add", "--detach", root, "HEAD"], repoRoot, CHECKOUT_STEP_MS);
  if (added.status !== 0) {
    throw new Error(`git worktree add failed: ${added.stderr}${added.stdout}`);
  }
  copyFileSync(join(repoRoot, "vitest.config.ts"), join(root, "vitest.config.ts"));
  symlinkSync(join(repoRoot, "node_modules"), join(root, "node_modules"));
  linkWorkspaceNodeModules(repoRoot, root);
  const coreDist = join(repoRoot, "packages/core/dist");
  if (!existsSync(coreDist)) {
    throw new Error("packages/core/dist missing; run core:build before this proof");
  }
  const copiedCore = run("cp", ["-a", coreDist, join(root, "packages/core/dist")], repoRoot, CORE_DIST_COPY_MS);
  if (copiedCore.status !== 0) {
    throw new Error(`cp core dist failed: ${copiedCore.stderr}`);
  }
  return root;
}

function removeColdCheckout(root: string): void {
  run("git", ["worktree", "remove", "--force", root], repoRoot, CHECKOUT_STEP_MS);
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

async function runColdSuites(cwd: string): Promise<{ status: number | null; output: string }> {
  // Call the checkout's vitest binary. `bunx vitest … -- <files>` lets bunx
  // swallow the `--` and drop the file filters, so the whole react-preview
  // include runs — including suites that import `lucide-static/icon-nodes.json`
  // through the root `node_modules` symlink.
  const vitest = join(cwd, "node_modules", ".bin", "vitest");
  const result = await runProcessGroup(vitest, ["run", "--project", "react-preview", ...COLD_SUITES], cwd, VITEST_CHILD_MS);
  const output = `${result.stdout}\n${result.stderr}`;
  if (result.timedOut) {
    throw new Error(`vitest exceeded ${VITEST_CHILD_MS}ms and was killed with its process group\n${output}`);
  }
  return {
    status: result.status,
    output,
  };
}

afterAll(() => {
  if (coldRoot) {
    removeColdCheckout(coldRoot);
    coldRoot = undefined;
  }
}, CLEANUP_MS);

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
    async () => {
      coldRoot = createColdCheckout();
      expect(existsSync(join(coldRoot, "packages/react/components/dist"))).toBe(false);
      expect(existsSync(join(coldRoot, "packages/svelte/components/dist"))).toBe(false);
      expect(readFileSync(join(coldRoot, "vitest.config.ts"), "utf8")).toMatch(REACT_PREVIEW_ALIAS);
      const result = await runColdSuites(coldRoot);
      expect(result.output, result.output).not.toContain(RESOLVE_FAILURE);
      expect(result.output, result.output).not.toContain("g18-019-markdown-renderer.test.tsx");
      expect(result.status, result.output).toBe(0);
    },
    COLD_PASS_TEST_MS,
  );

  test(
    "removing the react-preview alias fails the same three suites with Failed to resolve import",
    async () => {
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
        const result = await runColdSuites(coldRoot);
        expect(result.status).not.toBe(0);
        expect(result.output).toContain(RESOLVE_FAILURE);
        for (const suite of COLD_SUITES) {
          expect(result.output).toContain(suite);
        }
      } finally {
        writeFileSync(configPath, original);
      }
    },
    COLD_FAIL_TEST_MS,
  );
});
