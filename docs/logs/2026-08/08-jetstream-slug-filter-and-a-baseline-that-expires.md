---
title: A slug filter for the Jetstream gate, and a baseline that expires at midnight
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, native-visual, jetstream, calendar]
---

## The Papercut

The Jetstream visual runner had no focused filter, so checking one component
rendered and compared all 138 specimens.

`snap` now takes `--slug=a,b`, and the runner forwards `--slug=` to it. A
filtered sweep also skips the two non-component chrome pages (`_landing`,
`_specimen-page`), because rendering those for a one-slug check is most of the
cost the filter exists to avoid.

| Run | Time |
|-----|------|
| full sweep, 138 specimens | 81s |
| `--slug=button` | **0.7s** |

## What The Full Sweep Then Showed

Running it unfiltered to prove the early return had not broken anything, **10
specimens were failing** — on a gate whose tolerance is a true zero.

Eight were mine and are improvements: `audio-player`, `video-player`, the four
`media-*`, `bulk-action-bar` and `icon` all render glyphs that used to resolve
to nothing, because the seven missing lucide icons were vendored earlier today.
`stepper` is the other thread's collapsible work, now landed. All nine
baselines refreshed.

## The Tenth Was A Real Defect

`calendar` differed by 383 pixels: the "today" border had moved from **7 to 8**.
The baseline was captured yesterday.

`poodle_render::calendar` reads `SystemTime::now()` directly to decide which day
gets the today border, so **every pixel baseline containing a Calendar expires
at midnight**. This is not a Jetstream problem: the GPUI gate has the same
latent failure, and only passes today because its baseline happened to be
refreshed this morning.

Two gates that will both fail tomorrow for a reason unrelated to any change.

### What Was Done, And What Was Not

Both gates now skip `calendar`, with the reason recorded — matching the GPUI
gate's existing `time-ago` entry, which exists for exactly this class. The
Jetstream gate had no skip list at all before; it has one now.

That is a mitigation, not a fix, and it costs the whole specimen's pixel
coverage for one border. The real fix is to make `today` injectable so a
specimen can pin it — but **Svelte reads the clock the same way**
(`todayIsoDate()`), so adding it to the Rust spec alone would be target drift in
the one artifact the contract-drift gate compares. That makes it a contract
decision across all targets rather than something to slip in here. Filed as a
papercut.

## Verification

- `jetstream visual`: 138 rendered, 137 compared, **0 failing**, 1 skipped.
- `ci:native` exits 0; `docs:lint` exits 0.
- The filtered path was checked against the full path: `--slug=button` compares
  the same specimen the full sweep does, and the full sweep still renders the
  chrome pages.

## Papercuts

The Jetstream slug-filter entry is removed. The calendar clock dependence is
added. Five remain.
