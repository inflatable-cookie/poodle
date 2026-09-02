# g16.064 — Slider Negative-half Rounding Parity

Status: ready
Type: paired semantic repair
Opened: 2026-09-02
Depends on: current Slider and RangeSlider machines
Governing refs: `nucleus-gpui-parity-programme.md`,
`../../contracts/components/slider.md`

## Goal

Define one cross-runtime step-quantization law and make TypeScript and Rust
return the same value and callback at negative half steps.

## Fixed Boundary

JavaScript `Math.round(-0.5)` and Rust `f64::round(-0.5)` differ. State the
portable tie law explicitly, implement it in the shared machines, and cover
Slider and RangeSlider boundaries. Do not alter block appearance, gestures,
axis behavior, or Nucleus.

## Acceptance

- One shared vector corpus covers negative half, positive half, min-offset,
  non-zero min, step larger than range, and safe-max cases.
- Core and headless produce identical normalized values and change/commit
  effects for every vector.
- Existing pointer, keyboard, controlled, and block-appearance behavior stays
  unchanged.

## Review Oracle

| Invariant | Counterexample | Required proof |
| --- | --- | --- |
| Tie law is portable | raw value resolves to step index `-0.5` | same exact value in TS/Rust |
| Offset matters | non-zero minimum with half step | vector uses `(value-min)/step` |
| Range stays paired | fix only single-thumb Slider | RangeSlider vector fails |
| Max remains safe | last step exceeds max | clamped value stays within range |

## Writable Scope

Slider/RangeSlider core and headless normalization, shared vectors/tests,
contract wording, this card, one log, and new papercuts. No appearance, shell,
Nucleus, lab, workflow, release, or Jetstream changes.

## Validation

Run focused core/headless Slider and RangeSlider tests, vector parity checks,
`effigy ci:web`, `effigy ci:rust`, `effigy docs:check`, and `git diff --check
origin/main...HEAD`. No windowed selector.

## Stop Conditions

Stop if the chosen tie law changes an already documented public value outside
the half-step counterexample without an explicit contract decision.

## Worker Evidence

Branch `fix/g16-064-slider-rounding` (PR pending — orchestrator owns merge and
status). Law chosen: half ties round toward positive infinity (`Math.round`),
because core is the documented authority and `color.rs` already mirrors
`Math.round` for conformance; `f64::round` (half away from zero) was the
drift. Only `poodle-headless::slider::snap_to_step` changed law; TypeScript
kept `Math.round` and documents the law.

The negative-half counterexample bit before the repair: the shared corpus
added in this branch ran red on Rust (`slider_snap_conformance` failed:
`snap_to_step(-0.5, 0, 1)` = `-1.0` vs pinned `0.0`) while TypeScript passed.
Transition outputs were already equalized by the `min` clamp, so the corpus
pins the law through `sliderSnap` cases and through `slider`/`rangeSlider`
INPUT/COMMIT effects (negative half, positive half, min-offset, non-zero
min, step larger than range, safe-max). Full record:
`docs/logs/2026-09/20260902-g16-064-slider-negative-half-rounding-parity.md`.

## Continuation

Record the repaired pair in Nucleus confidence evidence. It does not block
unrelated cohort cards.
