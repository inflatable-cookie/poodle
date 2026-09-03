# g16.081 — Nucleus AgentPlan M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Button and Text receipts; serial
finalization follows `g16.080`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/agent-plan.md`
Handoff: `../../handoffs/20260903-133000-g16-081-nucleus-agent-plan-receipt.md`

## Goal

Produce one biting production-path mounted proof for Nucleus `AgentPlan` using
the retained `agent_plan_decisions_rebuild_the_host_spec_through_mounted_input`
fixture, then emit one terminal `M1` receipt at the exact committed runtime
source.

Completed at runtime source
`9caec51bf40c9e3d21af8c92475bc069460dbc5c`. The 18-receipt cohort shares that
identity, and the generated ledger advances only AgentPlan from missing to
mounted. M1 does not infer A1 or V1.

## Fixed Boundary

- Mount `node_compat::AgentPlan::from_spec(...).into_element()` through the
  element-backed `HeadlessDriver`; renderer-only Node construction is not
  adapter evidence.
- Prove production plan structure, heading/description/status, records,
  decision affordances, token metadata, mounted ordering, containment, and
  caller-scoped duplicate-instance identity.
- Drive mounted pointer and keyboard decision input. Host state owns accepted,
  rejected, and pending outcomes and must rebuild before painted state changes.
  Refusal must remain stable and duplicate instances must not cross callbacks.
- Use production Text and Button composition. Do not introduce Nucleus data,
  transcript orchestration, markdown policy, persistence, or approval APIs.
- Commit a biting counterexample before any repair. A bounded native repair is
  allowed only when the production mounted proof fails.
- Preparation did not edit the Nucleus manifest, receipts, generated ledger,
  or g16 front doors and stopped at a draft PR.
- Finalization rebases onto the latest receipt merge, revalidates, commits the
  runtime source, emits the cohort, updates this card and one log, then runs the
  full receipt boards. Merge remains orchestrator-owned.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle assertion fails |
| Dependencies are real | replace Text or Button with raw nodes | production metadata or input proof fails |
| Controlled ownership is real | paint callback result without host rebuild | mounted state remains stale |
| Decision paths differ | collapse accept/reject/pending/refusal | exact callback and rebuilt state trace fails |
| Input is mounted | invoke a handler directly | mounted observation is absent |
| Identity is caller-scoped | reuse one runtime id | focus or callbacks cross instances |
| Geometry is exact | overlap rows or escape the mount box | ordering/containment assertion fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before the latest predecessor merge | cohort validation fails |
| Levels stay separate | label M1 as A1/V1 | schema validation fails |

## Validation

Preparation: focused AgentPlan spec/machine/render/backend and named mounted
tests plus `git diff --check`. Finalization adds `effigy regressions:native`,
receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

After review and merge, the orchestrator may authorize the next serial Nucleus
receipt. This card does not start g16.082 finalization.
