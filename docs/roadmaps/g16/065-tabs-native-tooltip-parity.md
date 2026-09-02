# g16.065 — Tabs Native Tooltip Parity

Status: blocked — GPUI 0.2.2 tooltip lifecycle; PR #169
Type: cross-runtime semantic and mounted repair
Opened: 2026-09-02
Depends on: current Tabs contract, completed `g16.060`
Governing refs: `nucleus-gpui-parity-programme.md`,
`../../contracts/components/tabs.md`
Log: `../../logs/2026-09/20260902-g16-065-tabs-native-tooltip-parity.md`

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

This card does not complete. Nucleus Tabs stays blocked on a planned tooltip
delay/dismiss contract, or a house backend runtime that is not GPUI
`.tooltip()`. Do not treat merge of the projection work as native tooltip
parity.

## Outcome

Stopped on the card stop condition. `shows_tooltips` now projects each tab's
trimmed label onto `Node.tooltip` when the flag is true or the strip is
vertical. That projection is kept. It is not the accepted lifecycle.

The existing production boundary (`Node.tooltip` → GPUI 0.2.2 `.tooltip()`)
cannot meet the contract 300ms delay or hide on focus departure. Those gaps
are not accepted deltas.

## Blocking Boundary

Returned for planning. Do not invent a Tabs-only overlay, a `tooltip_delay`
Node field, or a second tooltip mechanism in this lane.

1. **Delay.** GPUI 0.2.2 `Interactivity::tooltip` hardcodes private
   `TOOLTIP_SHOW_DELAY = 500ms`. There is no delay argument and no
   `TooltipOptions`. `Node.tooltip` is `Option<String>` only.
2. **Focus-departure / Escape dismiss.** Show and hide are mouse hitbox
   (`handle_tooltip_mouse_move`, `check_visible_and_update`).
   `clear_active_tooltip` is `pub(crate)` inside gpui. `Interaction::on_focus_change`
   can observe blur; it cannot clear GPUI's private `ActiveTooltip`.

A later card would need a public contract such as tooltip delay plus dismiss
policy on Node, or an adopted backend-owned tooltip runtime that replaces
`.tooltip()` as the house path.

## Expressible On The Existing Boundary

- `showTooltips=false` omits `Node.tooltip` (inert after any delay).
- Trimmed label projection when `shows_tooltips` or vertical.
- Hide on pointer leave (non-hoverable `.tooltip()`).
- Cancel pending show on removal and teardown (unpaint drops `WaitingForShow`).

Disable-hide is renderer policy: omit `Node.tooltip` when the tab is disabled.
That does not need a new Node field. Current projection still emits the label
on disabled tabs, matching web wrap. The card names hide. Unresolved; not
forked here, and it does not unblock delay or focus-departure.
