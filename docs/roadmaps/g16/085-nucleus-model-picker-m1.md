# g16.085 — Nucleus ModelPicker M1 Receipt

Status: preparation-ready
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Select receipt; serial finalization
follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/model-picker.md`
Handoff: `../../handoffs/20260903-180000-g16-085-nucleus-model-picker-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus
`ModelPicker`. Pause before shared evidence. After the orchestrator supplies the
latest cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

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
- During preparation, do not edit manifest, receipts, ledger, g16 front doors,
  or claim M1 completion. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test,
  commit runtime source, emit the cohort after the terminal assertion, update
  this card and one log, and run full boards.

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

Preparation: focused ModelPicker spec/machine/render/backend and named mounted
tests plus `git diff --check`. Finalization adds `effigy regressions:native`,
receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
