# g12.014 — Native Visual Gate

**Status: active.** GPUI gate built, proven non-vacuous, and fully baselined
from verified captures — with a measured ~3% per-run flake that re-running
clears. See Baseline State. The better approach, Jetstream's
offscreen render, is blocked on the sibling repo's wgpu 30 upgrade; see Not
Done.

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

**Determinism took five wrong answers to pin down.** Recorded in order, because
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
5. **The actual cause.** The preview waits a fixed 1.5s for its first render and
   captures whatever is on screen — sometimes an incomplete frame. The
   `segmented-control` baseline proved it: its selected segment is barely
   painted there and fully filled in a fresh capture, same code, same flags.
   That one bad frame had been the reference image since the first sweep.

One mechanism explains every symptom: rotating failures, ratios that drift for
the same component, and bad frames frozen into baselines.

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

## Not Done

- **Jetstream should be the primary native gate, and is blocked.** Offscreen
  rendering is not a marginally different approach here, it is a categorically
  better one: every failure mode above is *structurally absent*. No fixed render
  delay (you control the readback), no compositor (no antialiasing jitter), no
  pointer, no window activation, no display to fall asleep, no relaunch. It is
  also one process rendering every component instead of 133 launches waiting
  1.5s each — twice over, for the stability check.

  `packages/jetstream/preview/src/bin/snap.rs` already does this and already
  routes by slug: `snap specimens` walks the component registry writing one PNG
  per slug on a headless wgpu device (`compatible_surface: None`). Adding
  baselines and diffing to it is this card's `run.ts` pointed at a different
  capture backend, and it would be CI-safe, unlike the GPUI one.

  **Blocked:** `snap.rs` does not currently compile — 15 errors, all wgpu API
  drift (`UiPass::new`, `upload_quad_geometry`, `set_bind_group` bounds). The
  sibling `jetstream` repo is mid-upgrade to wgpu 30; `snap` should build again
  once that lands, and repairing it against a moving API in the meantime would
  only conflict. Its doc comment is also stale — it claims no glyph pass, but
  text has been rendering since the pass was added.
- **One axis.** `eclipse-compact-sm` only. The web gate sweeps axes to catch
  Svelte/React divergence; here a second axis costs a full capture run and
  catches little the first would not.
- **Five of the six skips are still unproven.** Only `progress` was skipped on
  evidence. The other five are reasoned, and the two-agreeing-captures rule now
  makes checking them trivial: if a component settles, it does not belong in the
  list.
- **The 1.5s render wait is the underlying flaw.** The double capture works
  around it at 2x cost. Fixing it properly means the preview waiting for a
  settled frame rather than a fixed delay, which is a change to
  `packages/gpui/preview/src/main.rs`.
- **Cost.** Every capture now launches the preview twice, so a full sweep is
  roughly 20 minutes.
