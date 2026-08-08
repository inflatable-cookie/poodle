---
title: Both visual gates now say what they are doing
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, native-visual, tooling]
---

## The Papercut

Focused native-visual runs stayed silent for more than 90 seconds after the
batch header, so a slow capture and a hung driver looked identical.

Not hypothetical — I hit it twice today. A `markdown-editor` capture that was
actually **panicking** reported only "no screenshot within 90s", and a
background full sweep gave no sign of life until it exited.

## Cause

The GPUI runner printed progress every **tenth success**, and only on success.
A focused run of one to three slugs therefore printed nothing at all between the
header and the verdict, and even a full sweep went quiet for a minute whenever
ten components in a row passed.

The Jetstream runner was worse in a quieter way: its ~80s offscreen render is a
single `cargo run --bin snap`, spawned with `stdout: "ignore"` and `stderr:
"pipe"`. Everything the sweep said was swallowed and only surfaced if the
process failed.

## Changes

**GPUI gate** — each slug announces itself *before* its capture
(`→ 12/136 button`), which is the part that matters: a stalled slug is now
visible while it is stalling, not afterwards. Success prints the slug and its
duration (`✓ button (4.9s)`), because a component taking markedly longer than
its neighbours is the first sign of a capture going wrong and was invisible when
only failures printed. The every-tenth counter is gone, having been replaced by
something strictly more informative.

**Jetstream gate** — `snap` names each specimen as it renders, and the runner
inherits stderr instead of piping it. The render phase now streams instead of
blocking silently. The failure path no longer re-prints a captured tail, since
the output has already been shown.

## Verification

- `--slug=button,checkbox` prints four progress lines where it previously
  printed none.
- Full Jetstream sweep: 138 rendered, 137 compared, 0 failing, 1 skipped.
- `ci:native` exits 0; `docs:lint` exits 0.

## Papercut

Removed. Three remain: `effigy doctor`'s `isolation` manifest key, the calendar
baseline that expires at midnight, and the effigy snippet fix awaiting release.
