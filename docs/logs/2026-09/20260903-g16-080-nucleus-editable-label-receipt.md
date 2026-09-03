# g16.080 — Nucleus EditableLabel M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/080-nucleus-editable-label-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-130000-g16-080-nucleus-editable-label-receipt.md`
Branch: `feature/g16-080-nucleus-editable-label-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-080-nucleus-editable-label-receipt`
Planning base: `c8c59002322f9891f723902eef3c172ada75274e` (`origin/main`)

## Outcome

`EditableLabel` now has one validated `M1` execution receipt through the
production Rust adapter, renderer, Node backend, and GPUI test platform. The
retained `editable_label_live_draft_stays_off_the_committed_value` regression
was strengthened rather than duplicated.

The fixture mounts
`node_compat::EditableLabel::from_spec(...).into_element()` through the
element-backed `HeadlessDriver`. It proves host-owned committed state remains
separate from the session draft; exact double-click, Enter, Space, printable,
Escape, Tab, blur, disabled, and programmatic input boundaries; Unicode-scalar
`maxLength`; portable trim set T including NEL and BOM while preserving ZWSP;
unchanged trimmed commit; Enter/Escape focus restoration; Tab/blur departure;
equal-valued caller-id isolation; silent focused teardown; display/input roles,
accessible-name fallback, typography, field treatment, positive bounds,
containment, and non-overlap. Receipt emission follows the terminal teardown
assertion.

All 17 receipts and the manifest pin runtime source
`313a1f56bcc1041d1aa8a939e95d5422525c9392`. The generated ledger advances only
EditableLabel's GPUI mounted-behaviour cell: 17 mounted, 158 missing. M1 does
not infer A1 or V1.

## What landed

- `packages/gpui/preview/tests/headless_regressions.rs`: mounted controlled
  host, production adapter element path, input/focus/geometry/identity oracles,
  and terminal receipt emission.
- `packages/gpui/preview/src/node_compat.rs`: crate-local clone and
  `IntoCompatNode` support so the fixture observes the exact Node consumed by
  the adapter-backed element.
- `packages/render/src/editable_label.rs`: display `Button` and editing
  `TextInput` roles, text/not-allowed cursor projection, display tab stop, and
  focus-ring metadata.
- `packages/contracts/components/src/editable_label.rs`: focused spec tests for
  defaults, draft/value separation, name fallback, Unicode-scalar selection,
  and semantic token projection.
- Receipt cohort, manifest, generated evidence ledger, this card, and this log.

## Focused repair

Committed counterexample `efe896169a142eb259e7c5361ef5f875800f09af`
mounted the production adapter and failed before receipt emission:

```text
assertion `left == right` failed
  left: None
 right: Some(Button)
```

Repair `29f7c47b0c0c179fd6147b13cc58047780847425` projects the documented native
semantics and focus treatment from `poodle_render::editable_label`. No public
API, live draft callback, host persistence, browser behavior, Nucleus data, or
IME work changed. The Rust contract has no EditableLabel read-only input; this
card did not invent one. Disabled and programmatic activation paths are inert.

## Review oracle falsification

| Invariant | Smallest counterexample | Observed proof |
| --- | --- | --- |
| Production adapter owns execution | mount the renderer Node directly | adapter element mount, adapter-stamped ids, bounds, focus handles, and dispatch trace are required |
| Draft and committed value stay separate | paint the committed value in edit mode | mounted input paints `Kicks` while host value remains `Kick` until Enter |
| Input is mounted | call a transition or callback directly | GPUI test-platform dispatch and mounted observation are required for receipt emission |
| Activation routes are exact | accept one click in double-click mode or activate programmatic | phase and callback assertions fail |
| Commit and cancel differ | commit Escape or retain draft after cancel | rebuilt value, draft, previous-value, and callback trace fail |
| Focus departure is exact | restore after Tab/blur or lose display focus after Enter/Escape | active focus-handle assertions fail |
| Trim law is exact | use Rust `str::trim` | NEL/BOM assertion fails; ZWSP preservation catches over-trim |
| Scalar length is exact | count UTF-8 or UTF-16 units | one astral scalar is rejected or a second scalar is accepted |
| Teardown is silent | commit or cancel on focused unmount | terminal callback count fails and no receipt is emitted |
| Identity is caller-scoped | reuse one runtime id for equal values | focus, draft, callbacks, or bounds cross instances |
| Geometry is exact | overlap fields or escape the mount box | positive, ordering, and containment assertions fail |
| Semantics are exact | omit display Button or editing TextInput role | committed red proof fails before receipt emission |
| Evidence identity is exact | retain the g16.079 SHA | receipt cohort validation rejects the source mismatch |
| Levels stay separate | label M1 as A1 or V1 | receipt schema test rejects the proof level |

## Validation

Focused:

- EditableLabel spec: 2 passed.
- Edit machine: 2 passed.
- EditableLabel renderer: 4 passed.
- Node backend painted text identity: 2 passed.
- GPUI adapter EditableLabel: 2 passed.
- Named mounted fixture: 1 passed.

Required boards:

- `effigy regressions:native` — 187 passed; all 17 receipts emitted at runtime
  `313a1f56bcc1041d1aa8a939e95d5422525c9392`.
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed.
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.
- `effigy docs:check` — clean.
- `git diff --check origin/main...HEAD` — clean.

## Limits

- EditableLabel M1 only. No other Nucleus row advances.
- No A1, V1, Jetstream admission, workflow, release, version, web, or visual
  claim.
- No windowed or native-visual selector ran.
