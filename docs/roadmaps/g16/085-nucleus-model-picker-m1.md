# g16.085 — Nucleus ModelPicker M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Select receipt; serial finalization
follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/model-picker.md`
Handoff: `../../handoffs/20260903-180000-g16-085-nucleus-model-picker-receipt.md`

## Goal

Produce the first named production-path mounted proof and one terminal `M1`
receipt for Nucleus `ModelPicker` at a committed runtime source.

## Completed

- Runtime source `3bbbd0f51359bf562b8228b1f3fe27cd9853e18f`
  emits the terminal ModelPicker receipt from the stable named mounted test.
- All 22 cohort receipts pin that exact runtime source. The generated ledger
  advances only ModelPicker from missing to mounted: 22 mounted, 153 missing.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through the production `node_compat::ModelPicker` `IntoElement` path
  and the element-backed `HeadlessDriver`; renderer-only construction is not
  evidence.
- Prove the contract-owned trigger, selected and unavailable states, provider
  and model labels, filtering or selection behavior where supported, exact
  callback order, host-owned rebuilds and refusal, Select dependency metadata,
  token posture, geometry, and duplicate-instance isolation.
- Drive mounted pointer and keyboard input. Do not call handlers directly,
  introduce provider policy or persistence, or broaden the public API.
- Commit a biting counterexample before any repair. A bounded generalized
  native repair is allowed only when the mounted production proof fails.
- The preparation-accepted head
  `fe40f06f80010e3eafb3f53369ecff35dcaeb6c1` was rebased in full onto
  `231939cb3174ac1c76e6ad3eea66e723509e886d`, preserving both committed
  counterexamples before their repairs.
- The production dialog keeps resolved overlay elevation on its one real
  dialog Surface. Its child is a neutral Panel, not a second dialog.
- Shared evidence contains the complete 22-receipt cohort pinned to the
  runtime source. No g16 front door changes in this card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Select dependency is real | replace composed Select with raw nodes | metadata or mounted input fails |
| Controlled ownership is real | paint accepted selection without rebuild | mounted state stays stale |
| Availability is exact | permit disabled or unavailable model | callback/state trace fails |
| Input is mounted | call handler directly | mounted observation is absent |
| Identity is caller-scoped | reuse one runtime id | focus/selection/callbacks cross |
| Geometry is exact | overlap rows or escape mount | order/containment fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Focused ModelPicker spec, render, mounted, and backend checks passed after the
rebase. Final validation ran `effigy regressions:native`, receipt and ledger
tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, `effigy docs:check`, and `git diff --check`. No windowed or
native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start another receipt card.
