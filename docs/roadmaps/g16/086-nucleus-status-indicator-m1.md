# g16.086 — Nucleus StatusIndicator M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Icon receipt; serial finalization
follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/status-indicator.md`
Handoff: `../../handoffs/20260903-180100-g16-086-nucleus-status-indicator-receipt.md`

## Goal

Produce the first named production-path mounted proof and one terminal `M1`
receipt for Nucleus `StatusIndicator` at a committed runtime source.

## Completed

- Runtime source `23b968c4ba18749dab714bc9edf3a584c5fcacb3`
  emits the terminal StatusIndicator receipt from the stable named mounted test.
- All 23 cohort receipts pin that exact runtime source. The generated ledger
  advances only StatusIndicator from missing to mounted: 23 mounted, 152 missing.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through the production `node_compat::StatusIndicator` `IntoElement`
  path and the element-backed `HeadlessDriver`; renderer-only construction is
  not evidence.
- Prove contract-owned status and reason semantics, label and Icon composition,
  exact tone and size tokens, inertness, state rebuilds, geometry, and
  duplicate-instance identity. Keep runtime reason vocabulary contract-owned.
- Drive mounted input only where the contract exposes it. Do not invent an
  activation seam, Nucleus state machine, polling, persistence, or public API.
- Commit a biting counterexample before any repair. A bounded generalized
  native repair is allowed only when the mounted production proof fails.
- The preparation-accepted head
  `13f5f9e8f635131ca0798e08967d432316359bd8` was rebased in full onto
  `509e784c7734d679ebba3c5222bf2e25c9a24ed5`, preserving rebased counterexample
  `9043e3a78` before repair `8809011a5`.
- Shared evidence contains the complete 23-receipt cohort pinned to the runtime
  source. No g16 front door changes in this card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Icon/Text dependencies are real | replace composition with raw nodes | metadata or layout fails |
| Status semantics are exact | collapse statuses or reason posture | Node assertions fail |
| Token posture is exact | substitute nearby tone or size | exact metadata fails |
| Inertness is real | add focus or activation wiring | mounted posture fails |
| Identity is caller-scoped | reuse one runtime id | mounted instances alias |
| Geometry is exact | overlap content or escape mount | order/containment fails |
| Receipt is terminal | fail final rebuild/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Focused StatusIndicator spec, render, mounted, and backend checks passed after
the rebase. Final validation ran `effigy regressions:native`, receipt and ledger
tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, `effigy docs:check`, and `git diff --check`. No windowed or
native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start another receipt card.
