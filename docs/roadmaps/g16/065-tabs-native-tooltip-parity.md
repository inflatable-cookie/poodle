# g16.065 — Tabs Native Tooltip Parity

Status: complete — merged in PR #172 at `718d6f082`
Type: cross-runtime semantic and mounted repair
Opened: 2026-09-02
Depends on: current Tabs contract, completed `g16.060`, completed `g16.066`
Governing refs: `nucleus-gpui-parity-programme.md`,
`../../contracts/components/tabs.md`
Log: `../../logs/2026-09/20260902-g16-065-tabs-native-tooltip-parity.md`
PR: #172

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

- `showTooltips=false` exposes no tooltip after the full delay, on hover or
  keyboard focus.
- `showTooltips=true` exposes the hovered or keyboard-focused tab label after
  300ms and hides it on leave, blur, Escape, disabled/removal, and teardown.
  Disabled tabs never enter pending or visible.
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
| Disabled stays inert | web schedules a disabled tab | paired Svelte/React disabled proofs fail |
| Live disablement cancels | paint-gate only while Search is pending or visible | paired rerender proofs rematerialize Search |
| Removal follows identity | index-latched timer paints Git after Search is removed | paired rerender proofs show Git or a late tooltip |
| Horizontal focus matches native | `onFocus` only schedules when vertical | paired 299/300ms keyboard proofs fail |
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

PR #169 was closed without merge. Resume is this branch rebased onto merged
`g16.066`. Accepted merge unblocks the later Nucleus Tabs component card. It
does not claim Nucleus M2, accessibility A2, or visual V2.

## Outcome

`shows_tooltips` projects each tab's trimmed label onto `Node.tooltip` when
the flag is true or the strip is vertical. Empty labels are omitted. Disabled
tabs still project the label (web wrap); the shared GPUI backend keeps them
inert. Web never schedules or paints a disabled tab. Horizontal
`showTooltips=true` schedules on keyboard focus and paints at 300ms, matching
`Node.tooltip`. Delay is 300ms. Leave, blur, Escape, removal, and teardown
hide. No new Node field.
