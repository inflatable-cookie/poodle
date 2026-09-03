# g16.084 — Nucleus AgentQuestion M1 Receipt

Status: preparation-ready
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Text and Button receipts; serial
finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `037-transcript-inline-agent-approval-research.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/agent-question.md`
Handoff: `../../handoffs/20260903-163600-g16-084-nucleus-agent-question-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus
`AgentQuestion`. Pause before shared evidence. After the orchestrator supplies
the latest cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

- Create one stable mounted test through
  `node_compat::AgentQuestion::from_spec(...).into_element()` and the
  element-backed `HeadlessDriver`. Renderer-only construction is not evidence.
- Prove production Text/Button composition, prompt/options/selection/status
  structure, single- and multi-choice behavior if contract-owned, disabled and
  pending inertia, exact callbacks, host-owned rebuilds, token metadata,
  ordering, containment, and duplicate-instance identity.
- Drive mounted pointer and keyboard input. Do not call handlers directly or
  introduce transcript approval orchestration, Nucleus data, persistence, or a
  generic approval API.
- Commit a biting counterexample before any repair. A bounded generalized native
  repair is allowed only when the mounted production proof fails.
- During preparation, do not edit manifest, receipts, ledger, g16 front doors,
  or claim M1 completion. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test, commit
  runtime source, emit the cohort after the terminal assertion, update this card
  and one log, and run full boards.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Dependencies are real | replace Text/Button with raw nodes | metadata or mounted input fails |
| Controlled ownership is real | paint selection without host rebuild | mounted state stays stale |
| Choice semantics are exact | collapse single/multiple/disabled paths | callback/state trace fails |
| Input is mounted | call handler directly | mounted observation is absent |
| Identity is caller-scoped | reuse one runtime id | focus/selection/callbacks cross |
| Geometry is exact | overlap options or escape mount | order/containment fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Preparation: focused AgentQuestion spec/machine/render/backend and named mounted
tests plus `git diff --check`. Finalization adds `effigy regressions:native`,
receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
