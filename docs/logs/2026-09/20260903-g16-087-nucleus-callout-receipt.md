# g16.087 — Nucleus Callout M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/087-nucleus-callout-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-194500-g16-087-nucleus-callout-receipt.md`
Branch: `feature/g16-087-nucleus-callout-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-087-nucleus-callout-receipt`
Planning base: `509e784c7734d679ebba3c5222bf2e25c9a24ed5`
Preparation-accepted head: `b0fc557373d83dd9a1b6875f9c54c7f5cc670eed`
Finalization base: `ca5b62ea97e951df3402d41a30cdbee0567b9b97`
ConfirmAction closeout ancestor: `019f4dd1d`
Runtime source: `2d0e963c0b082c258213749a6d8d452339c7c01b`
PR: `#194`

## Outcome

`Callout` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, Icon/Button composition, Node backend,
and mounted GPUI test platform. The retained
`callout_dismiss_rebuilds_the_host_spec_through_mounted_input` fixture emits
only after its terminal controlled-state, callback-isolation, geometry, and
backend-channel assertions.

All 25 cohort receipts and the manifest pin runtime source
`2d0e963c0b082c258213749a6d8d452339c7c01b`. The generated Nucleus ledger
advances only Callout: 25/29 mounted. The full evidence ledger records 25
mounted and 150 missing GPUI behaviour cells. M1 does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::Callout::from_spec(...).into_element()` instances
  reach `HeadlessDriver` through the production adapter, renderer, Node
  backend, and GPUI element path.
- The production structure preserves exact body, actions, and dismiss order
  and part metadata. The status glyph and dismiss glyph use real Icon nodes.
  The actions region contains one live Button and one disabled Button.
- Mounted pointer input invokes the live action exactly once. The disabled
  action remains inert. Pointer dismissal is refused by the host before a
  keyboard dismissal is accepted through a controlled rebuild.
- `ControlSize::Sm` resolves the exact 8 px root gap, 9 px status icon, 12 px
  title, 11 px message, and 24 by 24 px dismiss geometry. The proof also locks
  tone, fill, announcement, label, density, padding, border, radius, and color
  token metadata.
- Positive 560x360 mounted bounds prove containment, internal part ordering,
  non-overlap, and duplicate-instance separation. Caller-scoped identities
  isolate focus, callbacks, controlled visibility, runtime ids, and geometry.
- The receipt makes no Spinner dependency claim, so pending Spinner composition
  is outside this proof.

## Committed falsification and repair

Range-diff preserves the complete accepted preparation series across the
rebase onto `ca5b62ea97e951df3402d41a30cdbee0567b9b97`:

- `e87a85f81` maps exactly to `fe00d70e4`. Its production-identity
  counterexample asked the mounted backend for `callout:counterexample`, which
  was absent before repair `3238febba`, now `0a346a14b`. The repair supplied
  caller-scoped identity and the production adapter/backend mounted proof. The
  range-diff marks the repair changed only because upstream context relocated
  deletion of the temporary counterexample lines; the repair scope and final
  tree remain intact.
- `2e10a0048` maps exactly to `e9ccbf295`. It failed first because
  `node_compat::Callout.with_actions` did not exist. The same tree still used a
  12 px root gap, icon, title, and message plus 28 px dismiss geometry. Repair
  `b0fc55737`, now `c6d751023`, routed production-context actions through the
  renderer and adapter and fixed the exact 8/9/12/11/24 small-size ladder.

Runtime commit `2d0e963c0` adds terminal receipt emission without changing the
accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/callout--nucleus-settings-callout.json` |
| Component | `Callout` |
| Scenario | `nucleus.settings.callout` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `2d0e963c0b082c258213749a6d8d452339c7c01b` |
| Outcome | `passed` |

## Validation

- Focused Callout contract, render, adapter, backend, and exact mounted checks
  passed after rebase.
- `effigy regressions:native` — 195 passed; all 25 receipts were emitted from
  the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated after generation.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- Callout M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
