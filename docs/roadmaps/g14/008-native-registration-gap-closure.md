# g14.008 Native Registration Gap Closure

Status: planned
Owner: Poodle core
Depends on: `g14.002` (gap baseline), `g14.007` (capability declarations)
Governing refs: `../g13/native-registration-gap.md`,
`../g13/014-native-backend-convergence.md` (acceptance carried, not the
card), `../../contracts/001-working-rules.md` (§Every component ships in
all four runtimes)

## Objective

Close the sixteen-component native registration gap by building each
component once on `poodle-render`, interpreted by GPUI and Jetstream.
This is where the drift sweeps actually get finished: missing is the
largest class of native hole, and it is a standing working-rules
violation, not a parity nicety.

## Deliverables

- Each missing component: contract-checked Svelte surface read as
  reference, a `poodle-render` implementation, registration in both native
  registries, and a specimen in both native previews.
- The strict-interpreter rule carried from g13.014: no placeholder
  rendering, no bypassing the shared path; registration parity cannot
  report green on a stub.
- **HistoryCentre is a named decision point**, carried from the b028
  deferral: flat-list core with no recursive renderer, callbacks for the
  three host operations. Maintainer rules on whether native HistoryCentre
  is in scope for g14 or stays deferred with a recorded reason.
- A gate that fails when a Svelte component lacks a native registration.

## Acceptance

- [ ] The gap table reaches zero (or every remaining row carries a
  recorded reason).
- [ ] Every new native component is a `poodle-render` consumer, never a
  hand-rolled fork.
- [ ] Parity evidence exists per component, not registration alone.

## Next

`g14.009` converts specimen evidence into standing gates. Scene-authored
specimens (`g14.003`) land for each component as its registration closes.
