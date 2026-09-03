# g16.090 — Nucleus CommandPalette M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/090-nucleus-command-palette-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-210100-g16-090-nucleus-command-palette-receipt.md`
Branch: `feature/g16-090-nucleus-command-palette-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-090-nucleus-command-palette-receipt`
Planning base: `ca5b62ea97e951df3402d41a30cdbee0567b9b97`
Preparation-accepted head: `4ef7b81d890fe3f142153f8c6a215067f74fcf3f`
Finalization base: `4a8dd5018da6feb30f5905a5da195cda03640fc5`
Callout closeout ancestor: `52409b89c`
Runtime source: `cd2f2e13888dba5dcd507cb65e7f63a03e840f03`
PR: `#195`

## Outcome

`CommandPalette` now has one validated `M1` receipt through the production
Rust compatibility adapter, shared renderer, Dialog/TextInput and
ActionDiscoveryPanel composition, Node backend, and mounted GPUI test
platform. The retained
`command_palette_composition_navigation_dismissal_and_identity_rebuild_the_host_spec`
fixture emits only after its terminal controlled-state, composition, token,
geometry, duplicate-instance, and backend-channel assertions.

All 26 cohort receipts and the manifest pin runtime source
`cd2f2e13888dba5dcd507cb65e7f63a03e840f03`. The generated Nucleus ledger
advances only CommandPalette: 26/29 mounted. The full evidence ledger records
26 mounted and 149 missing GPUI behaviour cells. M1 does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::CommandPalette::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Dialog and TextInput dependency metadata survives paint. Results delegate to
  ActionDiscoveryPanel-owned ready, loading, empty, and no-results composition.
- Loading contains five two-Skeleton rows. Empty and no-results retain distinct
  production EmptyState anatomy and copy. Ready results retain group/Eyebrow,
  list, ListCard-equivalent row, and trailing badge/shortcut chip structure.
- Active rows use the exact 18% accent fill and 22% inset ring. Badges use the
  16% accent chip with uppercase label treatment. Shortcuts use the 76% surface
  chip with monospace treatment.
- Mounted navigation proposes an exact active id. A refused unchanged host
  rebuild leaves the old row selected and roving, and Enter activates the old
  id. A later accepted proposal rebuilds to the new selection. Disabled skip
  and wrap remain exact.
- Query edits, enabled and disabled pointer activation, close button, backdrop,
  and Escape travel through mounted input and controlled host rebuilds.
  Positive bounds prove ordering and containment. Duplicate palettes isolate
  focus, callbacks, controlled state, runtime identity, and geometry.

## Committed falsification and repair

Range-diff preserves the complete accepted preparation series across the
rebase onto `4a8dd5018da6feb30f5905a5da195cda03640fc5`:

- `39ca354d5` maps exactly to `9c73d897a`. Its production-identity
  counterexample asked the mounted backend for caller-scoped overlay identity,
  which was absent before repair `40f1754bb`, now `fe1bd8278`. The repair maps
  with only the upstream-relocated deletion of its temporary red-driver line;
  the accepted final tree is unchanged.
- `586801be9` maps exactly to `52566cbda`. It failed first because the
  hand-built results tree had no `action-discovery-panel` dependency. Repair
  `4ef7b81d8`, now `e0a17acf2`, delegates production discovery composition and
  proves the controlled active-selection refusal path.

Runtime commit `cd2f2e138` adds terminal receipt emission without changing the
accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/commandpalette--nucleus-attention-command-palette.json` |
| Component | `CommandPalette` |
| Scenario | `nucleus.attention.command-palette` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `cd2f2e13888dba5dcd507cb65e7f63a03e840f03` |
| Outcome | `passed` |

## Validation

- Focused CommandPalette contract, machine, render, adapter/backend, and exact
  mounted checks passed after rebase.
- `effigy regressions:native` — 196 passed; all 26 receipts were emitted from
  the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated after generation.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- CommandPalette M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
