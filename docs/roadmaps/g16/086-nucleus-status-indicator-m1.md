# g16.086 — Nucleus StatusIndicator M1 Receipt

Status: preparation-ready
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Icon receipt; serial finalization
follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/status-indicator.md`
Handoff: `../../handoffs/20260903-180100-g16-086-nucleus-status-indicator-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus
`StatusIndicator`. Pause before shared evidence. After the orchestrator supplies
the latest cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

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
- During preparation, do not edit manifest, receipts, ledger, g16 front doors,
  or claim M1 completion. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test,
  commit runtime source, emit the cohort after the terminal assertion, update
  this card and one log, and run full boards.

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

Preparation: focused StatusIndicator spec/render/backend and named mounted tests
plus `git diff --check`. Finalization adds `effigy regressions:native`,
receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
