# g16.088 — Nucleus ConfirmAction M1 Receipt

Status: complete
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Dialog and Button receipts; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/confirm-action.md`
Handoff: `../../handoffs/20260903-194600-g16-088-nucleus-confirm-action-receipt.md`

## Goal

Produce the first named production-path mounted proof and one terminal `M1`
receipt for Nucleus `ConfirmAction` at a committed runtime source.

## Completed

- Runtime source `26a8c3a9dbfbe1649752d818ed4db7d42c23f1d4`
  emits the terminal ConfirmAction receipt from the stable named mounted test.
- All 24 cohort receipts pin that exact runtime source. The generated Nucleus
  ledger advances only ConfirmAction from missing to mounted: 24/29 mounted.
  The full evidence ledger records 24 mounted and 151 missing GPUI behaviour
  cells.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through the production `node_compat::ConfirmAction` `IntoElement` path and element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Dialog/Button composition, title/body/action semantics, destructive and ordinary posture, confirm/cancel/dismiss axes, disabled/pending inertia, host-owned acceptance/refusal rebuilds, exact callback order, token metadata, geometry, focus identity, and duplicate-instance isolation within M1 scope.
- Drive mounted pointer and keyboard input. Do not call handlers directly, claim A1 focus trapping/restoration, or invent application approval policy.
- Preserve both biting counterexample sequences before their bounded repairs.
  The accepted preparation head
  `7684e8a17779754770b5e3ef5215f36f3a2a679c` was rebased in full onto
  `1850811ee79b3a62a5e1e6fe2dba5aa5a14c72eb`. Range-diff maps
  `03feddfb5` to `0ebc8e722` before repair `43aa37fd4`, and `5ad6605cc` to
  `f0735387c` before repair `8b0d0c1fd`.
- Shared evidence contains the complete 24-receipt cohort pinned to the runtime
  source. No g16 front door changes belong to this card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Dialog/Button dependencies are real | replace composition with raw nodes | metadata or input fails |
| Dismissal axes stay separate | couple cancel, backdrop and Escape | exact trace fails |
| Controlled ownership is real | close without accepted host rebuild | refusal proof fails |
| Input is mounted | call handler directly | mounted trace is absent |
| Disabled/pending paths are inert | let confirm commit | callback/state trace fails |
| Identity is caller-scoped | reuse one runtime id | focus/callbacks cross |
| Geometry is exact | overlap actions or escape mount | order/containment fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Focused ConfirmAction contract, render, adapter, backend, and named mounted
checks passed after the rebase. Final validation ran `effigy
regressions:native`, receipt and ledger tests, `effigy
check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`, `effigy
docs:check`, and `git diff --check`. No windowed or native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start another receipt card.
