# g16.084 — Nucleus AgentQuestion M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/084-nucleus-agent-question-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-163600-g16-084-nucleus-agent-question-receipt.md`
Branch: `feature/g16-084-nucleus-agent-question-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-084-nucleus-agent-question-receipt`
Planning base: `068bb7f3b95340954142a71b896bf70c3b8b5892`
Preparation-accepted head: `992867b9632e620fd6828f1cf5acd4a4bc4c7599`
Finalization base: `10e7c22f9c822d3fb957d5d320f923107dcf215e`
Runtime source: `bb477fb28d0e7618c2f5dadc8c6e2b0a64944d0a`
PR: `#189`

## Outcome

`AgentQuestion` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, Node backend, and mounted GPUI test
platform. The retained
`agent_question_choices_rebuild_the_host_spec_through_mounted_input` fixture
emits only after its terminal backend observation assertion.

All 21 cohort receipts and the manifest pin runtime source
`bb477fb28d0e7618c2f5dadc8c6e2b0a64944d0a`. The generated ledger advances only
AgentQuestion's GPUI mounted-behaviour cell: 21 mounted, 154 missing. M1 does
not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::AgentQuestion::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Production Text and Button composition preserves progress, prompt, option,
  selection, dismissal, accessibility, and exact token metadata.
- Host-owned refusal, single selection, multi-selection, and dismissal rebuild
  the production factory. Pointer and keyboard dispatch preserve exact callback
  and state traces without crossing duplicate instances.
- Positive full extents prove authored option order and non-overlap. Options and
  dismissal stay inside their production question parent; both roots and the
  disabled witness stay inside the real mount.
- The disabled witness is a mounted production Button with a live callback and
  host-state sink. Real pointer dispatch is inert while disabled, and the
  committed counterexample fails when that gate is removed.

## Committed falsifications and repair

The rebased production-path counterexample `2ff10846f` failed on missing native
composition and mounted identity/input seams. Repair `3bf28e34f` routed
AgentQuestion through production Text and Button primitives with caller-scoped
identity and controlled host rebuilds.

Review counterexample `9a4389565` then exposed incomplete full-extent geometry
and an absent-target disabled oracle. Accepted repair `39ea04a1d` added exact
option ordering, production-parent and mount containment, plus a mounted
disabled production Button carrying a live callback sink. Runtime commit
`bb477fb28` adds terminal receipt emission without changing the accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/agentquestion--nucleus-agent-agent-question.json` |
| Component | `AgentQuestion` |
| Scenario | `nucleus.agent.agent-question` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `bb477fb28d0e7618c2f5dadc8c6e2b0a64944d0a` |
| Outcome | `passed` |

## Validation

- Focused AgentQuestion contract, machine, renderer, backend, and named mounted
  tests passed after rebase.
- `effigy regressions:native` — 192 passed; all 21 receipts were then emitted
  from the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- AgentQuestion M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
