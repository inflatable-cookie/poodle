/**
 * Planted orphan coverage for `build-tokens.ts --check`.
 *
 * `--check` used to compare only files the generator still emits, so a
 * committed artifact the emitter no longer writes stayed green. These tests
 * plant that file in the live artifact roots, then always unlink it.
 */

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, expect, test } from "bun:test";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const tokensDir = path.resolve(scriptDir, "..");
const script = path.join(scriptDir, "build-tokens.ts");
const planted = [
  path.join(tokensDir, "artifacts", "css", "stale-orphan.css"),
  path.join(tokensDir, "../core/src/tokens/generated/css", "stale-orphan.css"),
];

afterEach(() => {
  for (const file of planted) {
    fs.rmSync(file, { force: true });
  }
});

function runCheck(): { status: number; output: string } {
  try {
    const output = execFileSync("bun", [script, "--check"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
    return { status: 0, output };
  } catch (error) {
    const execError = error as { status?: number; stdout?: string | Buffer; stderr?: string | Buffer };
    return {
      status: execError.status ?? 1,
      output: `${execError.stdout?.toString() ?? ""}${execError.stderr?.toString() ?? ""}`,
    };
  }
}

test("a stale committed artifact the generator no longer emits fails --check", () => {
  fs.writeFileSync(planted[0], "/* planted orphan */\n");
  const result = runCheck();
  expect(result.status).not.toBe(0);
  expect(result.output).toContain("packages/tokens/artifacts/css/stale-orphan.css");
});

test("a stale core token mirror the generator no longer emits fails --check", () => {
  fs.writeFileSync(planted[1], "/* planted orphan */\n");
  const result = runCheck();
  expect(result.status).not.toBe(0);
  expect(result.output).toContain("packages/core/src/tokens/generated/css/stale-orphan.css");
});

test("the live token artifacts pass --check", () => {
  const result = runCheck();
  expect(result.status).toBe(0);
});
