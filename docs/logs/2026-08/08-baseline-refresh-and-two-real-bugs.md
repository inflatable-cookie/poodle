---
title: Refreshing the native baselines found a panic and a missing icon set
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, native-visual, gpui, icons, node-backend]
---

## Why

Baselines had been deferred through the whole GPUI node-backend campaign — the
header chrome changed four times, so any number measured against them was
stale. With the TextInput work settled, they were the last thing outstanding.

The point of doing it carefully: a baseline refresh is the one operation that
can *silently* bake a bug in forever. So nothing was refreshed before its delta
had a cause.

## What The Deltas Actually Were

All 136 components failed. The shape was clean:

- **~78 differ only in the header band** (y258–365), at a byte-identical
  0.9156% (own tolerance) / 0.5823% (pixelmatch). That is the ThemeSelect and
  contrast slider replacing the old theme-button row, and nothing else.
- **The rest have body deltas**, and every one of them has a source file newer
  than its own baseline: baselines date from 08-06 10:19, while the node-tier
  specimen migrations ran through 08-07. Checked by mtime rather than assumed,
  because "stale" and "regressed" look identical in a diff.

Two did not fit that story, and both were real bugs.

## Bug 1 — markdown-editor Panicked

The gate reported `no screenshot within 90s`, which reads like a slow capture.
It was an abort:

```
panicked at gpui-0.2.2/src/text_system.rs:372:9:
text argument should not contain newlines
```

`shape_line` shapes exactly one line and panics on a newline. The new
`input_text` element took over **every** input node, and a markdown body is
full of newlines.

Multi-line values now fall back to the plain wrapped text child — which is what
those fields rendered before the caret existed. No caret on a multi-line field,
but no crash and no lost content. Single-line fields are unaffected.

This is a bug the visual gate caught that no unit test would have: the element
is only constructible inside a window.

## Bug 2 — The Icon Set Was Missing Seven Glyphs

`bulk-action-bar` lost its select-all button entirely. The render tier emits an
`icon_button` with `check-check` (matching Svelte, which is the parity
authority) — but `check-check.svg` was not in
`packages/gpui/preview/assets/icons/`, so it resolved to nothing.

Same class as the papercut recorded on 08-07 about six missing media transport
glyphs (`play`, `pause`, `volume-2`, `volume-x`, `maximize-2`, `minimize-2`).
`check-check` was a seventh nobody had noticed.

`lucide-static` is already vendored in `node_modules`, so all seven were copied
from source and converted to the repo's convention (single line, `stroke="white"`
for gpui to tint) rather than hand-drawn from memory.

Worth noting: the **old baseline was wrong**, not just outdated. It showed a
single ✓ where the contract says double-check. Refreshing without looking would
have replaced a wrong picture with a right one and called it a regression.

The papercut entry is removed.

## Result

135 baselines rewritten, then re-run clean to prove they reproduce: **136
compared, 2 failing**, both explained.

`markdown-editor` is back in the set now that it no longer aborts.

### picker-shell Joins The Skip List

It failed the verification pass by 0.0021% — 260 pixels in an 18x30 box, which
turned out to be a pulsing dot grid in its loading state.

Measured rather than assumed, the way the `progress` entry was: three captures
in a row drifted by 132px and 32px, and **the first and third agreed exactly**.
That is precisely how the two-agreeing-captures rule gets fooled — it blesses a
moving frame whose period happens to line up. Now skipped, with the measurement
in the comment.

## What Was Not Refreshed

`stepper` — its 7.0% delta belongs to another thread's in-flight collapsible
work, the same change currently failing `docs:spec-drift` on a documented
`defaultCollapsed` that `StepperSpec` does not have. Baking a half-finished
state into a baseline would hide that thread's real diff. Whoever lands it
should refresh it.

The gate's existing `SKIPPED` set (spinner, page-loading, time-ago,
audio-player, video-player, progress) is otherwise untouched — all animation or
wall-clock dependent.

## Concurrent Editor

Another thread is working in this repo at the same time, and its in-flight
state is what fails the doc gates right now — not this work:

- `stepper.md` documents `defaultCollapsed`, absent from `StepperSpec`
  (`docs:spec-drift`, `docs:lint`).
- `dock-region.md` documents `dragZoneId`, absent from `DockRegion.svelte`
  (`docs:contract-drift`). Both files were edited *during* this gate run —
  01:23 and 01:45 — and `stepper.rs` broke a build mid-round earlier.

Green here: `poodle-render` 115, `poodle-headless` 16, `poodle-node` 2, node
backend 8, `effigy check:gpui`, `drift:handlers`, `git diff --check`.
