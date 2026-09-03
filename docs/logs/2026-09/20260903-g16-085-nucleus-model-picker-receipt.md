# g16.085 — Nucleus ModelPicker M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/085-nucleus-model-picker-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-180000-g16-085-nucleus-model-picker-receipt.md`
Branch: `feature/g16-085-nucleus-model-picker-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-085-nucleus-model-picker-receipt`
Planning base: `e48e1fdf0d198f0405af2ca992c301c19cdfa4d9`
Preparation-accepted head: `fe40f06f80010e3eafb3f53369ecff35dcaeb6c1`
Finalization base: `231939cb3174ac1c76e6ad3eea66e723509e886d`
Runtime source: `3bbbd0f51359bf562b8228b1f3fe27cd9853e18f`
PR: `#191`

## Outcome

`ModelPicker` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, Node backend, and mounted GPUI test
platform. The retained
`model_picker_selection_and_identity_rebuild_through_mounted_input` fixture
emits only after its terminal backend observation assertion.

All 22 cohort receipts and the manifest pin runtime source
`3bbbd0f51359bf562b8228b1f3fe27cd9853e18f`. The generated ledger advances only
ModelPicker's GPUI mounted-behaviour cell: 22 mounted, 153 missing. M1 does not
infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::ModelPicker::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Production Select, SegmentedControl, Switch, labels, states, capability axes,
  dependency metadata, and tokens survive the mounted path.
- Pointer and keyboard dispatch cover open, Escape, accepted selection,
  unavailable inertness, host refusal, and controlled rebuilds with exact
  callback and state traces across duplicate instances.
- Positive geometry proves authored order, non-overlap, parent containment,
  and real-mount containment under caller-scoped identities.
- The real dialog Surface owns the resolved overlay elevation and is the only
  dialog node. Its neutral Panel child cannot acquire a second dialog role.

## Committed falsifications and repair

The rebased production-path counterexample `1306c3660` failed on missing native
composition and mounted identity/input seams. Repair `a92ccce6a` routed
ModelPicker through production Select, SegmentedControl, and Switch primitives
with caller-scoped identity and controlled host rebuilds.

Review counterexample `4fdad1d02` observed missing overlay elevation and two
nested dialogs. Accepted repair `b16bbe6c5` restored the resolved overlay
elevation on one dialog Surface and kept its Panel child neutral. Runtime commit
`3bbbd0f51` adds terminal receipt emission without changing the accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/modelpicker--nucleus-agent-model-picker.json` |
| Component | `ModelPicker` |
| Scenario | `nucleus.agent.model-picker` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `3bbbd0f51359bf562b8228b1f3fe27cd9853e18f` |
| Outcome | `passed` |

## Validation

- Focused ModelPicker contract, renderer, named mounted, and backend tests
  passed after rebase.
- `effigy regressions:native` — 193 passed; all 22 receipts were then emitted
  from the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- ModelPicker M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
