# g16.107 — Validation Hygiene Bundle

Status: ready
Type: validation and tooling repair — no component, contract, or workflow change
Opened: 2026-09-05
Depends on: none
Governing refs: `tasks/effigy.tasks.toml`, `quality/effigy.scan.toml`,
`scripts/gate-tree-guard.ts:27`, `test/package-install/web-preview.ts`,
`packages/svelte/preview/scripts/contract-value-domain-drift.ts:20`,
`packages/svelte/preview/scripts/machine-shape-drift.ts`,
`packages/gpui/preview/src/bin/window_capture.rs:444,698`,
`packages/gpui/preview/src/specimen_probe.rs`,
`packages/jetstream/adapter/README.md:17`,
`../../triage/20260901-233708-holistic-posture-assessment.md` (validation
hygiene remainder), root `PAPERCUTS.md`
Dispatch manifest: `../dispatch.md`

## Goal

Close the validation-hygiene remainder from the 2026-09-01 audit in one
bounded lane so boards stop dirtying checkouts, doctor stops crying wolf,
and red-but-ungated checks either join a board or leave.

## Fixed Boundary

Each item is independent; land all of them or report which stopped.

1. `test:web-pack-install` writes its tarballs to a temp directory, never
   into the checkout (`web-preview.ts` around line 230).
2. `gate-tree-guard.ts` keys its snapshot by repository root and worktree
   (a hash of `git rev-parse --show-toplevel`), not one global file.
3. `quality/effigy.scan.toml`: exclude the committed generated catalogue and
   specimen roots the task catalogue calls intentional inputs from
   generated-in-src; scope stale-suppression scoring so an annotated
   `#[allow(...)]` with an adjacent rationale comment is not high; keep
   god-file findings but ratchet them (fail only on growth). `effigy
   doctor` must exit 0 on `main` afterwards with findings still listed.
4. Add `packages/contracts/node` to `test:contracts`.
5. Add a `check:react` typecheck selector and include it in `ci:web`.
6. `docs:machine-shape-drift`: either clear its 20 findings and add it to
   `docs:check`, or move it to an `advisory:` selector group. Report which
   and why. `docs:value-domain-drift`: same choice; if kept report-only, add
   a ratchet that fails on new findings.
7. GPUI harness flakes: `window_capture.rs` temp paths gain a per-call
   unique suffix so parallel tests cannot collide, and the smoke wrapper
   prints stderr on failure; `specimen_probe.rs` caps shard concurrency or
   makes its budget work-based, keeping timing as telemetry.
8. `packages/jetstream/adapter/README.md:17`: replace "does not implement
   components" with the truth: the crate carries a legacy direct
   implementation of 108 components that is not the poodle-node route and
   is not admitted.

No workflow edits, no component changes, no contract changes.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Boards leave the tree clean | run `effigy ci:web` | `git status --porcelain` empty afterwards |
| Gate state is per worktree | two worktrees run `gate:snapshot` | distinct snapshot files |
| Doctor is green and honest | `effigy doctor` on `main` | exit 0; god-file list still printed |
| Coverage widened | `test:contracts`, `ci:web` | node crate tests and React typecheck appear in the board output |
| No ungated red | `effigy tasks` | every `docs:*-drift` selector is in a board or in `advisory:` |
| Flake causes gone | run the two harness tests 10× in parallel with vitest | no collision, budget failure names contention |

## Validation

`effigy qa`, `effigy doctor`, `effigy docs:check`, `git diff --check
origin/main...HEAD`. Never run windowed selectors.

## Owned Paths

`tasks/effigy.tasks.toml`, `quality/effigy.scan.toml`,
`scripts/gate-tree-guard.ts`, `test/package-install/web-preview.ts`,
`packages/svelte/preview/scripts/{machine-shape-drift,contract-value-domain-drift}.ts`,
`packages/gpui/preview/src/bin/window_capture.rs`,
`packages/gpui/preview/scripts/window-capture-smoke.ts`,
`packages/gpui/preview/src/specimen_probe.rs`,
`packages/jetstream/adapter/README.md`, tests for each, execution log,
`PAPERCUTS.md` (append and close entries this card resolves).

## Stop Conditions

Stop an item, not the card, when it needs a design decision (report it in
the log). Escalation owner: Chatterbox.
