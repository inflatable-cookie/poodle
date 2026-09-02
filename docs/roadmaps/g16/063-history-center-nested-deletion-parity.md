# g16.063 — HistoryCenter Nested Deletion Parity

Status: complete
Type: paired semantic repair
Opened: 2026-09-02
Implemented: 2026-09-02
Completed: 2026-09-02
Depends on: current HistoryCenter contract and machines
Governing refs: `nucleus-gpui-parity-programme.md`,
`../../contracts/components/history-center.md`
Log: `../../logs/2026-09/20260902-g16-063-history-center-nested-deletion-parity.md`
PR: #167
Merge: `3cbc81d3f7d4beed4dae0e0ff49870a2febb3dc5`

## Goal

Make deletion of a nested continuation update the actual nested level in both
TypeScript and Rust. Preserve unrelated branches and emit the same delete and
reload effects.

## Outcome

One shared nested tree (`e1` → inner `l1a`/`l1b`, sibling root `e0`) now
runs in core and headless. Deleting inner fork `n1` invalidates `l1a` in
place, keeps `e0` and `l1b` structurally equal, leaves `e1`'s
continuations/pick/run untouched, and emits one `deleteContinuation` plus one
`loadContinuations` for `l1a`. TypeScript now uses the existing recursive
`replaceLevel` helper; Rust already did. Root-level deletion tests are
unchanged.

## Fixed Boundary

The TypeScript root-level `Map.set(anchorEntryId, ...)` path is not recursive;
the Rust machine already has recursive replacement but lacks a nested-delete
counterexample. Add one paired vector first, then align behavior. Do not change
public props, rejection meanings, consumer persistence, or Nucleus.

## Acceptance

- The same nested continuation tree is exercised in core and headless tests.
- Before the repair, the TypeScript counterexample leaves the stale descendant.
- After repair, the nested level is invalidated at its real location, unrelated
  branches remain byte/structurally equal, and both machines emit one delete
  plus the intended anchor reload.
- Root-level deletion behavior remains unchanged.

## Review Oracle

| Invariant | Counterexample | Required proof |
| --- | --- | --- |
| Replacement is recursive | delete a fork inside an inner run | stale nested level is absent |
| Siblings survive | second inner/root branch exists | equality proof retains it |
| Effects stay exact | recursive repair emits twice | paired effect vectors fail |
| Ledger cannot close defect | edit only evidence prose | paired regression remains red |

## Writable Scope

HistoryCenter core/headless machines, paired vectors/tests, contract wording if
needed, this card, one log, and new papercuts. No web-shell API, renderer
appearance, Nucleus, release, workflow, lab, or Jetstream changes.

## Validation

Run focused core and headless HistoryCenter tests, paired contract checks,
`effigy ci:web`, `effigy ci:rust`, `effigy docs:check`, and `git diff --check
origin/main...HEAD`. No windowed selector.

## Stop Conditions

Stop if the counterexample reveals an unresolved public identity/persistence
decision or if TypeScript and Rust cannot share the same semantic vector.

## Continuation

Record the repaired pair in the Nucleus confidence evidence. It is not a
dependency for unrelated cohort components. Front doors stay orchestrator-owned.
