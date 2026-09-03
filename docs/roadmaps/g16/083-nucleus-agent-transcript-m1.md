# g16.083 — Nucleus AgentTranscript M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Text and Surface receipts; serial
finalization follows `g16.082`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/agent-transcript.md`
Handoff: `../../handoffs/20260903-163500-g16-083-nucleus-agent-transcript-receipt.md`

## Goal

Produce the first named production-path mounted proof for Nucleus
`AgentTranscript`, then emit one terminal `M1` receipt at the exact committed
runtime source.

Completed at runtime source
`2d14d7e6afa25976cd475afb6b60542027b47216`. The 20-receipt cohort shares that
identity, and the generated ledger advances only AgentTranscript from missing
to mounted. M1 does not infer A1 or V1.

## Fixed Boundary

- The manifest names only the retained
  `agent_transcript_records_rebuild_through_production_mounted_input` fixture
  for this card.
- Mount
  `node_compat::AgentTranscript::from_spec(...).into_element()` and the
  element-backed `HeadlessDriver`. Renderer-only construction is not evidence.
- Prove real production Text, Surface, EmptyState, and Spinner composition;
  ordered records and statuses; exact elevated user-message fill and radius
  without shadow; bounded scrolling; mount containment; and caller-scoped
  duplicate identity.
- Host state owns records, disclosure, and appended content. Keyboard, pointer,
  wheel, and jump-control input pass through the mounted GPUI test platform and
  production rebuild factory.
- Empty and loading structures are mounted and bounded through the production
  path. The receipt emits only after their terminal structural assertion.
- Preparation committed both biting counterexamples before their generalized
  native repairs and was independently accepted at
  `61c5cc7c909ea8cfce44c044fbb6559b303dc44a`.
- Finalization rebased the complete preparation batch onto `dec442579`, then
  re-emitted all 20 receipts at the committed runtime source. No Nucleus data,
  streaming orchestration, markdown policy, citations, approval policy, or
  persistence entered the proof.

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

Pause for M1 re-review. Merge and g16 front-door closeout remain with the
orchestrator; this card does not start another receipt.
