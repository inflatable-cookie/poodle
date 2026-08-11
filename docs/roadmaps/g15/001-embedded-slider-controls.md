# 001 Embedded Slider Controls

Status: completed
Owner: Poodle core
Created: 2026-08-11
Depends on: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`docs/contracts/components/mod-matrix-grid.md`,
`docs/architecture/008-audio-control-family.md`

## Scope

- Add standard/embedded variants to Slider and RangeSlider.
- Put embedded pointer gestures, law mapping, and fill geometry in shared web
  and Rust cores.
- Cover unipolar/bipolar geometry in all four specimens.
- Replace ModMatrixGrid's bespoke pointer/value path with the embedded Slider
  control seam.

## Acceptance

- [x] Pointer begin/move/end and thumb selection have machine tests.
- [x] VisualState fully defines value, center, and fill geometry.
- [x] Svelte, React, GPUI, and Jetstream contracts and specimens agree.
- [x] ModMatrixGrid no longer computes slider pointer/value behavior itself.
- [x] Relevant core, component, accessibility, contract, build, and visual
  checks pass.
