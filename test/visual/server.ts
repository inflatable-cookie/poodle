import { SERVERS, type Framework } from "./config";

/**
 * Boots (or reuses) the two vite previews the gate diffs against each other.
 *
 * The gate outlives any single dev server: a preview started outside the run can
 * die mid-sweep (its parent shell exits), which shows up as a wall of
 * ERR_CONNECTION_REFUSED captures. `ensureUp` lets the run recover instead.
 */

const owned = new Map<Framework, ReturnType<typeof Bun.spawn>>();

async function isUp(port: number, timeoutMs = 5000): Promise<boolean> {
  try {
    const res = await fetch(`http://127.0.0.1:${port}/`, {
      signal: AbortSignal.timeout(timeoutMs),
    });
    return res.ok;
  } catch {
    return false;
  }
}

async function waitForPort(port: number, timeoutMs = 60_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (await isUp(port)) return;
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw new Error(`preview on port ${port} did not come up within ${timeoutMs}ms`);
}

function spawnPreview(framework: Framework): void {
  const { cwd, port } = SERVERS[framework];
  owned.set(
    framework,
    Bun.spawn(
      ["bun", "run", "dev", "--port", String(port), "--strictPort", "--host", "127.0.0.1"],
      { cwd, stdout: "ignore", stderr: "ignore" },
    ),
  );
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
  if (await isUp(port)) return false;
  await new Promise((resolve) => setTimeout(resolve, 2000));
  if (await isUp(port, 10_000)) return false;

  owned.get(framework)?.kill();
  owned.delete(framework);
  spawnPreview(framework);
  await waitForPort(port);
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

    if (await isUp(port)) {
      console.log(`  reusing ${framework} preview already on :${port}`);
      continue;
    }
    spawnPreview(framework);
  }

  await Promise.all(Object.values(SERVERS).map(({ port }) => waitForPort(port)));

  return {
    urls,
    stop: async () => {
      for (const proc of owned.values()) proc.kill();
      await Promise.all([...owned.values()].map((proc) => proc.exited));
      owned.clear();
    },
  };
}
