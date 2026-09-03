# g16.084 — Nucleus AgentQuestion M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Text and Button receipts; serial
finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `037-transcript-inline-agent-approval-research.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/agent-question.md`
Handoff: `../../handoffs/20260903-163600-g16-084-nucleus-agent-question-receipt.md`

## Goal

Produce the first named production-path mounted proof for Nucleus
`AgentQuestion`, then emit one terminal `M1` receipt at the exact committed
runtime source.

Completed at runtime source
`bb477fb28d0e7618c2f5dadc8c6e2b0a64944d0a`. The 21-receipt cohort shares that
identity, and the generated ledger advances only AgentQuestion from missing to
mounted. M1 does not infer A1 or V1.

## Fixed Boundary

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
- Positive full extents prove authored option-row order and non-overlap. Every
  option stays inside its production question parent, and both question roots
  plus the disabled witness stay inside the real mount.
- The disabled witness is a mounted production Button with a supplied live
  callback and host-state sink. Real pointer dispatch stays inert only while
  the disabled gate is present.
- Preparation committed both biting counterexamples before their generalized
  native repairs and was independently accepted at
  `992867b9632e620fd6828f1cf5acd4a4bc4c7599`.
- Finalization rebased the complete preparation batch onto
  `10e7c22f9c822d3fb957d5d320f923107dcf215e`, retained the counterexamples and
  accepted proof, then re-emitted all 21 receipts at the committed runtime
  source. No Nucleus data, approval orchestration, persistence, A1, or V1
  entered the proof.

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

Pause for M1 re-review. Merge and g16 front-door closeout remain with the
orchestrator; this card does not start another receipt.
