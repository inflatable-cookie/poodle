# g16.089 — Nucleus DetailItem M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/089-nucleus-detail-item-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-210000-g16-089-nucleus-detail-item-receipt.md`
Branch: `feature/g16-089-nucleus-detail-item-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-089-nucleus-detail-item-receipt`
Planning base: `52409b89c09a1544f3432bb9fefa51e9dabe5967`
Preparation-accepted head: `03d2a6ccbe91fcca3f63ffb2f78dbc3f5fd6ca84`
Finalization base: `1add36a7a4e9d77831b978b833e8ffe28c3a33dc`
CommandPalette closeout ancestor: `1add36a7a4e9d77831b978b833e8ffe28c3a33dc`
Rebased preparation head: `532ad4df5d76b788776709cfaf1a4184de7b6623`
Runtime source: `91bcb4367068d31d9be5a6844baf2b2b7f51baa7`
PR: `#196`

## Outcome

`DetailItem` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, real Text and Button composition, Node
backend, and mounted GPUI test platform. The retained
`detail_item_structure_states_actions_and_identity_rebuild_through_mounted_backend`
fixture emits only after its terminal state, token, structure, geometry,
duplicate-instance, and backend-channel assertions.

All 27 cohort receipts and the manifest pin runtime source
`91bcb4367068d31d9be5a6844baf2b2b7f51baa7`. The generated Nucleus ledger
advances only DetailItem: 27/29 mounted. The full evidence ledger records 27
mounted and 148 missing GPUI behaviour cells. M1 does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::DetailItem::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Real Text label, supporting, and value nodes plus a real Button action retain
  their dependency identity, exact parentage, order, colors, typography,
  line-height, weight, truncation, and empty-value state.
- Surface remains the portable default. Surface inline, surface stacked, and
  simple inline preserve exact presentation, background, radius, padding,
  alignment, gap, span, density, and accessibility metadata.
- The surface-stacked recipe resolves compact/default/comfortable root and
  content gaps to exact `2/3/4px` values.
- Mounted pointer input proves the value path inert and the left action live;
  mounted keyboard input activates the duplicate right action. Host rebuilds
  retain controlled ownership.
- Both roots and every child remain contained, ordered, and non-overlapping
  initially and after each rebuild. Duplicate instances isolate focus,
  callbacks, state, runtime identity, and geometry.

## Committed falsification and repair

Range-diff preserves the full accepted preparation series across the rebase
onto `1add36a7a4e9d77831b978b833e8ffe28c3a33dc`:

- Historical identity counterexample `0003edf11` maps exactly to `2e420b7f3`;
  repair `736e2cfb5` maps exactly to `e5377e40e`. Review found that sequence
  insufficient because caller scope was absent from its red arrange/act, so it
  remains history rather than the accepted identity proof.
- Caller-scoped review counterexample `faabf5aa0` maps exactly to `9a3b9ed88`;
  repair `36c1ad16c` maps exactly to `0abaf8f4b`. Its arrange/act supplies
  caller scope before the assertion and the production repair preserves the
  accepted presentation, alignment, typography, parentage, containment, and
  duplicate-root invariants.
- Density counterexample `614a6921e` maps exactly to `aa186e124`; repair
  `03d2a6ccb` maps exactly to `532ad4df5`. Compact observed `(4, 4)` against
  expected `(2, 2)` before the production recipe became density-aware for
  compact/default/comfortable `(2, 2)/(3, 3)/(4, 4)`.

Runtime commit `91bcb4367` adds terminal receipt emission without changing the
accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/detailitem--nucleus-settings-detail-item.json` |
| Component | `DetailItem` |
| Scenario | `nucleus.settings.detail-item` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `91bcb4367068d31d9be5a6844baf2b2b7f51baa7` |
| Outcome | `passed` |

## Validation

- Focused DetailItem contract, render, adapter/backend, Jetstream mapping, and
  exact mounted checks passed after rebase.
- `effigy regressions:native` — 198 passed; all 27 receipts were emitted from
  the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated after generation.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- DetailItem M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- ToastHost remains planning-only.
- No windowed or native-visual selector ran.
- No merge and no next card.
