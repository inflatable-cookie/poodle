import { afterAll, describe, expect, test } from "bun:test";
import { createServer } from "node:http";

import { listeningPidsOnPort } from "../../scripts/port-listeners";
import { REPO_ROOT, SERVERS, previewPortsForRoot } from "./config";
import {
  assertPreviewPortFree,
  formatPreviewSpawnFailure,
  isPathInsideRoot,
  preparePreviewPort,
  previewPortOccupant,
  waitForOwnedPreview,
  waitForSpawnedPreview,
} from "./server";

// Each occupancy call spawns `lsof` (a whole-host listen-table scan) and `ps`;
// the scan dominates. Measured 2026-09-29 on an 18-core host at load ~50: one
// lsof 1.1-3.3s, so a single `assertPreviewPortFree` check can take ~3.4s and
// the slowest two-scan test run measured 3.75s. The bound is ~4x the slowest
// measurement (still far under the old 60s squatter poll this test guards
// against); the per-test cap is ~8x the test's loaded runtime rather than
// bun's 5s default.
const OCCUPANCY_CHECK_BOUND_MS = 15_000;
const OCCUPANCY_TEST_TIMEOUT_MS = 30_000;

const planted: { close: () => Promise<void> }[] = [];

afterAll(async () => {
  for (const item of planted) await item.close();
});

function listen404(port: number): Promise<{ port: number; close: () => Promise<void> }> {
  const server = createServer((_req, res) => {
    res.statusCode = 404;
    res.end("squatter");
  });
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        server.close();
        reject(new Error("failed to listen"));
        return;
      }
      resolve({
        port: address.port,
        close: () =>
          new Promise((done, fail) => {
            server.close((error) => (error ? fail(error) : done()));
          }),
      });
    });
  });
}

function listen200(port: number): Promise<{ port: number; close: () => Promise<void> }> {
  const server = createServer((_req, res) => {
    res.statusCode = 200;
    res.end("ok");
  });
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        server.close();
        reject(new Error("failed to listen"));
        return;
      }
      resolve({
        port: address.port,
        close: () =>
          new Promise((done, fail) => {
            server.close((error) => (error ? fail(error) : done()));
          }),
      });
    });
  });
}

async function spawnForeignHttpServer(): Promise<{
  port: number;
  pid: number;
  close: () => Promise<void>;
}> {
  const proc = Bun.spawn(
    [
      "bun",
      "-e",
      'const s = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: () => new Response("foreign") }); console.log(s.port); await new Promise(() => {});',
    ],
    { cwd: "/tmp", stdout: "pipe", stderr: "pipe" },
  );
  const reader = (proc.stdout as ReadableStream<Uint8Array>).getReader();
  const decoder = new TextDecoder();
  let text = "";
  while (!text.includes("\n")) {
    const { done, value } = await reader.read();
    if (done) break;
    if (value) text += decoder.decode(value, { stream: true });
  }
  reader.releaseLock();
  const port = Number.parseInt(text.trim(), 10);
  if (!Number.isInteger(port) || port <= 0) {
    proc.kill();
    await proc.exited;
    throw new Error(`failed to read foreign server port: ${JSON.stringify(text)}`);
  }
  return {
    port,
    pid: proc.pid,
    close: async () => {
      proc.kill();
      await proc.exited;
    },
  };
}

describe("visual preview port occupancy", () => {
  test("a null lsof status retries then fails closed with the port", () => {
    const port = 59997;
    let calls = 0;
    let thrown: Error | undefined;
    try {
      listeningPidsOnPort(port, () => {
        calls += 1;
        return { status: null, signal: null, stdout: "", stderr: "" };
      });
    } catch (error) {
      thrown = error instanceof Error ? error : new Error(String(error));
    }

    expect(calls).toBe(2);
    expect(thrown?.message).toContain(`port ${port}`);
    expect(thrown?.message).toContain("status null");
  });

  test("lsof exit 1 with no output means there are no listeners", () => {
    const port = 59996;
    let calls = 0;
    const pids = listeningPidsOnPort(port, () => {
      calls += 1;
      return { status: 1, signal: null, stdout: "", stderr: "" };
    });

    expect(calls).toBe(1);
    expect(pids).toEqual([]);
  });

  test("assigns deterministic per-worktree ports outside restricted ranges", () => {
    const a = previewPortsForRoot("/tmp/worktree-alpha");
    const b = previewPortsForRoot("/tmp/worktree-beta");
    expect(a).toEqual(previewPortsForRoot("/tmp/worktree-alpha"));
    expect(a).not.toEqual(b);
    for (const ports of [a, b, { svelte: SERVERS.svelte.port, react: SERVERS.react.port }]) {
      expect(ports.svelte).toBeGreaterThanOrEqual(20_000);
      expect(ports.svelte).toBeLessThan(30_000);
      expect(ports.react).toBe(ports.svelte + 1);
    }
  });

  test("isPathInsideRoot only accepts paths inside the checkout", () => {
    expect(isPathInsideRoot(REPO_ROOT, REPO_ROOT)).toBe(true);
    expect(isPathInsideRoot(`${REPO_ROOT}/packages/svelte/preview`, REPO_ROOT)).toBe(true);
    expect(isPathInsideRoot(`${REPO_ROOT}-sibling/packages/svelte/preview`, REPO_ROOT)).toBe(false);
    expect(isPathInsideRoot("/tmp", REPO_ROOT)).toBe(false);
  });

  test("a planted foreign preview on the port is refused and names the owner", async () => {
    const foreign = await spawnForeignHttpServer();
    planted.push(foreign);
    let thrown: Error | undefined;
    try {
      await preparePreviewPort(foreign.port);
    } catch (error) {
      thrown = error instanceof Error ? error : new Error(String(error));
    }
    expect(thrown, "foreign preview must not be reused").toBeTruthy();
    expect(thrown!.message).toContain(`port ${foreign.port}`);
    expect(thrown!.message).toContain(`pid ${foreign.pid}`);
    expect(thrown!.message).toContain("/tmp");
    expect(thrown!.message).toContain("foreign preview");
    expect(thrown!.message).toContain(REPO_ROOT);
  }, OCCUPANCY_TEST_TIMEOUT_MS);

  test("an owned preview inside this checkout is reused", async () => {
    const ownedServer = await listen200(0);
    planted.push(ownedServer);
    const result = await preparePreviewPort(ownedServer.port);
    expect(result.action).toBe("reuse");
    if (result.action === "reuse") {
      expect(result.occupant.pid).toBe(process.pid);
      expect(result.occupant.cwd).toBeTruthy();
      expect(isPathInsideRoot(result.occupant.cwd!, REPO_ROOT)).toBe(true);
    }
  }, OCCUPANCY_TEST_TIMEOUT_MS);

  test("a squatted port fails at once and names the port and process", async () => {
    const squatter = await listen404(0);
    planted.push(squatter);
    const occupant = previewPortOccupant(squatter.port);
    expect(occupant, `expected a listener on ${squatter.port}`).toBeTruthy();
    let thrown: Error | undefined;
    const started = Date.now();
    try {
      assertPreviewPortFree(squatter.port);
    } catch (error) {
      thrown = error instanceof Error ? error : new Error(String(error));
    }
    expect(Date.now() - started).toBeLessThan(OCCUPANCY_CHECK_BOUND_MS);
    expect(thrown, "occupied port must fail closed").toBeTruthy();
    expect(thrown!.message).toContain(`port ${squatter.port}`);
    expect(thrown!.message).toContain(`pid ${occupant!.pid}`);
    expect(thrown!.message).toContain(occupant!.command.split(" ")[0]!);
  }, OCCUPANCY_TEST_TIMEOUT_MS);

  test("the owned-preview waiter reads the spawn handle's exit promise", async () => {
    // The 042 refactor wrapped the spawn in `{ proc, output }`; passing that
    // wrapper to `waitForSpawnedPreview` (which wants `{ exited, output }`) read
    // `undefined.exited` and crashed the gate before its first capture.
    const owner = { proc: { exited: Promise.resolve(7) }, output: () => "boom" };
    let thrown: Error | undefined;
    try {
      await waitForOwnedPreview(owner, 59998, "svelte", 5_000);
    } catch (error) {
      thrown = error instanceof Error ? error : new Error(String(error));
    }
    expect(thrown?.message).toContain("exited 7");
    expect(thrown?.message).toContain("boom");
  });

  test("spawn death names the port and prints the child output", async () => {
    const port = 59999;
    const output = `error when starting dev server:\nPort ${port} is already in use`;
    const proc = {
      exited: Promise.resolve(1),
      output: () => output,
    };
    const started = Date.now();
    let thrown: Error | undefined;
    try {
      await waitForSpawnedPreview(proc, port, "svelte", 5_000);
    } catch (error) {
      thrown = error instanceof Error ? error : new Error(String(error));
    }
    expect(Date.now() - started).toBeLessThan(2_000);
    expect(thrown?.message).toBe(
      formatPreviewSpawnFailure({
        framework: "svelte",
        port,
        exitCode: 1,
        output,
      }),
    );
    expect(thrown?.message).toContain(String(port));
    expect(thrown?.message).toContain(output);
  });
});
