# g13.006 RangeSlider Stateful-control Proof

Status: planned
Owner: Poodle core
Depends on: `g13.005`

## Objective

Stress the IR with controlled state, gesture effects, repeated anatomy, and
value-dependent VisualState.

## Deliverables

- RangeSlider definitions for standard/embedded treatment, unipolar/bipolar
  geometry, orientation, direction, sizes, densities, detents, and two thumbs.
- Declarative transition/effect intent where portable; shared conformance
  vectors where runtime machines remain hand-written.
- Adapter-owned pointer capture, hit-testing, keyboard input, and ARIA values.
- Four-runtime specimens and focused interaction/visual evidence.

## Acceptance

- Thumb selection and gesture begin/move/end semantics match the contract.
- Negative/positive fill geometry and recipe roles remain exact.
- The definition uses no runtime-specific value path or untyped side channel.

## Next

`g13.007` tests the boundary that should remain runtime-native.
