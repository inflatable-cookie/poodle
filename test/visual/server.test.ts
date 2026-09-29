import { afterAll, describe, expect, test } from "bun:test";
import { createServer } from "node:http";

import { SERVERS } from "./config";
import {
  assertPreviewPortFree,
  formatPreviewSpawnFailure,
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

describe("visual preview port occupancy", () => {
  test("the gate still uses the baseline ports", () => {
    expect(SERVERS.svelte.port).toBe(4174);
    expect(SERVERS.react.port).toBe(4180);
  });

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
