# g16.083 — Nucleus AgentTranscript M1 Receipt

Status: preparation-ready
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Text and Surface receipts; serial
finalization follows `g16.082`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/agent-transcript.md`
Handoff: `../../handoffs/20260903-163500-g16-083-nucleus-agent-transcript-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus
`AgentTranscript`. Pause before shared evidence. After the orchestrator supplies
the latest merged cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

- Create one stable mounted test through
  `node_compat::AgentTranscript::from_spec(...).into_element()` and the
  element-backed `HeadlessDriver`. Renderer-only construction is not evidence.
- Prove production Text/Surface composition, ordered message/record structure,
  roles/status metadata, empty/loading/error/content posture, token metadata,
  scrolling or bounded overflow where contract-owned, mount containment, and
  duplicate-instance identity.
- Drive only real mounted input exposed by the contract. Host state owns records
  and rebuilds; do not import Nucleus data, streaming orchestration, markdown
  policy, citations, approval policy, or persistence.
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
| Dependencies are real | replace Text/Surface with raw nodes | exact metadata or layout fails |
| Host ownership is real | mutate records without rebuild | mounted order/content stays stale |
| Postures differ | collapse empty/loading/error/content | exact structure/status fails |
| Identity is caller-scoped | reuse one runtime id | focus/scroll/callback state crosses |
| Geometry is exact | reorder rows, overlap, or escape mount | order/containment fails |
| Receipt is terminal | fail final posture/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Preparation: focused AgentTranscript spec/render/backend and named mounted tests
plus `git diff --check`. Finalization adds `effigy regressions:native`, receipt/
ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
