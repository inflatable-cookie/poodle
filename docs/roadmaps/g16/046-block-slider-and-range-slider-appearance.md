# g16.046 — Block Slider And RangeSlider Appearance

Status: ready
Type: implementation
Opened: 2026-09-01
Depends on: merged `g16.034` and the accepted block-slider packet in
`../../triage/20260901-221756-block-slider-promotion-decision.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/slider.md`,
`../../contracts/components/range-slider.md`,
`../../contracts/components/size-and-density.md`,
`../../architecture/007-appearance-recipe-contract.md`,
`../../architecture/012-semantic-motion-policy.md`

## Goal

Add one opt-in `appearance="block"` treatment to Slider and RangeSlider across
the active cohort. Preserve current track/embedded defaults and value machines.
Prove visible content, fit fallback, direction, stable thumb ownership,
idempotent terminal delivery, forced colors, and effective targets without
changing paging or admitting vertical RangeSlider early.

## Fixed Public Shape

- Shared `SliderAppearance = "track" | "block"`; `appearance` defaults to
  `track` and stays orthogonal to `variant`.
- Slider adds `visibleLabel` and `formatVisibleValue`. RangeSlider adds the
  same label, per-thumb `formatVisibleValue`, and `formatVisibleRange`.
  Native specs carry resolved strings, not closures.
- `direction: "ltr" | "rtl"` defaults to `ltr`. Horizontal geometry mirrors
  in RTL; arrow keys retain numeric meaning.
- Inline text is all-or-nothing. Every item must fit its assigned region under
  the accepted floor/ceil measurement law or one stable, noninteractive,
  accessibility-hidden line renders after the capsule.
- Exact RangeSlider pointer ties choose lower. The selected thumb remains the
  gesture owner, clamps at its sibling, and never swaps identity.
- Release, cancellation, lost capture, disablement, and teardown share one
  idempotent terminal. The first terminal emits one commit with the latest
  accepted value; cancellation does not roll back.
- Every Slider control and RangeSlider thumb has a measurable 44×44 logical-
  pixel effective target. Forced-color roles follow the accepted role table.
- PageUp/PageDown behavior stays unchanged in this card. Vertical active-
  cohort support stays gated on real native RangeSlider axis geometry.
- Invalid, read-only, indeterminate, tooltip-only, and public fit metrics stay
  outside the component surface. Jetstream remains deferred.

## Ordered Work

1. Amend Slider/RangeSlider and appearance-recipe authority. Add shared types,
   resolved native fields, fit/formatter laws, direction, terminal semantics,
   and the horizontal-only admission boundary.
2. Extend the current normalized machines and terminal effects. Do not fork
   value math for block paint.
3. Implement Svelte/React shared styling and fit measurement with stable
   fallback. Implement shared Rust composition and GPUI horizontal geometry,
   per-thumb metadata, hit bounds, and paint roles.
4. Add paired behavior, boundary-fit, forced-color, package-surface, browser,
   Rust-node, and mounted GPUI evidence. Preserve the track appearance byte-
   for-byte in observable API and callback traces.
5. Close with one execution log. Do not move visual ledger cells from this
   implementation proof.

## Acceptance

- Omitting `appearance` leaves current anatomy, values, callbacks, and track /
  embedded visuals unchanged.
- Formatter inputs are normalized, bounds-guarded, step-snapped values.
  Visible and accessible text channels never feed each other.
- Required-minus-one falls back; equality fits; required-plus-one fits. The
  external readout remains stable across focus, overlap, and thumb movement.
- Horizontal LTR/RTL geometry and numeric keyboard behavior agree in every
  active runtime.
- Equal-value RangeSlider keeps two focusable semantic thumbs. Pointer tie
  selects lower; upper remains directly keyboard operable.
- Every terminal sequence emits at most one commit and preserves live-change
  ordering through callback-triggered teardown.
- Web hit probes, Rust bounds, and mounted GPUI dispatch prove 44×44 targets
  from `xs` through `xl` and every density.
- Native dependent bounds and names/value text are per-thumb. GPUI metadata is
  not described as mounted assistive-technology proof.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Default is inert | existing Slider with no new props | before/after anatomy and callback trace match |
| Fit is atomic | label fits, value misses by one pixel | no inline text; one stable fallback line |
| Channels stay separate | only `ariaLabel="Gain"` is supplied | no visible `Gain` text |
| Tie and gesture identity are stable | `[50,50]`, upper previously focused | pointer chooses lower; drag never swaps |
| Terminal is idempotent | cancel then lost-capture then teardown | one commit, no rollback, later terminals inert |
| Targets are real | `xs` equal-value range | both 44×44 bounds exist; lower pointer tie and upper keyboard work |
| Forced colors preserve roles | selected and remainder both resolve to canvas | role/contrast gate fails |
| Vertical stays gated | native vertical specimen still uses horizontal scrub geometry | no advertised vertical block support |

Plant and restore the pre-fix behavior for every row after the real proof is
committed.

## Writable Scope

Slider/RangeSlider contracts; appearance recipe only for the shared role map;
component-local core, Svelte, React, Rust spec/render, GPUI, fixtures, tests,
styles, and public types required by the two controls; this card, one log, and
new papercuts. Do not edit unrelated controls, paging semantics, releases,
workflows, downstream consumers, visual ledger cells, native AT claims, or
Jetstream behavior.

## Validation

Run focused machine/terminal, web shell, package surface, fit/geometry,
forced-color, Rust node, and mounted GPUI checks; relevant drift selectors;
`effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, one
final headless `effigy qa`, and `git diff --check origin/main...HEAD`. Never run
`*-windowed` or native-visual selectors.

## Stop Conditions

Stop if the implementation needs public fit thresholds, thumb swapping,
rollback state, new paging semantics, a generic tooltip/readout system,
vertical admission without native axis proof, a new motion role, or a native
AT claim beyond contract 003.

## Continuation

After accepted merge, any PageUp/PageDown convergence is a separate all-
appearance decision. It is not implied by this card.
