# g16.092 — Native Fresh-Consumer tinyvec Build Repair

Status: ready
Type: repository validation repair
Opened: 2026-09-03
Depends on: reproduced `effigy ci:native` failure on merged `main` `4a615e990`
Governing refs: `../../contracts/001-working-rules.md`, `091-nucleus-toast-host-m1.md`
Handoff: `../../handoffs/20260903-223000-g16-092-native-consumer-tinyvec-build.md`

## Goal

Restore the fresh dual-dependency consumer compilation under the repository's
Rust 1.95 toolchain. The current gate reaches crates.io `tinyvec 1.13.0` with
an alloc-only configuration and fails because the crate cannot resolve the
`vec!` macro. The identical failure occurs on the exact pre-ToastHost base.

## Fixed Boundary

- Reproduce the failure through the production consumer gate and preserve its
  positive GPUI identity checks and degraded negative control.
- Find the dependency/feature/lock path that selects the broken shape. Apply
  the smallest repository-owned fix: a compatible lock/dependency resolution
  or feature declaration. Do not vendor or patch third-party source unless a
  narrower supported resolution is impossible and review explicitly accepts it.
- Prove the repair on a clean fresh consumer under Rust 1.95. Do not suppress
  compiler diagnostics, skip the consumer, relax the negative control, or
  broaden package/public API behavior.
- Keep component implementations, Nucleus receipts/manifest/ledger, releases,
  workflows, and sibling repositories out of scope.
- Because `packages/gpui/preview/Cargo.lock` and the receipt emitter's compiled
  lock digest are evidence identity, finish in two commits: C1 contains the
  dependency repair plus updated compiled lock digest; run the full native
  receipt command from C1. C2 may change only the manifest, the complete
  receipt cohort, generated ledger, this card, and one execution log to record
  the freshly observed C1 identity. Carried-forward or hash-only receipts are
  invalid.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Failure is real | warm workspace happens to compile | clean consumer reproduces the exact tinyvec diagnostic |
| Fix is repository-owned | edit cached crates.io source | clean checkout succeeds without cache mutation |
| Identity proof survives | remove duplicate-version assertion | positive and negative consumer checks still bite |
| Scope stays narrow | change component or public API | scope review rejects the branch |

## Validation

Run the focused fresh-consumer script, `effigy regressions:native` from C1,
receipt and ledger validation, `effigy ci:native`, `effigy ci:web`, the
relevant Rust board, `effigy docs:check`, and `git diff --check
origin/main...HEAD`. Never run windowed/native-visual or release selectors.

## Continuation

Merge only after exact-head review. MessageCenter preparation may run in
parallel, but its shared receipt finalization waits for this repair on main.
