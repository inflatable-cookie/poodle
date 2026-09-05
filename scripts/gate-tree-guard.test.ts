import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
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
