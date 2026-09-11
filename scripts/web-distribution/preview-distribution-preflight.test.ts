import { afterAll, describe, expect, test } from "bun:test";
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { createHash } from "node:crypto";
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  realpathSync,
  rmSync,
  statSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";

import { findRepoRoot } from "./core-build";

const repoRoot = findRepoRoot();

const PLANTED_EXPORT = "installCodeEditorFocusEntry";
const STALE_EDITOR_MARKER = "g18_015_stale_framework_dist";

const DIST_TREES = [
  "packages/core/dist",
  "packages/svelte/components/dist",
  "packages/react/components/dist",
] as const;

const SVELTE_EDITOR_DIST = "packages/svelte/components/dist/editor.client.js";
const REACT_EDITOR_DIST = "packages/react/components/dist/editor.js";
const CORE_DIST_INDEX = "packages/core/dist/index.js";
const SVELTE_ENGINE_SOURCE = "packages/svelte/components/src/code-editor-engine.ts";
const REACT_ENGINE_SOURCE = "packages/react/components/src/code-editor-engine.ts";

const RUN_TIMEOUT_MS = 120_000;
const PREVIEW_TIMEOUT_MS = 360_000;
const CLEANUP_TIMEOUT_MS = 60_000;

const childEnv = { ...process.env };
delete childEnv.FORCE_COLOR;

let fixtureRoot: string | undefined;
let liveFingerprint: Record<string, string> | undefined;
const owned = new Set<OwnedProcess>();

type OwnedProcess = {
  child: ChildProcess;
  port: number;
  log: { text: () => string };
};

function run(
  command: string,
  args: string[],
  cwd: string,
  timeout = RUN_TIMEOUT_MS,
): ReturnType<typeof spawnSync> {
  return spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    env: childEnv,
    timeout,
  });
}

function hashTree(root: string): string {
  const hash = createHash("sha256");
  const visit = (directory: string) => {
    for (const entry of readdirSync(directory).sort()) {
      const full = join(directory, entry);
      const rel = full.slice(root.length + 1);
      const stat = statSync(full);
      if (stat.isDirectory()) {
        hash.update(`dir:${rel}\n`);
        visit(full);
        continue;
      }
      hash.update(`file:${rel}:${sha256File(full)}\n`);
    }
  };
  visit(root);
  return hash.digest("hex");
}

function sha256File(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function fingerprintDist(root: string): Record<string, string> {
  const out: Record<string, string> = {};
  for (const rel of DIST_TREES) {
    const abs = join(root, rel);
    out[rel] = existsSync(abs) ? hashTree(abs) : "absent";
  }
  return out;
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

function createFixture(): string {
  const parent = mkdtempSync(join(tmpdir(), "poodle-preview-preflight-"));
  const root = join(parent, "checkout");
  const added = run("git", ["worktree", "add", "--detach", root, "HEAD"], repoRoot, 60_000);
  if (added.status !== 0) {
    throw new Error(`git worktree add failed: ${added.stderr}${added.stdout}`);
  }
  copyFileSync(join(repoRoot, "tasks/effigy.tasks.toml"), join(root, "tasks/effigy.tasks.toml"));
  symlinkSync(join(repoRoot, "node_modules"), join(root, "node_modules"));
  copyPackageNodeModules(repoRoot, root);
  return root;
}

function removeFixture(root: string): void {
  run("git", ["worktree", "remove", "--force", root], repoRoot, 60_000);
  run("git", ["worktree", "prune"], repoRoot, 30_000);
  rmSync(dirname(root), { recursive: true, force: true });
}

function sequenceFor(toml: string, name: string): Array<{ task?: string; run?: string }> {
  const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const block = toml.match(new RegExp(`"${escaped}"\\s*=\\s*\\[([\\s\\S]*?)\\]`));
  if (block) {
    const steps: Array<{ task?: string; run?: string }> = [];
    for (const item of block[1].matchAll(/\{([^}]*)\}/g)) {
      const task = item[1].match(/task\s*=\s*"([^"]+)"/);
      const command = item[1].match(/run\s*=\s*"([^"]+)"/);
      if (!task && !command) continue;
      const step: { task?: string; run?: string } = {};
      if (task) step.task = task[1];
      if (command) step.run = command[1];
      steps.push(step);
    }
    return steps;
  }
  const scalar = toml.match(new RegExp(`"${escaped}"\\s*=\\s*"([^"]+)"`));
  if (scalar) return [{ run: scalar[1] }];
  throw new Error(`task ${name} missing`);
}

function freePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = createServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        server.close();
        reject(new Error("failed to allocate an ephemeral port"));
        return;
      }
      const port = address.port;
      server.close((error) => {
        if (error) reject(error);
        else resolve(port);
      });
    });
  });
}

function listenersOn(port: number): number[] {
  const result = spawnSync("lsof", ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN", "-t"], {
    encoding: "utf8",
  });
  return result.stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .map(Number)
    .filter((pid) => Number.isInteger(pid) && pid > 0);
}

function isAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

function drain(child: ChildProcess): { text: () => string } {
  let data = "";
  const collect = (stream: NodeJS.ReadableStream | null) => {
    stream?.on("data", (chunk: Buffer | string) => {
      data += chunk.toString();
    });
  };
  collect(child.stdout);
  collect(child.stderr);
  return { text: () => data };
}

function patchPreviewPort(root: string, framework: "svelte" | "react", port: number): void {
  const configPath = join(root, `packages/${framework}/preview/vite.config.ts`);
  const original = readFileSync(configPath, "utf8");
  if (!/server:\s*\{/.test(original)) {
    throw new Error(`no Vite server block in ${configPath}:\n${original}`);
  }
  let next = original.replace(/port:\s*\d+/, `port: ${port}`);
  if (!/strictPort\s*:/.test(next)) {
    next = next.replace(/server:\s*\{/, "server: {\n    strictPort: true,");
  }
  if (/host:\s*["'][^"']+["']/.test(next)) {
    next = next.replace(/host:\s*["'][^"']+["']/, 'host: "127.0.0.1"');
  } else {
    next = next.replace(/server:\s*\{/, 'server: {\n    host: "127.0.0.1",');
  }
  if (!next.includes(`port: ${port}`) || !next.includes('host: "127.0.0.1"') || !/strictPort\s*:/.test(next)) {
    throw new Error(`failed to patch ${framework} vite port in ${configPath}:\n${original}\n---\n${next}`);
  }
  writeFileSync(configPath, next);
}

function stripNamedExport(source: string, name: string): string {
  if (!source.includes(name)) {
    throw new Error(`cannot plant stale dist: ${name} is absent`);
  }
  const stripped = source
    .replace(new RegExp(`\\b${name}\\s*,\\s*`, "g"), "")
    .replace(new RegExp(`,\\s*\\b${name}\\b`, "g"), "");
  const exportBlock = stripped.match(/export\s*\{[\s\S]*\}/)?.[0] ?? "";
  if (new RegExp(`\\b${name}\\b`).test(exportBlock)) {
    throw new Error(`failed to strip ${name} from the dist export list`);
  }
  return stripped;
}

function distHasExport(root: string): boolean {
  const path = join(root, CORE_DIST_INDEX);
  if (!existsSync(path)) return false;
  const exportBlock = readFileSync(path, "utf8").match(/export\s*\{[\s\S]*\}/)?.[0] ?? "";
  return new RegExp(`\\b${PLANTED_EXPORT}\\b`).test(exportBlock);
}

function plantStaleCore(root: string): void {
  const path = join(root, CORE_DIST_INDEX);
  if (!existsSync(path)) {
    throw new Error("stale plant needs an existing core dist");
  }
  writeFileSync(path, stripNamedExport(readFileSync(path, "utf8"), PLANTED_EXPORT));
  expect(distHasExport(root)).toBe(false);
}

function plantStaleEditor(root: string, framework: "svelte" | "react"): void {
  const rel = framework === "svelte" ? SVELTE_EDITOR_DIST : REACT_EDITOR_DIST;
  const path = join(root, rel);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `export const ${STALE_EDITOR_MARKER} = true;\n`);
}

function startSelector(root: string, selector: string, port: number): OwnedProcess {
  if (listenersOn(port).length > 0) {
    throw new Error(`port ${port} already has a listener`);
  }
  const child = spawn("effigy", [selector], {
    cwd: root,
    env: childEnv,
    stdio: ["ignore", "pipe", "pipe"],
    detached: true,
  });
  const log = drain(child);
  const ownedProcess = { child, port, log };
  owned.add(ownedProcess);
  return ownedProcess;
}

async function stopOwned(target?: OwnedProcess): Promise<void> {
  const batch = target ? [target] : [...owned];
  for (const item of batch) {
    const pids = new Set<number>(listenersOn(item.port));
    if (item.child.pid) pids.add(item.child.pid);
    for (const pid of pids) {
      try {
        process.kill(-pid, "SIGTERM");
      } catch {
        /* process group may not exist */
      }
      try {
        process.kill(pid, "SIGTERM");
      } catch {
        /* already gone */
      }
    }
    const deadline = Date.now() + 5_000;
    while (Date.now() < deadline && (item.child.exitCode === null && item.child.signalCode === null)) {
      await sleep(50);
    }
    for (const pid of new Set<number>([...pids, ...listenersOn(item.port)])) {
      if (!isAlive(pid)) continue;
      try {
        process.kill(-pid, "SIGKILL");
      } catch {
        /* ignore */
      }
      try {
        process.kill(pid, "SIGKILL");
      } catch {
        /* ignore */
      }
    }
    try {
      item.child.kill("SIGKILL");
    } catch {
      /* ignore */
    }
    await Promise.race([
      new Promise<void>((resolve) => item.child.once("exit", () => resolve())),
      sleep(2_000),
    ]);
    expect(listenersOn(item.port), item.log.text()).toEqual([]);
    if (item.child.pid) expect(isAlive(item.child.pid)).toBe(false);
    owned.delete(item);
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForListener(
  item: OwnedProcess,
  root: string,
  timeout: number,
  requireFreshCore: boolean,
): Promise<void> {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    if (item.child.exitCode !== null || item.child.signalCode !== null) {
      throw new Error(
        `selector exited ${item.child.exitCode ?? item.child.signalCode} before listen\n${item.log.text()}`,
      );
    }
    const listening = listenersOn(item.port).length > 0;
    const fresh = distHasExport(root);
    if (listening && requireFreshCore && !fresh) {
      throw new Error(`Vite listened on ${item.port} before core dist exported ${PLANTED_EXPORT}\n${item.log.text()}`);
    }
    if (listening) {
      try {
        const response = await fetch(`http://127.0.0.1:${item.port}/`, {
          signal: AbortSignal.timeout(1_000),
        });
        if (response.ok || response.status === 404) return;
      } catch {
        /* not ready */
      }
    }
    await sleep(150);
  }
  throw new Error(`timed out waiting for ${item.port}\n${item.log.text()}`);
}

async function waitForExit(item: OwnedProcess, timeout: number): Promise<number> {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    if (item.child.exitCode !== null) {
      if (listenersOn(item.port).length > 0) {
        throw new Error(`builder failed but Vite still listened on ${item.port}\n${item.log.text()}`);
      }
      return item.child.exitCode;
    }
    if (listenersOn(item.port).length > 0) {
      throw new Error(`builder failed but Vite still listened on ${item.port}\n${item.log.text()}`);
    }
    await sleep(100);
  }
  if (item.child.exitCode !== null) return item.child.exitCode;
  throw new Error(`timed out waiting for fail-closed exit\n${item.log.text()}`);
}

function candidateFsUrls(root: string, rel: string, port: number): string[] {
  const absolute = join(root, rel);
  const paths = [absolute];
  try {
    paths.push(realpathSync(absolute));
  } catch {
    /* missing until preflight writes it */
  }
  try {
    paths.push(join(realpathSync(root), rel));
  } catch {
    /* ignore */
  }
  return [...new Set(paths)].map((path) => `http://127.0.0.1:${port}/@fs${path}`);
}

async function fetchFirstOk(urls: string[]): Promise<{ url: string; body: string }> {
  const errors: string[] = [];
  for (const url of urls) {
    try {
      const response = await fetch(url, { signal: AbortSignal.timeout(5_000) });
      if (!response.ok) {
        errors.push(`${url} -> ${response.status}`);
        continue;
      }
      return { url, body: await response.text() };
    } catch (error) {
      errors.push(`${url} -> ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  throw new Error(`no served module responded:\n${errors.join("\n")}`);
}

async function importNamed(source: string, name: string): Promise<"present" | "missing"> {
  const dir = mkdtempSync(join(tmpdir(), "poodle-served-mod-"));
  const modPath = join(dir, "mod.mjs");
  const probePath = join(dir, "probe.mjs");
  writeFileSync(modPath, source);
  writeFileSync(
    probePath,
    `import { ${name} } from ${JSON.stringify(pathToFileURL(modPath).href)};\nvoid ${name};\n`,
  );
  try {
    await import(`${pathToFileURL(probePath).href}?t=${Date.now()}`);
    rmSync(dir, { recursive: true, force: true });
    return "present";
  } catch (error) {
    rmSync(dir, { recursive: true, force: true });
    const text = error instanceof Error ? `${error.name}: ${error.message}` : String(error);
    if (text.includes(name)) return "missing";
    throw error;
  }
}

async function assertServedMismatch(
  root: string,
  port: number,
  framework: "svelte" | "react",
): Promise<void> {
  const engineRel = framework === "svelte" ? SVELTE_ENGINE_SOURCE : REACT_ENGINE_SOURCE;
  const engine = await fetchFirstOk(candidateFsUrls(root, engineRel, port));
  expect(engine.body, engine.url).toContain(PLANTED_EXPORT);
  const core = await fetchFirstOk(candidateFsUrls(root, CORE_DIST_INDEX, port));
  expect(core.body, core.url).not.toMatch(new RegExp(`export\\s*\\{[\\s\\S]*\\b${PLANTED_EXPORT}\\b`));
  expect(await importNamed(core.body, PLANTED_EXPORT)).toBe("missing");
}

async function assertServedFresh(
  root: string,
  port: number,
  framework: "svelte" | "react",
): Promise<void> {
  const engineRel = framework === "svelte" ? SVELTE_ENGINE_SOURCE : REACT_ENGINE_SOURCE;
  const editorRel = framework === "svelte" ? SVELTE_EDITOR_DIST : REACT_EDITOR_DIST;
  const engine = await fetchFirstOk(candidateFsUrls(root, engineRel, port));
  expect(engine.body, engine.url).toContain(PLANTED_EXPORT);
  const core = await fetchFirstOk(candidateFsUrls(root, CORE_DIST_INDEX, port));
  expect(core.body, core.url).toMatch(new RegExp(`export\\s*\\{[\\s\\S]*\\b${PLANTED_EXPORT}\\b`));
  expect(await importNamed(core.body, PLANTED_EXPORT)).toBe("present");
  const editor = await fetchFirstOk(candidateFsUrls(root, editorRel, port));
  expect(editor.body, editor.url).not.toContain(STALE_EDITOR_MARKER);
  expect(editor.body, editor.url).toContain(PLANTED_EXPORT);
}

function plantBuilderFailure(root: string): void {
  writeFileSync(
    join(root, "scripts/web-distribution/core-build.ts"),
    'throw new Error("planted g18.015 core-build failure");\n',
  );
}

afterAll(async () => {
  await stopOwned();
  if (liveFingerprint) {
    expect(fingerprintDist(repoRoot)).toEqual(liveFingerprint);
  }
  if (fixtureRoot) {
    removeFixture(fixtureRoot);
    fixtureRoot = undefined;
  }
}, CLEANUP_TIMEOUT_MS);

describe("g18.015 preview distribution build preflight", () => {
  test("public preview selectors compose package builders before the low-level Vite run", () => {
    const toml = readFileSync(join(repoRoot, "tasks/effigy.tasks.toml"), "utf8");
    expect(sequenceFor(toml, "svelte:preview")).toEqual([
      { task: "svelte:package" },
      { task: "svelte:run" },
    ]);
    expect(sequenceFor(toml, "react:preview")).toEqual([
      { task: "react:package" },
      { task: "react:run" },
    ]);
    expect(sequenceFor(toml, "svelte:package")[0]).toEqual({ task: "core:build" });
    expect(sequenceFor(toml, "react:package")[0]).toEqual({ task: "core:build" });
    expect(sequenceFor(toml, "svelte:run")).toEqual([
      { run: "bun run --cwd packages/svelte/preview dev" },
    ]);
    expect(sequenceFor(toml, "react:run")).toEqual([
      { run: "bun run --cwd packages/react/preview dev" },
    ]);
    expect(sequenceFor(toml, "docs:dev")).toEqual([{ task: "svelte:preview" }]);
  });

  test(
    "isolated missing and stale distributions rebuild before Vite, and raw run still serves the mismatch",
    async () => {
      liveFingerprint = fingerprintDist(repoRoot);
      fixtureRoot = createFixture();
      const fixture = fixtureRoot;
      expect(existsSync(join(fixture, CORE_DIST_INDEX))).toBe(false);

      const svelteMissingPort = await freePort();
      patchPreviewPort(fixture, "svelte", svelteMissingPort);
      const svelteMissing = startSelector(fixture, "svelte:preview", svelteMissingPort);
      await waitForListener(svelteMissing, fixture, PREVIEW_TIMEOUT_MS, true);
      expect(distHasExport(fixture)).toBe(true);
      await assertServedFresh(fixture, svelteMissingPort, "svelte");
      await stopOwned(svelteMissing);

      plantStaleCore(fixture);
      plantStaleEditor(fixture, "svelte");
      const svelteRunPort = await freePort();
      patchPreviewPort(fixture, "svelte", svelteRunPort);
      const svelteRun = startSelector(fixture, "svelte:run", svelteRunPort);
      await waitForListener(svelteRun, fixture, RUN_TIMEOUT_MS, false);
      expect(distHasExport(fixture)).toBe(false);
      expect(readFileSync(join(fixture, SVELTE_EDITOR_DIST), "utf8")).toContain(STALE_EDITOR_MARKER);
      await assertServedMismatch(fixture, svelteRunPort, "svelte");
      await stopOwned(svelteRun);

      const svelteStalePort = await freePort();
      patchPreviewPort(fixture, "svelte", svelteStalePort);
      const svelteStale = startSelector(fixture, "svelte:preview", svelteStalePort);
      await waitForListener(svelteStale, fixture, PREVIEW_TIMEOUT_MS, true);
      expect(distHasExport(fixture)).toBe(true);
      expect(readFileSync(join(fixture, SVELTE_EDITOR_DIST), "utf8")).not.toContain(STALE_EDITOR_MARKER);
      await assertServedFresh(fixture, svelteStalePort, "svelte");
      await stopOwned(svelteStale);

      plantStaleCore(fixture);
      plantStaleEditor(fixture, "react");
      const reactRunPort = await freePort();
      patchPreviewPort(fixture, "react", reactRunPort);
      const reactRun = startSelector(fixture, "react:run", reactRunPort);
      await waitForListener(reactRun, fixture, RUN_TIMEOUT_MS, false);
      expect(distHasExport(fixture)).toBe(false);
      await assertServedMismatch(fixture, reactRunPort, "react");
      await stopOwned(reactRun);

      const reactStalePort = await freePort();
      patchPreviewPort(fixture, "react", reactStalePort);
      const reactStale = startSelector(fixture, "react:preview", reactStalePort);
      await waitForListener(reactStale, fixture, PREVIEW_TIMEOUT_MS, true);
      expect(distHasExport(fixture)).toBe(true);
      expect(readFileSync(join(fixture, REACT_EDITOR_DIST), "utf8")).not.toContain(STALE_EDITOR_MARKER);
      await assertServedFresh(fixture, reactStalePort, "react");
      await stopOwned(reactStale);

      plantBuilderFailure(fixture);
      for (const selector of ["svelte:preview", "react:preview"] as const) {
        const port = await freePort();
        patchPreviewPort(fixture, selector.startsWith("svelte") ? "svelte" : "react", port);
        const item = startSelector(fixture, selector, port);
        const code = await waitForExit(item, RUN_TIMEOUT_MS);
        expect(code, item.log.text()).not.toBe(0);
        expect(item.log.text()).toContain("planted g18.015 core-build failure");
        expect(listenersOn(port)).toEqual([]);
        if (item.child.pid) expect(isAlive(item.child.pid)).toBe(false);
        await stopOwned(item);
      }

      expect(fingerprintDist(repoRoot)).toEqual(liveFingerprint);
    },
    PREVIEW_TIMEOUT_MS * 4,
  );
});
