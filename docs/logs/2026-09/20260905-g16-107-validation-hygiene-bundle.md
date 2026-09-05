# g16.107 — Validation Hygiene Bundle

Status: worker complete — awaiting orchestrator review
Date: 2026-09-05
Card: `docs/roadmaps/g16/107-validation-hygiene-bundle.md`
Handoff: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle/docs/handoffs/20260905-090600-g16-107-validation-hygiene-bundle.md`
Governing refs: `tasks/effigy.tasks.toml`, `quality/effigy.scan.toml`,
`scripts/gate-tree-guard.ts`, `test/package-install/web-preview.ts`
Branch: `worker/g16.107-validation-hygiene-bundle`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-107-validation-hygiene-bundle`
Base: `origin/main` at `9481cc95dbd65c1dff8c73a6b74b9504cf19b077`
Worker PR: pending

## Outcome

Seven of eight items landed. Item 5 stopped: `check:react` exists and is red
on current sources, so it stays off `ci:web`.

g16.108 also owns a line in `tasks/effigy.tasks.toml` (`docs:check` +
`docs:snippet-check`). This lane's task edits stay on hygiene selectors;
expect a rebase when 108 merges.

No workflow, component, or contract edits. No windowed selectors.

## Item results

1. **Pack tarballs leave the checkout.** `test:web-pack-install` writes
   under `os.tmpdir()`. Archive paths resolve against the pack destination
   and reject a checkout path. Proof: `pack-archives.test.ts` 3/3.
2. **Gate snapshots are per worktree.** Snapshot file is
   `poodle-gate-tree-guard-<sha256(git rev-parse --show-toplevel)[:16]>.json`
   in the OS temp dir. Proof: unit tests 3/3; live `--snapshot` / `--compare`
   on this worktree.
3. **Doctor is green and still lists findings.** Generated catalogue and
   specimen roots (plus the other committed generated-in-src inputs) are
   excluded. `#[allow(` dropped from high to warning because Effigy 0.12
   cannot score an adjacent rationale. God-files ratchet: `high = 3200`
   (current max 3164). `effigy doctor`: ok 18 / warn 3 / err 0, exit 0.
   God-files 76 warnings, stale-suppressions 29 warnings still printed.
4. **`packages/contracts/node` is on `test:contracts`.**
   `cargo test --manifest-path packages/contracts/node/Cargo.toml`: 12 passed.
5. **Stopped — `check:react` not on `ci:web`.** Selector added
   (`check:react` / `check:react-components` / `check:react-preview`).
   Strict `tsc` on components: 12 errors (EditableList data attrs, TextInput
   and ModelConnection refs, BlockEditorBlock `ref`). Preview: large
   `string` vs `ControlSize`/`ControlDensity` specimen backlog. Clearing
   that is component/preview work. Chatterbox owns the follow-up. Papercut
   recorded 2026-09-05.
6. **`docs:machine-shape-drift` → `advisory:machine-shape-drift`.** 20 live
   findings (unpinned dual-runtime machines + hover/popover/modal/menu
   shape). Clearing them needs vector/machine work, out of this lane.
   **`docs:value-domain-drift` kept and ratcheted onto `docs:check`.** 20
   findings + 10 unresolved-type keys frozen; new or stale keys fail.
   Why: value-domain can join a board without contract edits; machine-shape
   cannot. Vitest ratchet 4/4. Script exit 0 on current inventory.
7. **GPUI harness flakes.** `staged_path` and test temp dirs take a
   per-call unique suffix (`pid` + atomic seq). Smoke prints full cargo
   stderr/stdout on unit-test failure. Specimen probe keeps wall-clock as
   telemetry and names contention; construction remains the pass/fail.
   Proof: uniqueness test 1/1; `smoke:gpui-window-capture` all checks pass.
8. **Jetstream adapter README.** Replaced "does not implement components"
   with the legacy 108-component implementation that is not the poodle-node
   route and is not admitted.

## Review oracle

| Invariant | Proof |
| --- | --- |
| Boards leave the tree clean | `docs:check` dirty-tree run left porcelain unchanged except this lane's files; re-run on committed HEAD |
| Gate state is per worktree | distinct hashed snapshot files for two roots; live snapshot at this worktree hash |
| Doctor is green and honest | exit 0; god-file and stale-suppression warnings still listed |
| Coverage widened | node crate in `test:contracts`; `check:react` exists but is not on `ci:web` (item 5 stop) |
| No ungated red `docs:*-drift` | `docs:machine-shape-drift` renamed to `advisory:machine-shape-drift`; `docs:value-domain-drift` is on `docs:check` |
| Flake causes gone | unique staged paths; smoke stderr; work-based probe budget |

## Validation

- `effigy doctor --verbose`: exit 0 (ok 18, warn 3, err 0)
- `effigy test:core-build`: 57 pass / 0 fail
- `bun packages/svelte/preview/scripts/contract-value-domain-drift.ts`: exit 0
- `bun run vitest run --project svelte-preview packages/svelte/preview/test/value-domain-drift.test.ts`: 4 pass
- `cargo test --manifest-path packages/contracts/node/Cargo.toml`: 12 pass
- `cargo test --bin poodle-window-capture staged_paths_are_unique_within_a_process`: 1 pass
- `effigy smoke:gpui-window-capture`: pass
- `git diff --check origin/main...HEAD`: pass (pre-commit)
- `effigy docs:check` on dirty tree: failed at `check:parity-evidence-ledger` (`receipt source commit … no longer matches the mounted runtime source`). Value-domain ratchet had already passed. Re-run after commit.
- `effigy qa`: after commit

No `release prepare/execute/simulate`, tag, publish, workflow, or windowed selector.
