# g16.080 — Nucleus EditableLabel M1 Receipt

Status: complete
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.045`, completed `g16.062`, completed `g16.077`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`045-editable-label-editing-model-and-mounted-parity.md`,
`077-nucleus-text-input-m1.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/editable-label.md`
Handoff: `../../handoffs/20260903-130000-g16-080-nucleus-editable-label-receipt.md`

## Goal

Produce one validated `M1` receipt for Nucleus `EditableLabel` through the
production Rust adapter, renderer, Node, GPUI backend, and mounted input path.
Retain the g16.045 edit-session and focus-restoration contract; do not create a
second editor or move host persistence into Poodle.

## Fixed Boundary

- Keep and strengthen
  `editable_label_live_draft_stays_off_the_committed_value`; do not add a
  duplicate receipt fixture.
- Mount `node_compat::EditableLabel::from_spec(...).into_element()` through the
  element-backed `HeadlessDriver`. Renderer-only Node construction is not
  adapter evidence.
- Host state owns committed value and rebuilds. The adapter owns the live draft,
  selection, and edit session. Prove the painted input follows draft while the
  committed display/value remains unchanged until commit.
- Drive pointer double activation, Enter/Space activation, printable editing,
  scalar `maxLength`, Enter commit, Escape cancel, Tab/blur commit, and teardown
  through mounted production input. Do not call transitions or handlers
  directly after mount.
- Prove Enter/Escape restore focus to the display control; Tab/blur let focus
  leave. Prove unchanged trimmed commit still fires once and teardown emits
  neither commit nor cancel.
- Prove the exact trim set from the contract, including NEL, BOM, and ZWSP
  counterexamples. Do not substitute Rust `str::trim`.
- Prove two equal-valued instances with caller-owned ids keep focus, draft,
  callbacks, and teardown separate. Disabled and read-only behavior remain
  inert according to the contract.
- Assert display/input semantics, visible-name fallback, edit/display
  visibility, typography, field treatment, positive bounds, production mount
  containment, and non-overlap. Do not infer A1 or V1 from Node metadata.
- A focused native repair is allowed only after a committed mounted
  counterexample. Stop for a public API, live draft callback, host persistence,
  browser-only behavior, app validation policy, or OS IME work.
- Emit the receipt only after every claimed assertion. Refresh the manifest,
  all existing receipts, generated ledger, this card, and one execution log
  from the exact committed runtime source. No other row advances.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter lifecycle or identity assertion fails |
| Committed value and draft stay separate | paint committed value while editing | mounted text/value witness fails |
| Input is mounted | call transition or handler directly | mounted observation or callback trace is absent |
| Activation routes are exact | treat click as double activation or omit keyboard entry | phase/focus trace fails |
| Commit and cancel differ | mutate on Escape or retain draft after commit | rebuilt display/value trace fails |
| Focus departure is exact | restore on Tab or lose focus after Enter/Escape | active focus handle fails |
| Trim law is exact | use `str::trim` | NEL/BOM/ZWSP vectors fail |
| Scalar length is exact | count UTF-8 bytes or UTF-16 units | astral-character vector fails |
| Teardown is silent | emit commit/cancel on unmount | terminal callback count fails |
| Identity is caller-scoped | reuse one runtime id | focus, draft, or callbacks cross instances |
| Geometry is exact | overlap instances or escape the production mount box | bounds assertion fails |
| Receipt is terminal | fail final isolation/teardown assertion | no receipt is emitted |
| Evidence identity is exact | retain the g16.079 source SHA | cohort validation fails |
| Levels stay separate | label M1 as A1/V1 | schema validation fails |

## Writable Scope

The retained EditableLabel mounted regression; focused EditableLabel spec,
machine, renderer, backend, and GPUI adapter tests; one bounded native repair
after a committed counterexample; receipt/manifest/ledger refresh; this card;
one log; and new papercuts. Do not edit Nucleus, web behavior, visual-lab code,
Jetstream, workflows, versions, releases, or other component rows.

## Validation

PR #186 review revision closes the activation and typography counterexamples:
default single-click is mounted and inert before a distinct default
double-click entry; `enterOrSpace` enters through separate mounted Enter and
Space paths; display and input Nodes carry label size, weight, relative
line-height, and explicit sans-family inheritance metadata; empty display text
carries italics.

Run focused EditableLabel spec/machine/render/backend tests, the named mounted
fixture, `effigy regressions:native`, receipt and ledger tests,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Never run
windowed or native-visual selectors.

## Stop Conditions

Stop for orchestrator review if the proof needs a public API, another edit
machine, live draft channel, Nucleus data, browser-only selectors, broad A1/V1
claims, app persistence/validation policy, or OS IME authority. Record the gap
instead of weakening the receipt.

## Continuation

After merge, compile the next dependency-ready Nucleus M1 row from the
refreshed cohort. AgentChatInput and CommandPalette are unblocked by TextInput;
receipt-producing merges remain serial.
