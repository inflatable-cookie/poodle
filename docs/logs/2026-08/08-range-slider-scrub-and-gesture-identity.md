---
title: RangeSlider joins the scrub path, and gesture drags learn whose they are
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, poodle-node, poodle-render, node-backend, range-slider, drag]
---

## The Known Gap

`Slider` moved to `on_scrub` when its drag was fixed; `range_slider` was left on
the delta path, recorded at the time as "needs either a per-thumb grab area
carrying its own scrub, or a channel that names the thumb". Two consequences:

- It converted pixel deltas with `units_per_px` derived from a **fixed 10rem**
  `track_w()`. The specimen's sliders render at **320px**, so the pointer moved
  the value at half the rate it should have.
- A delta cannot express "jump to where I pressed", so the track was inert.

## Why A Fraction Was Not Enough On Its Own

One fraction cannot say *which* thumb the pointer is moving. The answer is not
a second channel — it is that the question is only asked **once**, at the press:

`Interaction::on_scrub` now carries a `ScrubPhase` (`Press` or `Drag`). The
press picks the nearer thumb and the gesture keeps it, so a thumb dragged past
its partner clamps (the contract's existing invariant) rather than handing the
gesture over halfway. `Slider` ignores the phase — with one thumb there is
nothing to decide.

Two details the live app forced:

- gpui delivers a **click at the end of a drag** as well as for a bare press, so
  a `Press` arriving after moves is a release. Treating it as a new gesture
  would re-pick a thumb from wherever the pointer stopped and move it again.
  The component tracks that and ignores it.
- A `Drag` with no `Press` before it falls back to nearest-thumb once, so a
  swallowed press cannot strand the gesture.

`track_w()` is deleted. Nothing computes a value from an assumed width any more.

## The Bug This Uncovered

Driving the specimen, **two range sliders moved from one drag**:

```
PROBE scrub f=0.278 phase=Drag thumb=Lower live=(20.0, 80.0)
PROBE scrub f=0.278 phase=Drag thumb=Lower live=(25.0, 45.0)   <- a different slider
```

`on_drag_move` is dispatched by drag **type**, and every gesture-draggable node
registered the same unit-struct `NodeGestureDrag`. So every scrub and every
resize handle in the window heard every gesture drag. It had been invisible
because no page had previously shown two of them at once — the slider fix was
verified against a header with exactly one.

`NodeGestureDrag` now carries the originating element's id, and each listener
ignores gestures that did not begin on itself. The id comes from a per-frame
counter reset alongside `NEXT_ID`: the tree is walked in the same order every
frame, so a drag begun on one frame still recognises itself on the next.

The fix is visible in its own probe — `mine=gesture-2 from=gesture-1`, a second
draggable correctly declining a gesture that was not its own.

This affected `resize-handle` and the `split-view` divider too, not just
sliders.

## Verification

Live, on the specimen's 320px track (bounds read from the element, not measured
off a screenshot):

- Press at x=300 → fraction 0.122, **Lower** thumb chosen, value 12.
- Drag x=300 → x=500 → 0.278/0.356/0.434/0.512/0.591 producing 28/36/43/51/59.
  Exactly `fraction x 100`: 1:1 tracking, where the old basis would have moved
  twice as far.
- Only one slider responds now.
- SplitView divider still drags (104k pixels moved), proving the gesture-id
  change did not break the delta consumers.

Four new tests pin the rules that only exist in the component: the scrub is on a
full-width absolute overlay and exactly one node carries it; a press moves the
nearest thumb; a drag keeps the press's thumb across a crossing; the
end-of-drag click changes nothing.

Green: `poodle-render` 119, `poodle-node` 2, node backend 8,
`poodle-jetstream` 161, `effigy check:gpui`, `drift:handlers`,
`docs:contract-drift`, `docs:spec-drift`, `docs:lint`, `git diff --check`.

`test:native-visual` on `range-slider`, `slider`, `split-view`, `resize-handle`
and `dock-region`: **0 failing**. The change is behavioural — the grab overlay
is transparent — so no baseline moved.

## Contract

`range-slider.md` gains a **Pointer** section: press moves the nearer thumb, a
drag keeps the thumb it started with. On the web both fall out of the two
overlapping native range inputs; the Rust targets reach the same behaviour
through the grab overlay and the press decision. The old fixed-width delta is
recorded there as the reason the rule is written down.
