# g16.065 — Tabs Native Tooltip Parity

Status: ready
Type: cross-runtime semantic and mounted repair
Opened: 2026-09-02
Depends on: current Tabs contract, completed `g16.060`
Governing refs: `nucleus-gpui-parity-programme.md`,
`../../contracts/components/tabs.md`

## Goal

Give `showTooltips` one documented cross-runtime meaning and propagate it
through the shared Rust renderer and mounted GPUI path before the Nucleus Tabs
cohort card.

## Fixed Boundary

The Rust spec carries `shows_tooltips`, but the renderer does not consume it.
Decide the existing house-native tooltip boundary from current primitives,
wire label/show/hide timing through production nodes, and prove it with mounted
input. Keep web drag behavior and `focusOnValueChange` unchanged.

## Acceptance

- `showTooltips=false` exposes no tooltip after the full delay.
- `showTooltips=true` exposes the selected/hovered tab label after the contract
  delay and hides it on leave, focus departure, disabled/removal, and teardown.
- Svelte, React, Rust spec, renderer, and mounted GPUI agree on the public
  meaning; runtime mechanism may differ.
- A Nucleus-shaped Tabs fixture is included without Nucleus data or source.
- Existing selection, focus, overflow, history, drag, and controlled-focus
  semantics remain unchanged.

## Review Oracle

| Invariant | Counterexample | Required proof |
| --- | --- | --- |
| Spec reaches renderer | `shows_tooltips=true` is dropped | mounted tooltip never appears |
| Delay is real | tooltip appears immediately | timer assertion fails |
| Lifecycle is bounded | tab is removed while pending | no late tooltip/task residue |
| False stays inert | adapter shows tooltip from label alone | negative mounted proof fails |
| Web semantics survive | native fix changes web drag/focus | focused paired Tabs suites fail |

## Writable Scope

Tabs contract/spec/renderer/GPUI tooltip path, focused paired and mounted tests,
one generic Nucleus-shaped fixture, this card, one log, and new papercuts. No
public prop addition, Nucleus edit, local windowed run, release, workflow,
visual-lab, or Jetstream change.

## Validation

Run focused core/Svelte/React/Rust/renderer/mounted Tabs tests, `effigy ci:web`,
`effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, and `git diff
--check origin/main...HEAD`. No windowed/native-visual selector.

## Stop Conditions

Stop if GPUI 0.2.2 cannot express the accepted tooltip lifecycle without a new
public primitive/Node contract; return that boundary for planning rather than
using preview-only state.

## Continuation

Accepted merge unblocks the later Nucleus Tabs component card. It does not
claim Nucleus M2, accessibility A2, or visual V2.
