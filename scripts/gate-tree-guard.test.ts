import { expect, test } from "bun:test";
import { execFileSync, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { snapshotPathForRoot } from "./gate-tree-guard.ts";

test("two repository roots get distinct snapshot files", () => {
  const first = snapshotPathForRoot("/tmp/poodle-worktree-a");
  const second = snapshotPathForRoot("/tmp/poodle-worktree-b");
  expect(first).not.toBe(second);
  expect(path.dirname(first)).toBe(os.tmpdir());
  expect(path.dirname(second)).toBe(os.tmpdir());
});

test("the same repository root reuses the same snapshot file", () => {
  const root = "/tmp/poodle-worktree-a";
  expect(snapshotPathForRoot(root)).toBe(snapshotPathForRoot(root));
});

test("the snapshot name is a hash of git rev-parse --show-toplevel", () => {
  const root = "/tmp/poodle-worktree-a";
  const hash = createHash("sha256").update(root).digest("hex").slice(0, 16);
  expect(snapshotPathForRoot(root)).toBe(
    path.join(os.tmpdir(), `poodle-gate-tree-guard-${hash}.json`),
  );
});

test("detects rewrites of dirty and untracked files whose Git statuses stay the same", () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "poodle-tree-guard-test-"));
  const git = (...args: string[]) => execFileSync("git", args, { cwd: root, encoding: "utf8" });
  const guard = (arg: string) => spawnSync(process.execPath, [path.join(import.meta.dir, "gate-tree-guard.ts"), arg], {
    cwd: path.join(root, "nested dir"),
    encoding: "utf8",
  });
  const write = (file: string, contents: string) => writeFileSync(path.join(root, file), contents);
  const tracked = "nested dir/dirty file";
  const untracked = "nested dir/untracked file";
  try {
    git("init", "-q");
    mkdirSync(path.join(root, "nested dir"));
    write(tracked, "original\n");
    git("add", tracked);
    git("-c", "user.name=fixture", "-c", "user.email=fixture@example.com", "commit", "-qm", "init");
    write(tracked, "before\n");
    write(untracked, "before\n");
    const before = git("status", "--porcelain");
    expect(guard("--snapshot").status).toBe(0);
    expect(guard("--compare").status).toBe(0);
    expect(guard("--snapshot").status).toBe(0);
    write(tracked, "rewritten\n");
    write(untracked, "rewritten\n");
    expect(git("status", "--porcelain")).toBe(before);
    const result = guard("--compare");
    expect(result.status).toBe(1);
    expect(result.stderr).toContain(tracked);
    expect(result.stderr).toContain(untracked);
  } finally {
    const gitRoot = git("rev-parse", "--show-toplevel").trim();
    rmSync(snapshotPathForRoot(gitRoot), { force: true });
    rmSync(root, { recursive: true, force: true });
  }
});
