---
title: The GPUI preview's contrast control is a Slider, not four preset buttons
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, gpui, preview, slider]
---

## What Changed

The GPUI preview header drove neutral contrast through a toggle group of four
preset stops (`0.4`, `0.5`, `0.75`, `1`). The axis is continuous, and both other
targets already treat it that way — the web preview uses a range input, and the
Jetstream shell an engine slider over `0.0..=1.0`. GPUI was the odd one out, and
the four presets could not express any value between them.

It is now a real Poodle `Slider` — the node-backed component, not a bespoke
control — with a live value readout beside the caption.

- `ContrastStop` is gone. `AppState::contrast` is an `f32` with
  `CONTRAST_MIN` / `CONTRAST_MAX` / `CONTRAST_DEFAULT` (0.5, the midpoint the
  tokens are authored against) replacing the four-variant enum.
- Slider changes route through the same context-free node event queue as the
  rest of the chrome, via a new `ChromeEvent::Contrast(f32)` that clamps and
  rebuilds the theme.
- The toggle-group dispatch lost its now-unreachable `"contrast"` branch.

## One Real Trap

`SliderSpec`'s default `step` is `1.0`. Over a `0.0..=1.0` range that means the
value can only ever snap to an endpoint — the first drag jumped straight from
0.50 to 1.00 and small drags did nothing at all. The spec needs an explicit fine
step (`0.01`) for any range narrower than the default step. Worth remembering for
every other normalised-range slider.

## Verification

Driven with the click driver:

- Default renders at `0.50` with the accent fill and thumb at the midpoint.
- A drag right with the default step jumped to `1.00` (the bug above); with
  `step: 0.01` the same drag lands at `0.54`, i.e. continuous and proportional.
- The value clamps at the bounds.

The synthesized drag emits far fewer move events than a real mouse, so a short
driver drag moves the value only slightly — the same driver artifact recorded in
`PAPERCUTS.md` for clicks, not a component issue.

The header is also visibly tighter: dropping four buttons for one slider let the
SEARCH group back into view at this width.

Green: `poodle-render` 110, GPUI preview build, `effigy drift:handlers`,
`git diff --check`. No `ContrastStop` reference remains in the workspace.
