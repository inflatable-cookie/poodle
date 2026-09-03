# g16.086 — Nucleus StatusIndicator M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/086-nucleus-status-indicator-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-180100-g16-086-nucleus-status-indicator-receipt.md`
Branch: `feature/g16-086-nucleus-status-indicator-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-086-nucleus-status-indicator-receipt`
Planning base: `e48e1fdf0d198f0405af2ca992c301c19cdfa4d9`
Preparation-accepted head: `13f5f9e8f635131ca0798e08967d432316359bd8`
Finalization base: `509e784c7734d679ebba3c5222bf2e25c9a24ed5`
Runtime source: `23b968c4ba18749dab714bc9edf3a584c5fcacb3`
PR: `#192`

## Outcome

`StatusIndicator` now has one validated `M1` receipt through the production
Rust compatibility adapter, shared renderer, Node backend, and mounted GPUI
test platform. The retained
`status_indicator_status_reason_tokens_and_identity_rebuild_through_mounted_backend`
fixture emits only after its terminal backend observation assertion.

All 23 cohort receipts and the manifest pin runtime source
`23b968c4ba18749dab714bc9edf3a584c5fcacb3`. The generated ledger advances only
StatusIndicator's GPUI mounted-behaviour cell: 23 mounted, 152 missing. M1 does
not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::StatusIndicator::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Production Icon dot and Text label composition preserve exact status, reason,
  tone, size, density, typography, glow, and wrap metadata.
- Host-owned rebuilds cover all six statuses, five sizes, and three densities
  while an identical duplicate remains unchanged.
- Mounted pointer dispatch proves inertness; roots and descendants stay outside
  the focus chain and cannot mutate host-owned state.
- Positive geometry proves authored order, parent and mount containment, and
  caller-scoped duplicate isolation.

## Committed falsification and repair

The rebased production-path counterexample `9043e3a78` failed on the missing
caller-scoped StatusIndicator `IntoElement` facade. Repair `8809011a5` routed
the dot and label through production Icon and Text primitives, added scoped
mounted identity, exposed exact painted text size, and retained the named proof
inside the native regression selector. Runtime commit `23b968c4b` adds terminal
receipt emission without changing the accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/statusindicator--nucleus-agent-status-indicator.json` |
| Component | `StatusIndicator` |
| Scenario | `nucleus.agent.status-indicator` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `23b968c4ba18749dab714bc9edf3a584c5fcacb3` |
| Outcome | `passed` |

## Validation

- Focused StatusIndicator contract, renderer, named mounted, and backend tests
  passed after rebase.
- `effigy regressions:native` — 194 passed; all 23 receipts were then emitted
  from the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- StatusIndicator M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
