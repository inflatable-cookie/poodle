---
title: The click driver now holds the button down across repaints
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, gpui, click-driver, testing]
---

## The Problem

Recorded as a papercut on 2026-08-07: `--click` posted mouse-down and mouse-up
inside a single frame, so it could not catch a bug that only appears when a
press spans frames — which every real click does.

The evidence for that was concrete. The node-backend's id-stability bug (element
ids drifting per frame, so state keyed on the press was lost) passed this driver
in **both** the broken and the fixed states. A driver that agrees with itself
either way is worse than no driver: it produces a green result that means
nothing.

## The Fix

`dispatch_click` is split into `dispatch_press` and `dispatch_release`, with the
hold awaited **outside** `cx.update` so the window rebuilds while the button is
down. Default 120ms; `--hold 0` restores the old single-frame behaviour for runs
that only want speed.

Non-zero by default deliberately. The failure this exists to catch is invisible
at zero, and a correctness tool whose sharp setting is opt-in mostly runs blunt.

The driver still does not chain frames — the existing design note explains why
(gpui stops a window's display link the moment macOS reports it occluded, and a
script-launched preview usually is, so a frame-chained driver deadlocks). A
timer-based hold gets the same property without that risk.

## Measured, Not Asserted

A frame counter incremented in the render pass, read at press and release:

| Setting | Press | Release | Rebuilds under the button |
|---------|-------|---------|---------------------------|
| default (120ms) | frame 18 | frame 22 | **4** |
| `--hold 0` | frame 17 | frame 17 | 0 |

## An Honest Negative

I tried to prove the fix by reintroducing the original id-stability bug and
watching the driver catch it. It did not — the click still worked, under both
settings.

The reason is that the bug no longer reproduces on these targets: the fields and
sidebar items it originally broke have since been given **stable explicit ids**,
so a drifting anonymous-id counter cannot affect them. The regression test I
wanted is not available, because the code moved out from under it.

So the property is demonstrated directly (the table above) rather than through
the bug it was meant to catch. Worth stating plainly: this proves a press now
outlives repaints, not that it would have caught that specific historical bug.

## Regressions Checked

A longer press is exactly the kind of change that breaks click handling, so the
paths that depend on down and up pairing were re-driven:

- Field focus and typing — `abc` lands with the caret after it.
- Double-click word-select, `cmd-x`, `cmd-v` — round-trips to `helloworld`.
- **Slider track press** — the riskiest, because it uses `on_click`, which needs
  down and up on the same element. The contrast slider still jumps to the
  clicked fraction.
- `ci:native` exits 0.

## Papercut Closed

Removed. Four remain, none blocking a gate.
