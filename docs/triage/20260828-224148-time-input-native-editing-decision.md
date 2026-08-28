# TimeInput Native Editing Decision

Status: resolved — approved and promoted into the TimeInput contract and g16.029
Captured: 2026-08-28
Source: g16 component-continuation planning while g16.021 runs

## Current Boundary

The public web component already has a useful value contract:

- controlled `value: string | null | undefined`;
- uncontrolled `defaultValue: string | null`;
- accepted committed forms `HH:MM` and `HH:MM:SS`;
- `min` and `max` use the same forms;
- positive step size is expressed in seconds; and
- `onValueChange` reports `string | null`.

Svelte and React delegate editing to native `input[type=time]`. The Rust
declaration preserves those props, but `poodle-render` presents one
unconstrained text input. It does not model time segments, bounds, step
alignment, partial entry, or a contract-valid commit. GPUI therefore cannot
claim the observable result of the web control.

The public string is not the problem. Replacing it with a time object would
create needless consumer churn and still leave partial editing unresolved.
The missing boundary is committed value versus adapter-owned draft.

## Recommended Model

Keep the existing external value shape and add one paired pure time-entry
model in TypeScript and Rust:

- committed values are canonical 24-hour `HH:MM` or `HH:MM:SS` strings, or
  `null`;
- parsing, formatting, bound membership, step alignment, and stepping are pure
  shared semantics;
- Svelte/React may retain the native browser control, while GPUI renders one
  visual control containing hour, minute, and conditional second segments;
- seconds appear when `step < 60` or an authored value/bound includes a seconds
  segment;
- each native segment is a labelled spin-button-like focus stop inside one
  labelled group; Tab/Shift+Tab traverse the segments and then leave;
- digit and arrow interaction may create an incomplete or invalid local draft,
  but that draft is adapter-owned and never becomes the controlled value;
- `onValueChange` fires live only when the edit forms a complete,
  constraint-valid value, or when the whole control is cleared to `null`;
- an invalid/incomplete draft is visibly invalid while focused and reverts to
  the last committed value on blur or Escape; it is not silently clamped;
- external controlled-value replacement discards any local draft; and
- disabled controls expose no editable segment or callback path.

This keeps the host out of keystroke bookkeeping while preserving the current
portable value and callback. It also avoids importing NumberInput's open raw
draft problem into TimeInput.

## Constraint Recommendation

- Require a positive whole-second `step`; fractional seconds remain outside
  the documented value grammar.
- Anchor the step grid at `min` when present, otherwise `00:00:00`.
- Arrow stepping moves by the configured step and stops at the allowed bound;
  direct digit entry is accepted only when it lands on the same grid.
- Support the browser time-input convention where `min > max` denotes a valid
  range that crosses midnight. This matters for schedules and avoids making
  the GPUI contract weaker than the web primitive.
- Keep locale-specific 12-hour presentation outside this card. The portable
  value and the first custom GPUI editor remain deterministic 24-hour time.

## Clean Rust Migration

The Rust public type and renderer still use the legacy `TimeFieldSpec` /
`time_field` name although the component has long been `TimeInput` everywhere
else. The implementation card is the natural pre-1.0 breaking tranche to
rename them to `TimeInputSpec` / `time_input` and migrate in-repository
callers. The recommendation is a clean rename with no alias or compatibility
wrapper.

## Decision

The operator approved this package on 2026-08-28:

1. segmented 24-hour native editing with conditional seconds;
2. adapter-owned drafts and valid-value-only callbacks;
3. invalid drafts revert rather than clamp;
4. whole-second step plus overnight-range support; and
5. clean `TimeFieldSpec` to `TimeInputSpec` Rust rename.

The decision is now authoritative in
`docs/contracts/components/time-input.md` and compiled as
`docs/roadmaps/g16/029-time-input-semantic-model-and-native-parity.md`.
Implementation remains serial with g16.021 because both lanes edit paired
TypeScript/Rust core exports and shared conformance infrastructure. This triage
note is historical context, not execution authority.
