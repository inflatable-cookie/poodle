# g16.032 — Continuous Audio Native Mounted Parity

Status: complete — awaiting review
Date: 2026-08-29
PR: https://github.com/inflatable-cookie/poodle/pull/100
Branch: `t3code/continuous-audio-native-parity`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-03a3eccc`
Card: `docs/roadmaps/g16/032-continuous-audio-native-mounted-parity.md`
Handoff: `docs/handoffs/20260829-231553-g16-032-continuous-audio-native-mounted-parity.md`

## Outcome

Knob, Fader, and XYPad now mount as real GPUI controls over the paired Rust
machines from `g16.031`. One renderer-neutral Node continuous-value event
owns captured press/move/release/cancel. Wheel and double-activation are
separate Node routes. Page Up/Down join the physical key vocabulary.

The ledger moves only those three GPUI mounted-behaviour cells: 49 → 52
mounted, 125 → 122 missing. Accessibility and visual-comparison cells do not
move. Known-delta totals are unchanged.

## Node event

`Interaction::on_continuous_value` carries `ContinuousValuePhase` (press, move,
release, cancel), local x/y with x right and y up, per-dispatch logical-pixel
delta on the same axes, and `NodeModifiers`. It has no payload, pointer id,
GPUI type, or window coordinate.

The GPUI backend admits one primary pointer, captures moves with `on_drag_move`,
releases inside with `on_mouse_up` and outside with `on_mouse_up_out`, and
cancels exactly once when the host is gone. A second press while a gesture is
open is inert. crates.io GPUI 0.2.2 supplied those facts without a fork.

`on_scrub` remains Slider/RangeSlider. `on_drag` remains ResizeHandle.

## Mounted GPUI

Named headless regressions:

- `a_continuous_value_gesture_releases_once_and_cancels_on_lost_host`
- `fader_mounted_parity_through_production_dispatch`
- `knob_mounted_parity_through_production_dispatch`
- `xy_pad_mounted_parity_through_production_dispatch`

Fader proves horizontal and vertical mapping, detents, fine rebase, Page
keys, wheel that is consumed so a parent does not also scroll, double-click
reset, Enter/type/Enter and Escape type-in with focus return, callback
pairing, disabled inertia, two-instance identity, and a rebuild between
press and move. Knob proves vertical and circular mapping, fine rebase
across a host rebuild, Page keys, wheel, reset, type-in, callbacks,
disabled inertia, and Slider accessibility. XYPad proves coarse press,
atomic pair moves, fine rebase, reset, independent axis keys, callbacks,
disabled inertia, two-instance identity, and two child Slider semantics.

GPUI Examples for the three controls use the renderer-owned instance
registry and a value readout so interaction rebuilds the page. Size and
density matrices stay on the shared specimen builders.

## Review round

PR review on `aa64b9471` asked for five blockers. This round:

1. Required `Handlers::new(instance_id)` scopes root, entry, and XY axis
   ids. Two-instance rebuild regressions cover Fader and XYPad.
2. `overlay_frame_begin` now cancels a continuous-value host that was not
   rebuilt last frame. Production preview already calls begin. Lost-host is
   also proven on `HeadlessDriver::draw_preview_frame` (begin only, no
   `overlay_frame_end`).
3. Wheel dispatch calls `prevent_default` and `stop_propagation`.
   `RequestEntryFocus` queues Node `request_focus`. Entry fields have a
   label and focus ring. Enter/Escape return focus to the root. Root
   `on_submit` is omitted while entry is open so Enter cannot reopen.
4. Machines live in a renderer registry keyed by instance id, not an
   optional field on the callback struct. Host value replacement applies
   only while idle. Rebuild between press and move keeps the gesture.
5. The named mounted tests now exercise the acceptance paths above.

Entry blur is wired (`on_focus_change` commits once) and unit-tested on the
node. Headless window-blur/tab did not deliver that callback for the child
input; Enter/Escape are the mounted proof.

## Non-claims

- No TypeScript, Svelte, or React behaviour change
- No paired-machine vector change
- No payload drag-and-drop work
- No GPUI accessibility-axis or visual-comparison cell moved
- Jetstream received no behaviour claim; mechanical compile only if Node
  vocabulary required it
- Specimens are examples, not a hidden conformance matrix

## Validation

Focused Node and GPUI backend lifetime tests, named mounted Fader/Knob/XYPad
regressions, and retained Slider/RangeSlider/ResizeHandle regressions passed
during implementation. Final board: every selector named on card 032, all
headless.

## Next

Do not start payload drag-and-drop, another audio control, accessibility,
visual comparison, motion, Longhorn-lab, or Jetstream work from this PR.
After operator-authorized merge, the orchestrator chooses from `g16.022` and
the component-continuation runway. Never overlap a Node/GPUI routing card
with `g16.025`.
