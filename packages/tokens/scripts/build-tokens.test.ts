/**
 * Planted orphan coverage for `build-tokens.ts --check` and write mode.
 *
 * `--check` used to compare only files the generator still emits, so a
 * committed artifact the emitter no longer writes stayed green. Write mode
 * used to copy an artifact-root orphan into the core/Svelte mirror before
 * unlinking the source, so `--check` stayed red after regeneration. These
 * tests plant that file in the live artifact roots, then always unlink it.
 * The third root is the generated tree inside the poodle-tokens crate, where
 * the Rust artifact family has lived since the crate became self-contained.
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
  path.join(tokensDir, "../contracts/tokens/src/generated", "stale-orphan.rs"),
];

afterEach(() => {
  for (const file of planted) {
    fs.rmSync(file, { force: true });
  }
});

function runScript(args: string[] = []): { status: number; output: string } {
  try {
    const output = execFileSync("bun", [script, ...args], {
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

function runCheck(): { status: number; output: string } {
  return runScript(["--check"]);
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

test("a stale generated file inside the poodle-tokens crate fails --check", () => {
  fs.writeFileSync(planted[2], "// planted orphan\n");
  const result = runCheck();
  expect(result.status).not.toBe(0);
  expect(result.output).toContain(
    "packages/contracts/tokens/src/generated/stale-orphan.rs",
  );
});

test("write mode removes an artifact-root orphan from every root so --check passes", () => {
  fs.writeFileSync(planted[0], "/* planted orphan */\n");
  fs.writeFileSync(planted[1], "/* planted orphan */\n");
  fs.writeFileSync(planted[2], "// planted orphan\n");
  const write = runScript();
  expect(write.status).toBe(0);
  expect(fs.existsSync(planted[0])).toBe(false);
  expect(fs.existsSync(planted[1])).toBe(false);
  expect(fs.existsSync(planted[2])).toBe(false);
  const result = runCheck();
  expect(result.status).toBe(0);
});

test("the live token artifacts pass --check", () => {
  const result = runCheck();
  expect(result.status).toBe(0);
});
