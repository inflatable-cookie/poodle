# g16.081 — Nucleus AgentPlan M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/081-nucleus-agent-plan-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-133000-g16-081-nucleus-agent-plan-receipt.md`
Branch: `feature/g16-081-nucleus-agent-plan-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-081-nucleus-agent-plan-receipt`
Planning base: `a5266ec41198e0d432f9dd4453c70c635979c1dc` (`origin/main`)
Preparation-accepted head: `416b94cbc408f8954ddb8d7b7c410f7b2b948884`
Runtime source: `9caec51bf40c9e3d21af8c92475bc069460dbc5c`
PR: `#187`

## Outcome

`AgentPlan` now has one validated `M1` execution receipt through the production
Rust compat adapter, shared renderer, Node backend, and GPUI test platform. The
retained `agent_plan_decisions_rebuild_the_host_spec_through_mounted_input`
fixture mounts two caller-scoped AgentPlan instances through
`node_compat::AgentPlan::from_spec(...).into_element()` and emits only after its
terminal state, focus, bounds, probe-channel, and mounted-observation checks.

All 18 receipts and the manifest pin runtime source
`9caec51bf40c9e3d21af8c92475bc069460dbc5c`. The generated ledger advances only
AgentPlan's GPUI mounted-behaviour cell: 18 mounted, 157 missing. M1 does not
infer A1 or V1.

## Production proof

- Pending and settled plans retain their heading, description, list records,
  status, decision order, and production Text/Button composition.
- Primary, secondary, and ghost decisions resolve exact AgentPlan-owned idle,
  hover, focus, border, text, radius, and spacing tokens while Button retains
  semantics, activation, and variant roles.
- Pointer and keyboard decisions rebuild host-owned accepted, dismissed, and
  revised states. A refused acceptance remains pending.
- Equal-content caller-scoped instances keep focus, callbacks, state, runtime
  ids, and bounds separate.
- Mounted roots and actions have positive ordered bounds and remain contained
  in their own plan.
- Backend probes observe Button structure, Text content, semantic tokens, and
  accessibility projection before terminal receipt emission.

## Focused repair

The committed preparation counterexample failed on the production accept
Button hover fill before the repair:

```text
left:  ColorValue(0.80413985, 0.6029071, 0.27319318, 1.0)
right: ColorValue(1.0, 0.7921569, 0.4509804, 1.0)
```

The left value was the general Button primary hover mix. The right value is
AgentPlan's contract-owned `color.accent.hover` token in ECLIPSE (`#ffca73`). A
crate-private Button visual recipe now lets composites replace exact resting
and hover visuals without changing Button semantics, focus, activation,
variant roles, or public API.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/agentplan--nucleus-agent-agent-plan.json` |
| Component | `AgentPlan` |
| Scenario | `nucleus.agent.agent-plan` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `9caec51bf40c9e3d21af8c92475bc069460dbc5c` |
| Outcome | `passed` |

## Validation

Focused:

- AgentPlan machine conformance: 3 passed.
- AgentPlan spec: 3 passed.
- AgentPlan renderer: 1 passed.
- Button renderer: 49 passed.
- GPUI Node backend: 51 passed.
- Named mounted AgentPlan fixture: 1 passed.

Required boards:

- `effigy regressions:native` — 187 passed; all 18 receipts emitted at the
  runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check` — clean.

## Limits

- AgentPlan M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus or web implementation, g16 front-door, workflow, release,
  version, or native-visual change.
- No windowed or native-visual selector ran.
- g16.082 finalization did not start.
