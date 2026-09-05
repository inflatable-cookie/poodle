// Detects a gate dirtying the working tree.
//
// `ci:web` and `docs:check` both compose write-mode generators: `react:docs`,
// `report:parity` and `report:accessibility` are writers, and `report:parity`
// reaches `tokens:build` in write mode. So a gate can regenerate an artifact,
// disagree with what is committed, and still exit 0 — which is how the web
// gate sat red on `audit:tokens` from 45caae82 until 5854634c with nothing
// pointing at the cause.
//
// This compares the tree before and after, rather than asserting the tree is
// clean. Asserting cleanliness would make the gate unrunnable on uncommitted
// work, which is precisely when you most want to run it. Only paths the gate
// itself changed are reported.
//
//   bun scripts/gate-tree-guard.ts --snapshot   # first step of the gate
//   bun scripts/gate-tree-guard.ts --compare    # last step of the gate
//
// A missing snapshot is a failure, not a pass: it means the gate was composed
// with the compare step but not the snapshot step, and a guard that silently
// passes when misconfigured is worse than no guard.
//
// Snapshot files are keyed by `git rev-parse --show-toplevel` so parallel
// worktrees do not clobber each other in the shared OS temp directory.

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

export function snapshotPathForRoot(toplevel: string): string {
  const hash = createHash("sha256").update(toplevel).digest("hex").slice(0, 16);
  return path.join(os.tmpdir(), `poodle-gate-tree-guard-${hash}.json`);
}

function repositoryRoot(): string {
  const result = spawnSync("git", ["rev-parse", "--show-toplevel"], {
    encoding: "utf8",
  });
  if (result.status !== 0) {
    console.error(`gate-tree-guard: git rev-parse --show-toplevel failed\n${result.stderr}`);
    process.exit(1);
  }
  const toplevel = result.stdout.trim();
  if (toplevel === "") {
    console.error("gate-tree-guard: git rev-parse --show-toplevel returned an empty path");
    process.exit(1);
  }
  return toplevel;
}

function snapshotPath(): string {
  return snapshotPathForRoot(repositoryRoot());
}

function treeState(): Record<string, string> {
  const result = spawnSync("git", ["status", "--porcelain"], { encoding: "utf8" });
  if (result.status !== 0) {
    console.error(`gate-tree-guard: git status failed\n${result.stderr}`);
    process.exit(1);
  }
  const state: Record<string, string> = {};
  for (const line of result.stdout.split("\n")) {
    if (line.trim() === "") continue;
    state[line.slice(3)] = line.slice(0, 2);
  }
  return state;
}

function run(argv: string[]): void {
  const snapshot = snapshotPath();

  if (argv.includes("--snapshot")) {
    fs.writeFileSync(snapshot, JSON.stringify(treeState()));
    process.exit(0);
  }

  if (!argv.includes("--compare")) {
    console.error("gate-tree-guard: pass --snapshot or --compare");
    process.exit(1);
  }

  let before: Record<string, string>;
  try {
    before = JSON.parse(fs.readFileSync(snapshot, "utf8")) as Record<string, string>;
  } catch {
    console.error(
      "gate-tree-guard: no snapshot found. The gate must run `--snapshot` as its first step;\n" +
        "without it this check cannot tell the gate's writes from pre-existing edits.",
    );
    process.exit(1);
  }
  fs.rmSync(snapshot, { force: true });

  const after = treeState();
  const changed = Object.keys(after)
    .filter((file) => after[file] !== before[file])
    .sort();

  if (changed.length === 0) process.exit(0);

  console.error(
    "gate-tree-guard: the gate changed files it did not commit.\n" +
      "A generated artifact disagrees with its generator — run the writer and commit the result.\n",
  );
  for (const file of changed) console.error(`  ${after[file]} ${file}`);
  process.exit(1);
}

if (import.meta.main) {
  run(process.argv);
}
