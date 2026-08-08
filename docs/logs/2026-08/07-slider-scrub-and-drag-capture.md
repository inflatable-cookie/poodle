---
title: Slider drags detached from the pointer; scrub replaces delta guessing
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, poodle-render, node-backend, slider, drag]
---

## Symptoms

Four, reported against the header contrast slider:

1. A drag stayed with the mouse for a few pixels, then detached.
2. The value moved only a little per drag, never keeping up with the pointer.
3. Clicking the track did nothing.
4. The control was too short and misaligned in the header.

## Causes

Three distinct ones, all real.

**Drags detached because the backend used `on_mouse_move`.** gpui only fires
that listener while the pointer is over the element's own hitbox
(`div.rs:269` — `hitbox.is_hovered(window)`), so the gesture died the instant
the pointer left a 6px-tall track. The right primitive is `on_drag_move`, whose
own documentation says it fires "for all move events, inside or outside of this
element, as long as the drag was started with this element under the mouse.
Useful for implementing draggable UIs ... like resizing." This affected every
`on_drag` consumer — slider, range slider, resize handle, the SplitView
divider — not just the one reported.

**The value lagged because a delta cannot know the control's length.**
`poodle_render::slider` converted pixel deltas with `units_per_px` derived from
a fixed `track_w()` of 10rem. Any slider not rendered at exactly 160px moved by
the wrong amount, and the header's control is 160px only by coincidence of the
fix below. A delta also cannot express "jump to where I clicked" at all, which
is why the track was dead.

**The track was a 6px pointer target**, with no larger grab area.

## Changes

- **`Interaction::on_scrub`** — a new vocabulary channel reporting where the
  pointer sits along a node's main axis, as a fraction (0.0..=1.0), on press and
  continuously while dragging. The backend derives it from bounds it already
  owns; the component receives a semantic value and never a coordinate, exactly
  like `DropEdge`. Documented as the channel to prefer over `on_drag` for value
  controls, because a delta forces the component to guess its own size.
- **The slider uses it**, mapping the fraction onto `min..max` through the same
  `poodle_headless::slider` machine as before. Track clicks now work because a
  press is just a fraction.
- **The scrub lives on a grab-area overlay**, not on the visible 6px track: an
  absolutely-positioned transparent child spanning the track's full width and
  reaching a thumb-radius above and below it. Width matches the track, so the
  fraction is still measured across the right element. Same reasoning as
  ResizeHandle's contract putting its grab area on an overlay.
- **`on_drag` moved to `on_drag_move`** for every consumer.
- **Header sizing**: the slider fills its host, so the header now gives it a
  160px box and a top pad that centres the track against the taller controls
  beside it.

## Two gpui Behaviours Worth Recording

- **Registering `on_drag` on an element makes gpui swallow that element's
  `on_mouse_down`.** The press handler had to move to `on_click`, which
  survives. Found by removing the drag registration and watching the down
  listener start firing.
- **A mouse-down event carries no bounds.** The press needs them to compute its
  fraction, so a zero-cost `canvas` child records the grab area's rectangle at
  paint time for the press to read. `on_drag_move` supplies its own.

Because `on_drag` swallows mouse-down, `NodeDragPhase::Start` can no longer come
from a down listener; the first move of a gesture emits it instead.

## Verification

Driven with the click driver, on the header control (track spans x 696..856):

- Click at x=816 → `0.75`, exactly the fraction clicked.
- Drag x=700 → x=850 → `0.96`; x=700 → x=760 → `0.40`. The value tracks the
  pointer 1:1 and no longer detaches.
- SplitView divider drag still works (ratio 50 → 62), confirming the `on_drag`
  rework did not break the delta consumers, and it now follows the pointer
  beyond the divider's own narrow bounds too.
- `slider`, `range-slider`, `split-view` and `resize-handle` captures differ
  only in the header band (y 258..361), by an identical 0.5939% — the chrome
  delta from this round, not a component regression.

New tests pin the scrub's placement: the track-width node carries it and the
thumb does not (a fraction measured across the thumb's few pixels would jump
wildly), and no change handler means no scrub. `poodle-render`: 110 → 112.

Green: `poodle-render` 112, node backend 5, `poodle-jetstream` 161,
`effigy check:gpui`, `effigy drift:handlers`, `git diff --check`.

## Not Done

`range_slider` still uses the delta path with the fixed `track_w()` basis. It
gains the drag-capture fix, so it no longer detaches, but it keeps the 1:1
mismatch and cannot be clicked to a position. A single scrub fraction cannot say
*which* of its two thumbs is being moved, so it needs either a per-thumb grab
area carrying its own scrub, or a channel that names the thumb.
