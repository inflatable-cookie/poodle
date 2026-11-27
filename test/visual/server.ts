import { spawnSync } from "node:child_process";
import { realpathSync } from "node:fs";
import path from "node:path";

import { listeningPidsOnPort } from "../../scripts/port-listeners";
import { REPO_ROOT, SERVERS, type Framework } from "./config";

/**
 * Boots (or reuses) the two vite previews the gate diffs against each other.
 *
 * The gate outlives any single dev server: a preview started outside the run can
 * die mid-sweep (its parent shell exits), which shows up as a wall of
 * ERR_CONNECTION_REFUSED captures. `ensureUp` lets the run recover instead.
 *
 * Ports are deterministic per checkout in `config.ts`, and `preparePreviewPort`
 * checks the listen table before spawning or reusing: a listener whose working
 * directory is outside this checkout is refused immediately with its owner
 * named, and spawn failure prints the child output rather than polling 404s.
 */

type OwnedPreview = {
  proc: ReturnType<typeof Bun.spawn>;
  output: () => string;
};

const owned = new Map<Framework, OwnedPreview>();

export type PortOccupant = { pid: number; command: string; cwd: string | null };

function safeRealpath(target: string): string {
  try {
    return realpathSync(target);
  } catch {
    return path.resolve(target);
  }
}

export function isPathInsideRoot(candidate: string, root: string = REPO_ROOT): boolean {
  const resolvedCandidate = safeRealpath(candidate);
  const resolvedRoot = safeRealpath(root);
  const rel = path.relative(resolvedRoot, resolvedCandidate);
  return rel === "" || (!rel.startsWith("..") && !path.isAbsolute(rel));
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function isUp(port: number, timeoutMs = 5000): Promise<boolean> {
  try {
    const res = await fetch(`http://127.0.0.1:${port}/`, {
      signal: AbortSignal.timeout(timeoutMs),
    });
    return res.ok;
  } catch {
    return false;
  }
}

export function processCwd(
  pid: number,
  spawn: typeof spawnSync = spawnSync,
): string | null {
  const result = spawn("lsof", ["-p", String(pid), "-a", "-d", "cwd", "-Fn"], {
    encoding: "utf8",
  });
  if (result.error || result.status !== 0 || result.signal !== null) {
    return null;
  }
  for (const line of (result.stdout ?? "").split("\n")) {
    if (line.startsWith("n")) {
      const candidate = line.slice(1).trim();
      return candidate ? safeRealpath(candidate) : null;
    }
  }
  return null;
}

function occupantForPid(pid: number): PortOccupant {
  const args = spawnSync("ps", ["-p", String(pid), "-o", "args="], {
    encoding: "utf8",
  });
  const command = args.stdout.trim() || `pid ${pid}`;
  return { pid, command, cwd: processCwd(pid) };
}

export function previewPortOccupant(port: number): PortOccupant | null {
  const pids = listeningPidsOnPort(port);
  const pid = pids[0];
  if (pid === undefined) return null;
  return occupantForPid(pid);
}

export function formatOccupiedPreviewPort(port: number, occupant: PortOccupant): string {
  const cwdNote = occupant.cwd ? `, cwd ${occupant.cwd}` : "";
  return (
    `visual preview port ${port} is already taken by pid ${occupant.pid} (${occupant.command}${cwdNote}); ` +
    "stop that process. The gate uses the per-worktree ports in test/visual/config.ts."
  );
}

export function formatForeignPreviewPort(
  port: number,
  occupant: PortOccupant,
  checkoutRoot: string = REPO_ROOT,
): string {
  const cwdNote = occupant.cwd ?? "unknown";
  return (
    `visual preview port ${port} is held by a foreign preview (pid ${occupant.pid}, ` +
    `cwd ${cwdNote}, ${occupant.command}); expected checkout ${checkoutRoot}. ` +
    "Stop that process instead of reusing another checkout's preview."
  );
}

export function assertPreviewPortFree(port: number): void {
  const occupant = previewPortOccupant(port);
  if (occupant) throw new Error(formatOccupiedPreviewPort(port, occupant));
}

export async function preparePreviewPort(
  port: number,
  checkoutRoot: string = REPO_ROOT,
): Promise<{ action: "reuse"; occupant: PortOccupant } | { action: "spawn" }> {
  if (await isUp(port)) {
    const pids = listeningPidsOnPort(port);
    if (pids.length === 0) {
      throw new Error(
        `visual preview port ${port} answered HTTP but no local listener was found for checkout ${checkoutRoot}`,
      );
    }
    let primary: PortOccupant | undefined;
    for (const pid of pids) {
      const occupant = occupantForPid(pid);
      primary ??= occupant;
      if (!occupant.cwd || !isPathInsideRoot(occupant.cwd, checkoutRoot)) {
        throw new Error(formatForeignPreviewPort(port, occupant, checkoutRoot));
      }
    }
    return { action: "reuse", occupant: primary! };
  }
  assertPreviewPortFree(port);
  return { action: "spawn" };
}

export function formatPreviewSpawnFailure(args: {
  framework: Framework;
  port: number;
  exitCode: number | null;
  output: string;
}): string {
  const body = args.output.trim() || "(no stdout/stderr)";
  return (
    `preview spawn for ${args.framework} on port ${args.port} exited ` +
    `${args.exitCode ?? "unknown"}:\n${body}`
  );
}

export async function waitForSpawnedPreview(
  proc: { exited: Promise<number>; output: () => string },
  port: number,
  framework: Framework,
  timeoutMs = 60_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (await isUp(port)) return;
    const remaining = Math.max(0, Math.min(250, deadline - Date.now()));
    const result = await Promise.race([
      proc.exited.then((exitCode) => ({ kind: "exit" as const, exitCode })),
      sleep(remaining).then(() => ({ kind: "wait" as const })),
    ]);
    if (result.kind === "exit") {
      throw new Error(
        formatPreviewSpawnFailure({
          framework,
          port,
          exitCode: result.exitCode,
          output: proc.output(),
        }),
      );
    }
  }
  throw new Error(`preview on port ${port} did not come up within ${timeoutMs}ms`);
}

function collectSpawnOutput(proc: ReturnType<typeof Bun.spawn>): () => string {
  const chunks: string[] = [];
  const decoder = new TextDecoder();
  const read = async (stream: ReadableStream<Uint8Array> | undefined) => {
    if (!stream) return;
    const reader = stream.getReader();
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      if (value) chunks.push(decoder.decode(value, { stream: true }));
    }
  };
  void read(proc.stdout as ReadableStream<Uint8Array> | undefined);
  void read(proc.stderr as ReadableStream<Uint8Array> | undefined);
  return () => chunks.join("");
}

/**
 * `OwnedPreview` keeps the spawn handle under `proc`; the waiter wants it flat.
 * Passing the wrapper straight to `waitForSpawnedPreview` read
 * `undefined.exited` and crashed the gate before its first capture.
 */
export function waitForOwnedPreview(
  preview: { proc: { exited: Promise<number> }; output: () => string },
  port: number,
  framework: Framework,
  timeoutMs = 60_000,
): Promise<void> {
  return waitForSpawnedPreview(
    { exited: preview.proc.exited, output: preview.output },
    port,
    framework,
    timeoutMs,
  );
}

function killPreviewProcess(proc: ReturnType<typeof Bun.spawn>): void {
  if (proc.pid) {
    try {
      process.kill(-proc.pid, "SIGTERM");
    } catch {
      /* process group may already be gone */
    }
  }
  try {
    proc.kill();
  } catch {
    /* already gone */
  }
}

function spawnPreview(framework: Framework): OwnedPreview {
  const { cwd, port } = SERVERS[framework];
  const proc = Bun.spawn(
    ["bun", "run", "dev", "--port", String(port), "--strictPort", "--host", "127.0.0.1"],
    { cwd, stdout: "pipe", stderr: "pipe", detached: true },
  );
  const preview = { proc, output: collectSpawnOutput(proc) };
  owned.set(framework, preview);
  return preview;
}

/**
 * Restarts the preview if it stopped answering. Returns true if it had to.
 *
 * Two consecutive failed checks are required: a single slow response under a
 * browser-driven load is not a dead server, and restarting on that signal throws
 * away vite's warm module graph — which then makes the next captures slower and
 * the false positive self-sustaining.
 */
export async function ensureUp(framework: Framework): Promise<boolean> {
  const { port } = SERVERS[framework];
  if (await isUp(port)) {
    await preparePreviewPort(port);
    return false;
  }
  await sleep(2000);
  if (await isUp(port, 10_000)) {
    await preparePreviewPort(port);
    return false;
  }

  const previous = owned.get(framework);
  if (previous) {
    killPreviewProcess(previous.proc);
    await previous.proc.exited;
    owned.delete(framework);
  }
  assertPreviewPortFree(port);
  const preview = spawnPreview(framework);
  await waitForOwnedPreview(preview, port, framework);
  return true;
}

export type PreviewServers = {
  urls: Record<Framework, string>;
  stop: () => Promise<void>;
};

export async function startPreviews(): Promise<PreviewServers> {
  const urls = {} as Record<Framework, string>;

  for (const framework of Object.keys(SERVERS) as Framework[]) {
    const { port } = SERVERS[framework];
    urls[framework] = `http://127.0.0.1:${port}`;

    const portState = await preparePreviewPort(port);
    if (portState.action === "reuse") {
      console.log(
        `  reusing ${framework} preview already on :${port} (pid ${portState.occupant.pid})`,
      );
      continue;
    }
    spawnPreview(framework);
  }

  await Promise.all(
    (Object.keys(SERVERS) as Framework[]).map((framework) => {
      const preview = owned.get(framework);
      if (!preview) return Promise.resolve();
      return waitForOwnedPreview(preview, SERVERS[framework].port, framework);
    }),
  );

  return {
    urls,
    stop: async () => {
      for (const item of owned.values()) killPreviewProcess(item.proc);
      await Promise.all([...owned.values()].map((item) => item.proc.exited));
      owned.clear();
    },
  };
}
