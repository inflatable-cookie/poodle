# g12.014 — Native Visual Gate

**Status: complete.** Two native visual gates, and the comparison between them
is the finding: Jetstream's headless offscreen render does in 90 seconds with
zero flake what GPUI's window capture does in 20 minutes with 3%.

## Problem

`g12.009` built a cross-framework pixel diff for the web because the structural
gates could not see a component render at the wrong size — the ListCard
`data-size` bug proved it. The native targets were left in exactly that
pre-009 state, and `g12.013` made the cost concrete: that card changed native
rendering in several places (the Pagination chrome default flip most of all) and
none of it could be shown, only argued.

## The Correction That Made This Possible

The plan of record said GPUI could not be run here — memory note
`rust-previews-build-verified-only` had it as build-only. That was wrong.
`packages/gpui/preview` already accepted `--screenshot <path>`: it opens its
window, waits for first render, finds itself by PID through a `swift -e`
CoreGraphics lookup, shells out to `screencapture -x -l <wid>`, and exits.

Tested rather than assumed, it produced a real 2696×2396 PNG of the fully
rendered Button page on the first try.

The same note had already been wrong once before in the same direction — it
claimed Jetstream was unverifiable until someone tried and found it was ~130
lines of existing pieces. Both times the pessimistic claim survived because
nobody tested it. The note now says so.

## Why This Gate Is Shaped Differently

The web gate needs no committed baselines: Svelte and React emit the same DOM
from the same stylesheet, so any difference between them is a bug by
construction.

The native targets have no twin. GPUI is a different renderer with its own
shell, font stack and compositor — diffing it against Svelte would be all noise
and would fail on the first run for reasons no one could act on. So this is a
**baseline** gate: capture, commit, diff on the next change. It answers "did
this edit move native rendering?" rather than "do two targets agree?".

**Determinism took six wrong answers, and is still not pinned down.** The sixth
is below, under "The Settled-Frame Wait". Recorded in order, because
the sequence is the lesson:

1. *"Captures are bit-identical."* One component launched twice gave zero
   differing pixels. Tolerance set to `0` on n=1.
2. *"There is an antialiasing floor."* A sweep found four components differing
   by 1–14 pixels along glyph edges, so the tolerance was raised to sit between
   that floor and a real change.
3. *"`order-by` is slow."* It timed out at 40s three runs running, so the limit
   went to 90s. The actual cause was relaunching immediately after the previous
   window closed; a 1s pause fixed it, and the raised limit was treating a
   symptom.
4. *"The pointer contaminates captures."* Three components showed hover-shaped
   diffs, so a `parkCursor()` mouse-warp was added and declared the fix. Direct
   testing then showed cursor position makes **no** difference to the render —
   and bisection showed the warp itself was causing two *other* components to
   differ. A regression shipped inside a fix. Removed.
5. *"The actual cause."* The preview waits a fixed 1.5s for its first render and
   captures whatever is on screen — sometimes an incomplete frame. The
   `segmented-control` baseline proved it: its selected segment is barely
   painted there and fully filled in a fresh capture, same code, same flags.
   That one bad frame had been the reference image since the first sweep.
6. *"So wait for a settled frame instead."* Built, measured, and the flake did
   not move — 2 in 90 against 3 in 150 before. Detailed below.

One mechanism *appeared* to explain every symptom — rotating failures, ratios
that drift for the same component, bad frames frozen into baselines — and
removing that mechanism left the symptoms in place. The incomplete frames were
real; they were not the whole story.

**The fix is self-verifying rather than something to be right about.** A capture
is accepted only when two consecutive attempts produce identical bytes
(`captureSlugStable`). Raising the tolerance until bad frames passed would have
let real regressions through with them.

It earned its keep immediately by separating two things that had been conflated:
`segmented-control`, `nav-card` and `file-upload` settle on retry, while
`progress` never does — three capture pairs, none agreeing. That one is
genuinely non-deterministic and is now skipped with a *measured* reason, unlike
the other five which are skipped on reasoning.

## What Shipped

- `test/native-visual/config.ts` — axis, paths, the skip list with reasons, and
  the slug list parsed from the GPUI preview's own component registry
- `test/native-visual/capture.ts` — `captureSlugStable` (two agreeing captures)
  over `captureSlug` (one subprocess; waits for the file to appear *and its size
  to settle*, since the file exists before `screencapture` has flushed it)
- a 1s pause between captures, and a stop after three consecutive capture
  failures with a message naming the display session as the cause
- `test/native-visual/run.ts` — diff, `--update`, `--slug=`, diffs written to
  `test/native-visual/out/`
- `effigy test:native-visual` / `effigy native-visual:update`
- `test/native-visual/README.md`, including the rule that a baseline update must
  land in the same commit as the change that caused it — a baseline updated on
  its own is indistinguishable from a regression waved through

**Confirmed non-vacuous.** Adding 3px to the GPUI button height failed the gate
at 1.72% of pixels, and the diff image showed exactly what moved: every button
grew and everything below it shifted down. Reverted; green again.

## Baseline State

All **133 baselines have been rebuilt** through the two-agreeing-captures path,
in five chunks, every chunk clean. The set no longer contains images of unknown
provenance.

**A residual flake remains: roughly one component per thirty, per run.** Measured
over five verification passes — chunks of 30 came back with 0, 0, 1, 1 and 1
failures. The failures are not reproducible: `media-browse-panel` failed once and
then passed five consecutive re-runs.

So the double capture reduced the flake substantially but did not eliminate it —
both probes can land in the same incomplete state and agree with each other. The
honest reading is that **a green run is meaningful and a single red component is
not**; re-run it before believing it.

This is the strongest argument for the offscreen approach below. Three rounds of
fixes have each reduced the flake without reaching zero, because the underlying
problem — reading pixels off a live compositor at a guessed moment — cannot be
fully solved from the outside.

**Speed.** Captures went from ~118s to ~15s per component when the harness
stopped invoking `cargo run` per capture (which re-checks freshness every time)
and started building once, then exec'ing the binary. A full rebuild is ~20
minutes rather than the several hours that implied.

## Local-Only

The capture needs a live macOS window-server session, so this is not in
`ci:web` or `ci:native` — the same constraint as `check:jetstream`, which needs
the sibling runtime repo. Run it before and after any change to `poodle-specs`
accessors or the GPUI component crate.

## The Jetstream Gate — And Why It Wins

Once the sibling repo's wgpu 30 upgrade landed, `snap.rs` needed three small
things, none of them the API rewrite the error count suggested:

- **`wgpu = "29"` → `"30"`** in `packages/jetstream/preview/Cargo.toml`. Thirteen
  of the sixteen errors were a *duplicate wgpu in the graph*, not changed APIs —
  they report as `expected wgpu::Device, found wgpu::Device`, which reads as
  nonsense until you notice the two versions.
- `RequestAdapterOptions` gained `apply_limit_buckets`
- `get_mapped_range()` became fallible

Then it worked, and the measurements are not close:

| | GPUI (window capture) | Jetstream (offscreen) |
|-|-----------------------|-----------------------|
| full sweep | ~20 min | **90 s** |
| flake | ~3% per run | **0** |
| determinism | 2 agreeing captures needed | 135/135 bit-identical across two sweeps |
| tolerance | 0.002%, a measured noise floor | **true zero** |
| requires | awake, unlocked display | nothing |

Every failure mode the GPUI gate fought is *structurally absent*: no compositor
means no antialiasing jitter, no window means no hover or activation state, and
reading back a texture you rendered means no guessed moment and no incomplete
frames. The five wrong answers above were all consequences of one decision —
reading pixels off a live desktop — and none of them can occur here.

**Confirmed non-vacuous.** Widening the Jetstream pill by 4px failed 9 specimens,
`pill` itself at 3.46% and `token-input` at 1.37% — the latter because it
composes pills, which is exactly the cross-component reach a visual gate exists
to catch. Reverted; 135/135 green again.

`effigy test:jetstream-visual` / `effigy jetstream-visual:update`. Local-only,
but only because it needs the sibling `jetstream` repo — nothing about it wants
a display, so it would be CI-safe wherever that repo is available.

## Baselines Are Not Committed

Both baseline directories are gitignored. The GPUI set is 103MB — 2696x2396
full-window screenshots including the sidebar and theme picker in every frame —
against 7.9MB for Jetstream's 900x640 component renders. Committing them meant
paying that on every clone forever, and paying it again on every rebaseline,
for the *less* trustworthy of the two gates.

Both runners write a missing baseline on first run and say so, so a clone
self-populates. The trade is that a gate now compares against the machine's own
last capture rather than a shared reference — it answers "did my change move the
render?" and not "does this branch match main". Given both gates are local-only
regardless (a display for GPUI, the sibling repo for Jetstream), that is the
question they were already answering.

## Which Gate To Trust

**Jetstream.** It is faster, deterministic, and needs no environment. The GPUI
gate stays because GPUI is a separate renderer that can regress independently,
but it is a developer convenience: a green run means something, a single red
component means re-run it.

## The Settled-Frame Wait — Attempt Six, And It Did Not Work

The section above named the real fix: stop guessing, wait for the renderer to
report. That is now implemented, and **it did not reduce the flake.**

`packages/gpui/preview/src/main.rs` no longer sleeps 1.5s. It chains
`Window::on_next_frame` three deep (`schedule_frames_drawn`), sets an atomic,
and the capture thread polls that with a 20s deadline fallback.

Two things were learned building it, both by testing rather than reasoning:

- **Frames-drawn is not readiness.** Three frames land in about 50ms, and a
  capture that early comes back 1348x1478 instead of 2696x2396 — painted, but
  not yet on the Retina backing store. So a `MIN_SETTLE` floor of 900ms is kept
  *alongside* the frame signal, and both must hold. The old fixed sleep was
  masking window **setup**, not only paint, which is not what attempt five
  claimed.
- **Sixth diagnosis, same result.** Measured over three passes of the same 30
  components: 2, 0, 0 failures — **2 in 90**. The five passes before the change
  gave 0, 0, 1, 1, 1 — **3 in 150**. Those are the same number. And of the two
  failures, one (`confirm-action`) was `could not create image from window`,
  a window-server refusal that no amount of frame-waiting addresses; the other
  (`context-menu`, 1.34% of pixels) passed three consecutive re-runs.

**Kept anyway, for a reason that is not flake.** Captures went from ~15s to
~5.3s per component (30 components in 2m40s), and waiting on a signal the
renderer actually emits is defensible in a way that a tuned magic number is not
— attempt five had real evidence that the 1.5s sleep froze an incomplete
`segmented-control` frame into a baseline. It is a better mechanism that does
not solve the stated problem.

The conclusion the table above already reached is now also the empirical one:
**reading pixels off a live compositor cannot be made deterministic from the
outside.** Six attempts, three of which reduced the flake and none of which
removed it. Offscreen rendering is the answer, and Jetstream has it.

## GPUI Baselines Are Display-Dependent

Every GPUI baseline failed on size after the machine's display setup changed:
`1348x1478` against a `2696x2396` baseline — a 1x capture where the baseline was
2x. Confirmed as environmental by building the **unmodified HEAD binary** and
getting the identical size, which is the only reason it was not misread as a
seventh regression in the wait work.

There is nothing to fix: the capture is whatever the window server hands over on
whichever display the window opened. Since the baselines are gitignored and each
machine compares against its own last capture, this is a rebaseline, not a
defect. It is one more failure mode the offscreen Jetstream gate does not have,
and it is worth knowing before assuming a wall of size mismatches means someone
broke layout.

## Not Done

- **GPUI's ~2% flake is unfixed, and now demonstrated to be unfixable from
  outside.** See above — six attempts.
- **One axis** on both gates: `eclipse-compact-sm`.
- **Five of GPUI's six skips are unproven** — only `progress` was skipped on
  evidence. Jetstream needs no skip list at all, which is its own signal.
