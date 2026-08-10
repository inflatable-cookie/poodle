# 001 Audio Family Phase 3

Status: completed
Owner: Poodle core
Created: 2026-08-10
Depends on: `docs/architecture/008-audio-control-family.md`,
`docs/roadmaps/g13/001-audio-cross-runtime-parity.md`

## Scope

Close the audio family with three generalized components using the established
VisualState seam and full Svelte, React, GPUI, and Jetstream parity.

## Contracts

- [x] Add canonical Keyboard, Waveform Display, and Mod Matrix Grid contracts.
- [x] Extend Architecture 008 with note gestures, peak-pyramid limits, and the
  generic matrix model.
- [x] Keep input, hit testing, accessibility, and host effects outside drawing.

## Shared Machines

- [x] Add Keyboard range, velocity, held-note, external-highlight,
  computer-key, octave-shift, and horizontal/vertical geometry laws with
  exhaustive tests.
- [x] Add Waveform Display peak-pyramid validation/selection plus cursor and
  selection transitions with the 4,096-column ceiling.
- [x] Add Mod Matrix Grid normalization, bipolar amounts, enablement, and
  keyboard navigation with exhaustive tests.
- [x] Mirror the machine and VisualState contracts in Rust.

## Runtime Surfaces

- [x] Add thin Svelte and React adapters over shared web cores and styles.
- [x] Add native specs and VisualState-only node builders shared by GPUI and
  Jetstream.
- [x] Register all three components and their standalone specimens in all four
  previews.
- [x] Cover all five sizes and three densities on every specimen page.

## Acceptance

- [x] Every renderer consumes serializable VisualState only.
- [x] Note gestures, waveform reduction, and matrix laws have golden tests.
- [x] Recipe hooks have no candidate-only tokens and are documented.
- [x] Contracts, exports, registries, manifests, accessibility reports, and
  specimens are complete across every in-flight backend.
- [x] Focused web/native checks and repository parity gates pass or existing
  unrelated exceptions are recorded.

## Closeout Evidence

- Core, component, accessibility, parity, Rust contract, renderer, GPUI, and
  Jetstream adapter tests pass.
- Svelte and React preview builds pass. Contract prop/spec drift and native role
  drift pass.
- Focused GPUI and Jetstream visual captures for all three components compare
  exactly after baseline refresh.
- The repository-wide Svelte surface audit still reports the pre-existing
  `AgentSubagent` usage-doc gap. No Phase 3 component has a coverage gap.
- The Jetstream AccessKit launcher stalled once after opening the preview;
  Jetstream's static accessibility audit passed across 147 specimens and the
  focused offscreen visual capture passed.
