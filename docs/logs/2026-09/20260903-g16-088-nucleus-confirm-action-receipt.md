# g16.088 — Nucleus ConfirmAction M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/088-nucleus-confirm-action-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-194600-g16-088-nucleus-confirm-action-receipt.md`
Branch: `feature/g16-088-nucleus-confirm-action-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-088-nucleus-confirm-action-receipt`
Planning base: `509e784c7734d679ebba3c5222bf2e25c9a24ed5`
Preparation-accepted head: `7684e8a17779754770b5e3ef5215f36f3a2a679c`
Finalization base: `1850811ee79b3a62a5e1e6fe2dba5aa5a14c72eb`
Runtime source: `26a8c3a9dbfbe1649752d818ed4db7d42c23f1d4`
PR: `#193`

## Outcome

`ConfirmAction` now has one validated `M1` receipt through the production Rust
compatibility adapter, shared renderer, Dialog/Button composition, Node
backend, and mounted GPUI test platform. The retained
`confirm_action_composition_dismissal_inertia_and_identity_rebuild_the_host_spec`
fixture emits only after its terminal controlled-state and callback-isolation
assertions.

All 24 cohort receipts and the manifest pin runtime source
`26a8c3a9dbfbe1649752d818ed4db7d42c23f1d4`. The generated Nucleus ledger
advances only ConfirmAction: 24/29 mounted. The full evidence ledger records 24
mounted and 151 missing GPUI behaviour cells. M1 does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::ConfirmAction::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- Production Dialog and Button composition preserves exact title, description,
  body, actions, alert-dialog role, destructive/default tones, size, density,
  surface, border, radius, and background metadata.
- Confirm, cancel button, backdrop, and Escape run as separate mounted inputs.
  Each exact trace assertion is followed immediately by proof that host refusal
  leaves the same dialog mounted.
- Pending state makes both actions, backdrop, and Escape inert. Accepted host
  rebuilds independently close the selected duplicate instance.
- Positive 800x600 mount, backdrop, and surface bounds prove exact mount to
  backdrop equality and mount to backdrop to surface containment. Body and
  actions remain contained, ordered, and non-overlapping.
- Caller-scoped identities isolate focus, callback traces, controlled state,
  runtime ids, and geometry across destructive and ordinary instances.

## Committed falsification and repair

Range-diff preserves the complete accepted preparation series across the
rebase onto `1850811ee79b3a62a5e1e6fe2dba5aa5a14c72eb`:

- `03feddfb5` maps exactly to `0ebc8e722`. Its scoped production trigger
  counterexample failed before repair `503249b1f`, now `43aa37fd4`, supplied
  caller identity, controlled trigger/working state, production composition,
  and the full mounted proof.
- `5ad6605cc` maps exactly to `f0735387c`. It failed first because the mounted
  scoped backdrop was absent. An isolated diagnostic past that stop exposed a
  second red: Escape appended four dismiss callbacks where three were expected.
  Repair `7684e8a17`, now `8b0d0c1fd`, records the real backdrop paint box,
  supplies the mount positioning context, chooses a mounted outside-surface
  backdrop point, and removes the duplicate native-wrapper Escape listener.

Runtime commit `26a8c3a9d` adds terminal receipt emission without changing the
accepted proof.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/confirmaction--nucleus-settings-confirm-action.json` |
| Component | `ConfirmAction` |
| Scenario | `nucleus.settings.confirm-action` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `26a8c3a9dbfbe1649752d818ed4db7d42c23f1d4` |
| Outcome | `passed` |

## Validation

- Focused ConfirmAction contract, render, adapter, backend, and exact mounted
  checks passed after rebase.
- `effigy regressions:native` — 195 passed; all 24 receipts were emitted from
  the same test target at the runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated after generation.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- ConfirmAction M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
