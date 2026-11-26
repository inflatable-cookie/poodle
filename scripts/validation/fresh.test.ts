import { afterEach, describe, expect, test } from "bun:test";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, readlinkSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { delimiter, join } from "node:path";

import { pathWithPinnedBun, pinnedBunVersion } from "./fresh";

const roots: string[] = [];
const version = "1.4.2";

afterEach(() => {
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

function fixture() {
  const root = mkdtempSync(join(tmpdir(), "poodle-fresh-test-"));
  roots.push(root);
  const bin = join(root, "bin");
  const scratch = join(root, "scratch");
  mkdirSync(bin);
  mkdirSync(scratch);
  const env = { ...process.env, PATH: [bin, process.env.PATH].join(delimiter) };
  return { root, bin, scratch, env };
}

function executable(path: string, script: string): void {
  writeFileSync(path, `#!/bin/sh\nset -eu\n${script}\n`, { mode: 0o755 });
}

describe("ci:fresh pinned Bun", () => {
  test("reads an exact bun@x.y.z packageManager pin", () => {
    expect(pinnedBunVersion(JSON.stringify({ packageManager: "bun@1.4.2" }))).toBe("1.4.2");
  });

  test("refuses a missing, ranged or non-Bun pin", () => {
    for (const manager of [undefined, "bun@^1.4.2", "bun@1.4", "npm@10.0.0"]) {
      expect(() => pinnedBunVersion(JSON.stringify({ packageManager: manager }))).toThrow(/exact bun@x\.y\.z/);
    }
  });

  test("the repository pins an exact Bun", () => {
    const root = join(import.meta.dir, "..", "..");
    expect(pinnedBunVersion(readFileSync(join(root, "package.json"), "utf8"))).toMatch(/^\d+\.\d+\.\d+$/);
  });

  test("keeps PATH when its Bun already matches the pin", () => {
    const { bin, scratch, env } = fixture();
    executable(join(bin, "bun"), `test "$*" = "--version" && echo ${version}`);
    expect(pathWithPinnedBun(version, scratch, env)).toBe(env.PATH);
    expect(readdirSync(scratch)).toEqual([]);
  });

  test("resolves the pinned binary and places both Bun commands first on PATH", () => {
    const { bin, scratch, env } = fixture();
    const pinned = join(bin, "pinned-bun");
    executable(join(bin, "bun"), 'test "$*" = "--version" && echo 0.0.1');
    executable(pinned, `test "$*" = "--version" && echo ${version}`);
    executable(join(bin, "bunx"), `test "$1" = bun@${version} && echo "$PINNED_BUN_FIXTURE"`);
    const path = pathWithPinnedBun(version, scratch, { ...env, PINNED_BUN_FIXTURE: pinned });
    expect(path).toBe([scratch, env.PATH].join(delimiter));
    expect(readlinkSync(join(scratch, "bun"))).toBe(pinned);
    expect(readlinkSync(join(scratch, "bunx"))).toBe(pinned);
  });
});

test.each([false, true])("ci:fresh checks Git status after CI (dirties tree: %s)", (dirty) => {
  const { root, bin, env } = fixture();
  const repo = join(root, "repo");
  mkdirSync(repo);
  const git = (...args: string[]) => execFileSync("git", args, { cwd: repo, stdio: "pipe" });
  writeFileSync(join(repo, "package.json"), JSON.stringify({ packageManager: `bun@${version}` }));
  git("init", "-q");
  git("add", "package.json");
  git("-c", "user.name=fixture", "-c", "user.email=fixture@example.com", "commit", "-qm", "init");
  executable(join(bin, "bun"), `if [ "$*" = "--version" ]; then echo ${version}; else test "$*" = "install --frozen-lockfile"; fi`);
  executable(join(bin, "effigy"), `test "$*" = "ci"\n${dirty ? "touch generated.txt" : ":"}`);
  const result = spawnSync(process.execPath, [join(import.meta.dir, "fresh.ts")], {
    cwd: repo,
    env,
    encoding: "utf8",
  });
  expect(result.status, result.stderr).toBe(dirty ? 1 : 0);
  if (dirty) expect(result.stderr).toContain("?? generated.txt");
});
