# NumberInput Native Value Model

Status: resolved — approved and promoted to contract plus `g16.030`
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

Option 1 was the recommendation taken forward. It keeps numeric application
state typed while allowing the editor to preserve intermediate input such as
`-`, `.` or an empty field. The paired transition owns parsing, validity,
precision, stepping, and commit results. Web adapters may own an uncontrolled
draft; controlled web use and the declarative Rust host expose the same raw
draft channel explicitly.

## 2026-08-28 Evidence Pass

The current web API does not provide the raw-string model its contract claims.
It infers a payload mode from the authored `value`/`defaultValue`, parses every
complete draft through `Number(...)`, and then returns either the parsed number
or `String(parsedNumber)`. A string such as `"01.20"` therefore becomes
`"1.2"`; partial and invalid text remains adapter-local and is never returned.
The string union changes callback type, not editing semantics.

The mismatch is wider than the spec field:

- Svelte and React commit every parseable edit immediately, including values
  outside min/max or off step, then clamp and snap again on blur.
- An invalid draft reverts indirectly when blur ends editing, but Escape has no
  behavior and Enter submits the last parsed value rather than explicitly
  resolving the draft.
- `onIncrement` and `onDecrement` duplicate `onValueChange`; no in-repository
  or inspected sibling consumer uses them. `onSubmit` is exercised only by the
  component's own tests.
- `snapToStep` and `formatNumber` are duplicated in the two framework packages
  rather than owned by the shared core.
- Rust exposes a concrete `f64`, infinite sentinel bounds, and a display-only
  value node. Its preview owns only increment/decrement intent and silently
  replaces unparsable specimen state with a fallback number.

Sibling evidence explains why a raw-draft channel is still needed. Acowtancy
has many string-bound numeric form fields, Jetstream's inspector binds engine
text, and Underlay's action dialog uses NumberInput for schema-driven numeric
fields. Those consumers need empty and partial text without making the
portable committed value itself polymorphic.

## Recommended Resolution

Use two explicit channels instead of a number-or-string union:

- committed `value` / `defaultValue`: finite `number | null` on web and
  `Option<f64>` in Rust;
- optional controlled `draftValue`: raw `string` while editing, paired with
  `onDraftValueChange`; when uncontrolled, the adapter/host wrapper stores the
  same draft locally;
- `min`, `max`, and `step`: numeric optional values only; `step` must be finite
  and positive when present;
- `precision`: an explicit decimal normalization/display rule, not a second
  string-value mode.

The paired pure TypeScript/Rust machine should own parsing, decimal
normalization, bound and step validity, stepping, clearing, controlled-value
replacement, and draft resolution. Adapters own key/pointer/focus dispatch,
storage of an uncontrolled draft, drawing, and accessibility projection.

Recommended behavior:

- every raw edit updates the draft channel;
- clearing the whole field emits committed `null`;
- a complete finite draft emits a normalized number only when it satisfies
  authored bounds and step; incomplete, non-finite, out-of-range, and off-step
  drafts emit no committed value and expose invalid draft state;
- blur or Escape reverts an unresolved draft to the last committed value;
- Enter resolves a valid draft and reports commit; it never submits a stale
  value for an invalid draft;
- external controlled-value replacement discards an uncontrolled draft;
- steppers and Arrow Up/Down start from a valid draft when one exists, otherwise
  the committed value, then apply step/bounds/precision deterministically;
- no silent clamp, snap, `0`, or `step=1` recovery is used for invalid authored
  input or configuration.

For a clean pre-1.0 interface, replace `onSubmit` with `onCommit`, remove the
redundant `onIncrement` / `onDecrement` callbacks, and keep `onValueChange` for
committed numeric changes. This is intentionally breaking. The implementation
card must inventory downstream string-bound consumers and provide a migration
table, but it must not add aliases or update sibling repositories.

The operator approved this package on 2026-08-28. It is now authoritative in
`docs/contracts/components/number-input.md` and compiled for execution as
`docs/roadmaps/g16/030-number-input-value-draft-and-mounted-parity.md`.

## Guardrail

Do not bundle NumberInput into a generic text-entry mounted card or promote its
current stepper-only renderer as parity. Do not add a compatibility alias or a
silent `0.0` fallback for empty/invalid drafts.

## Promotion Route

Complete `g16.029` first because the cards overlap shared core/headless exports
and the domain-vector corpus. Then dispatch `g16.030` from the landed base.
Keep `ColorPicker` and `FilterBuilder` as bounded regression consumers and do
not widen the card into sibling-repository migrations.
