# NumberInput Native Value Model

Status: open — separate semantic/API decision before mounted execution
Captured: 2026-08-26
Source: post-`g16.006` parity-ledger checkpoint

## Finding

`NumberInput` is a high-leverage text/value primitive, but its active-cohort
contract and shared Rust surface cannot currently describe the same value
model.

- Svelte and React accept `number | string | null | undefined`, preserve raw
  string-form drafts, allow an empty value, type directly, commit on blur, and
  expose value/submit/focus callbacks.
- `NumberInputSpec` stores one concrete `f64`; it cannot represent an empty or
  invalid draft.
- `poodle-render::number_input` renders a value label plus optional steppers.
  Its handlers expose increment/decrement only; the value surface is not an
  editable text field.
- The contract's Jetstream note describes the old pointer-only limitation. It
  does not authorize GPUI to omit direct editing, and Jetstream is deferred at
  program level.

This is not a mounted-test omission. A direct mounted test would prove the
wrong interface unless the native value/draft ownership is decided first.

## Decision Needed

Choose an idiomatic native representation that preserves the observable web
contract:

1. a typed committed number plus an explicit host-owned raw draft/empty state;
2. a tagged native value enum carrying numeric, raw-string, and empty forms;
3. a narrower native numeric-only contract recorded as an intentional runtime
   delta.

Option 1 is the current recommendation. It keeps numeric application state
typed while allowing the editor to preserve intermediate input such as `-`,
`.` or an empty field. The shared Rust transition should own parsing, clamping,
precision, stepping, and commit results; the host should own the current draft
and rebuild.

## Guardrail

Do not bundle NumberInput into a generic text-entry mounted card or promote its
current stepper-only renderer as parity. Do not add a compatibility alias or a
silent `0.0` fallback for empty/invalid drafts.

## Promotion Route

Return to this note after the core `TextInput` mounted lane. Confirm the native
draft model with the operator, promote it into
`docs/contracts/components/number-input.md`, then compile a dedicated semantic,
API, and mounted-parity card. Keep `ColorPicker` and `FilterBuilder` as
regression consumers rather than widening the first decision card.
