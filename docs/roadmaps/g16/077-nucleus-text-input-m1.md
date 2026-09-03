# g16.077 — Nucleus TextInput M1 Receipt

Status: ready
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.007`, completed `g16.062`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`007-text-input-controlled-editing-and-mounted-evidence.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/text-input.md`
Handoff: `../../handoffs/20260903-104800-g16-077-nucleus-text-input-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `TextInput` row through the
production Rust adapter, renderer, Node, GPUI backend, and test-platform input
paths. Retain the established g16.007 behavior envelope and convert its named
mounted regression into exact receipt evidence without widening TextInput.

## Fixed Boundary

- Keep the manifest test name
  `text_input_controlled_editing_and_identity_rebuild_the_host_spec` and
  strengthen it rather than creating a duplicate receipt fixture.
- Mount `node_compat::TextInput::from_spec(...).into_element()` through the
  element-backed HeadlessDriver. Renderer-only Node construction is not
  adapter evidence.
- Host state owns value, selection, focus, and rebuilds. Drive focus, pointer,
  printable input, caret/selection movement, replacement, Backspace/Delete,
  Home/End, submit, cancel, blur, and search clear through mounted production
  input. Do not call transitions or handlers directly after mount.
- Prove two equal-valued fields with caller-owned ids keep focus, selection,
  edit history, callbacks, and composition identity separate.
- Prove disabled inertia; read-only focus/selection with mutation suppression;
  placeholder/value separation; exact scalar `maxLength`; validation,
  required, described-by, label, value, and selection projection at the Node
  boundary. Do not infer an accessibility tree from Node metadata.
- Preserve ordered value-before-clear effects, one submit/cancel callback per
  input, and exactly one blur transition. Teardown is silent. Existing focused
  clipboard, undo/redo, caret paint, and IME tests remain regression evidence,
  not claims of OS input-method parity.
- Assert contract-owned typography, field surface, focus/validation treatment,
  padding, cursor, and positive mounted bounds/containment. Do not claim V1
  pixels or multiline geometry.
- A focused native repair is allowed only after a committed mounted
  counterexample. Stop for a public API, another editor machine, multiline or
  slug lifecycle, browser-only behavior, app validation policy, or OS IME work.
- Emit the receipt only after every claimed assertion. Refresh the manifest,
  all existing receipts, generated ledger, this card, and one execution log
  from the exact committed runtime source. No other row advances.

## Acceptance

- TextInput has one valid `nucleus.settings.text-input` M1 receipt naming the
  retained mounted test. The denominator stays 29 and the existing 13 receipts
  remain valid.
- Raw renderer mounting, shared field identity, direct handler calls, callback-
  only state without rebuild, disabled/read-only leakage, UTF-16 max-length
  authority, placeholder-as-value, crossed selection/focus, or early receipt
  emission fails the proof.
- M1 does not infer A1, V1, browser DOM parity, OS IME behavior, multiline,
  slug lifecycle, Nucleus M2, or Jetstream.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::text_input` directly | adapter-path assertion or lifecycle fails |
| Identity is caller-scoped | give equal-valued fields one id | focus, selection, history, or callbacks cross streams |
| Input is mounted | call edit transition or handlers directly | mounted observation or callback trace is absent |
| Controlled ownership is real | record callback without rebuilding supplied state | painted value/selection disagrees with host state |
| Disabled/read-only differ | let read-only mutate or disabled focus | exact focus/edit trace fails |
| Scalar length is exact | enforce by UTF-16/code units | astral-character max-length vector fails |
| Placeholder is not value | copy or submit placeholder text | value/selection/clipboard witness fails |
| Commands are exact | mutate on Enter/Escape or double-fire | value and callback counts fail |
| Clear ordering is exact | emit clear before empty value | ordered effect trace fails |
| Blur/teardown are exact | double blur or emit on unmount | terminal callback trace fails |
| Structure and tokens are exact | drop semantic projection or field metadata | Node assertion fails |
| Receipt is terminal | fail final independent-field assertion | no TextInput receipt is emitted |
| Evidence identity is exact | retain the g16.076 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

The retained TextInput mounted regression; focused TextInput spec, machine,
renderer, backend, and GPUI adapter tests; a bounded native repair only after a
committed counterexample; receipt/manifest/ledger refresh; this card; one log;
and new papercuts. Do not edit Nucleus, web behavior, public APIs, visual-lab
code, Jetstream, workflows, versions, releases, or other component rows.

## Validation

Run focused TextInput spec/machine/render/backend tests, the named mounted
fixture, retained clipboard/undo/IME regression tests when touched,
`effigy regressions:native`, `effigy check:parity-evidence-ledger`,
`effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, and
`git diff --check origin/main...HEAD`. Do not run windowed or native-visual
selectors.

## Stop Conditions

Stop for orchestrator review if the proof needs a public API, another editor
machine, browser-only selectors, Nucleus data, broad A1/V1 claims, multiline or
slug lifecycle, app validation policy, or OS IME authority. Record the exact
gap instead of weakening the receipt.

## Continuation

After merge, TextInput unblocks the Nucleus EditableLabel, AgentChatInput, and
CommandPalette dependency rows. Compile the next smallest dependency-ready M1
receipt from the refreshed identity; receipt-producing merges remain serial.
