---
title: GPUI node backend waves 41–45
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, g12.019, gpui, batch-b]
---

## What Landed

Five waves of `g12.019` Batch B, continuing the handoff at
[`07-175745-gpui-node-backend-batch-b-handoff.md`](07-175745-gpui-node-backend-batch-b-handoff.md).
Per-wave proof lives in
[`docs/roadmaps/g12/019-gpui-node-backend.md`](../../roadmaps/g12/019-gpui-node-backend.md);
this is the batch summary.

- **Wave 41** — AudioPlayer, VideoPlayer, TimeAgo onto the node backend;
  IconProvider and UiPresentationProvider moved to a new preview-local
  `providers.rs` (both are pure context boundaries with no `poodle-render`
  recipe to move to, and they outlive the old tier).
- **Wave 42** — SplitView and DockRegion.
- **Wave 43** — the preview shell itself: `main.rs`, `token_view.rs`,
  `usage_docs_view.rs`.
- **Wave 44** — BlockEditor, via an additive consumer-slot path in
  `poodle-render`.
- **Wave 45** — LogList, including the entries payload its contract has
  documented all along.

## Audit Findings That Changed The Plan

The handoff's picture of what remained was accurate about the parked component
names but wrong about why three of them were parked, and it missed one whole
category:

1. **The constructor census was specimen-scoped.** Four non-specimen preview
   files still constructed the old tier and appeared nowhere in the roadmap:
   `main.rs`, `token_view.rs`, `usage_docs_view.rs` and `demo_view.rs`. Batch C
   cannot delete the tier while those stand. Wave 43 migrated the first three.
2. **`demo_view.rs` is dead source.** No `mod demo_view;` exists and the
   `app_state::DemoScreen` type it needs is gone, so it has not compiled since
   an earlier refactor. Its 26 old-tier call sites are Batch C deletion work,
   not migration work.
3. **AudioPlayer, VideoPlayer and TimeAgo were not blocked.** `poodle-render`
   already had plain `(spec, theme) -> Node` recipes for all three. They were
   parked because the *native gate skips their slugs as non-deterministic*,
   which never blocked the constructor migration.
4. **IconProvider and UiPresentationProvider render nothing.** Both are
   `with_child` passthroughs. No shared provider contract was needed to remove
   the imports.
5. **SplitView and DockRegion already had slot-shaped signatures** in
   `poodle-render` — `primary`/`secondary` Nodes, a `content` Node, and handler
   structs. The "needs slot/host-event design" note was stale.

## Shared Recipe Defects Found And Fixed

Migrating these exposed real divergence in `poodle-render`, not just plumbing.
Per the roadmap's Decision Log, the old GPUI tier is the recipe reference and
`poodle-render` was reconciled to it (cross-checked against Svelte and the
contract, both of which agreed):

- **AudioPlayer** — the seek track carried `self_stretch`, painting as a
  full-height pill instead of a 0.25rem rail, and neither slider set the
  `text_color` channel the backend reads for a Progress fill, so both tracks
  rendered white with no accent. Also corrected to contract: 0.125rem track
  radius, label-size centred time labels, the component's own density ladders
  for `pad-y`/`gap` and `pad-x`, and the missing transport hover tint.
- **VideoPlayer** — no minimum chrome height and no bottom-pinned controls, so
  the viewport collapsed to zero and the controls escaped the black surface.
  Old-tier delta went from 1.4999% to 0.0152%.
- **SplitView** — root had width only, so the split collapsed to content
  height; panes were allocated by grow-against-zero-basis, which spreads the
  divider's thickness across both panes and moves the split.
- **DockRegion** — the expanded layout ignored edge placement entirely (a
  left-edge dock laid its tabs in a row above the body); the border boxed all
  four sides instead of ruling the docked edge; the active tab wore a
  TabStrip-style underline instead of the contract's accent pill; and the
  icon/label pair was interpolated into one string, collapsing the gap.
- **LogList** — `render/src/log_list.rs` drew the literal string `"{n} entries"`
  where rows belong, because `LogListSpec` carried only `entry_count`.

## Vocabulary Additions

All additive; `poodle-node` still names no backend.

- `flex_basis_pct` — flex-basis as a fraction of the parent. The existing
  `flex_basis` is pixels only, and a ratio-allocated pane must seed at a share
  of the container and then shrink for its siblings.
- `text_italic` — the vocabulary could not express `font-style: italic`, which
  five contracts' CSS calls for and which `block_editor.rs` was working around
  with a documented "italic-substitute" tone change.
- `VideoPlayerSpec::with_captions_src` / `with_show_captions` /
  `renders_captions_track()` — replacing the old tier's ad-hoc element builder.
- The LogList entry types (`LogLevel`, `LogActor`, `StreamLogEntry`,
  `AuditLogEntry`, `LogEntry`) and `LogListSpec::entries`.

## Old-Tier Inventions Dropped

Each had no contract or Svelte counterpart:

- VideoPlayer's extra "subtitles" control button. Contract §2 renders captions
  as a `<track>`, which carries no chrome.
- LogList's `Debug` level. The contract and Svelte both define exactly
  info/warn/error.
- LogList's flattened actor/resource fields, now the contract's nested
  `LogActor` and audit resource fields.

## Validation

Green: `effigy gpui:build`, `effigy drift:handlers`, `effigy docs:spec-drift`,
`git diff --check`. Tests — `poodle-render` 98, `poodle-specs` 227,
`poodle-gpui-node-backend` 5, `poodle-gpui` 133.

Pixel-verified against existing baselines, no baseline changed:

- `split-view` — exact
- `icon-provider`, `ui-presentation-provider` — exact
- `dock-region` — 0.4194% → 0.0415%, the remainder being the specimen's own
  caller-authored body text in a deliberately overflowing demo cell
- `time-ago` — 0 differing pixels against a direct old-tier capture
- `video-player` — 1.4999% → 0.0152% against a direct old-tier capture
- `audio-player` — reconciled and eyeballed against a direct old-tier capture

## Open Risk: Waves 43–45 Are Not Pixel-Verified

The display went to sleep partway through the batch. `screencapture` can still
read the framebuffer (it returns black) but cannot read a window, so the native
gate reports `could not create image from window` and stops. `caffeinate -u`
does not wake a sleeping display; that needs physical input.

Wave 43 is the one to watch: the preview shell's nav, sidebar, search and code
blocks appear in **every** baseline, so if that migration shifted anything, all
136 slugs move together. Waves 44 and 45 are specimen-local by comparison.

A full `bun test/native-visual/run.ts` run is the outstanding gate. It aborted
after 14 components on the display failure; all 14 matched their documented
residuals exactly, with no new regressions among them.

## Next Task

1. With the display awake, run `bun test/native-visual/run.ts` and reconcile.
2. Wave 46 (Tree) is the last in-scope specimen and is parked on four named
   interaction-vocabulary gaps — see the roadmap. The fourth (reporting a drop
   target id) inverts the vocabulary's documented delta-only principle and is an
   operator decision.
3. Batch C then needs the `poodle-gpui-components` dependency dropped, the
   directory deleted (including the dead `demo_view.rs`), the mined probe tests
   ported, the deletion logged, and the one-line Jetstream g06.013 pointer.
   Its precondition is the gate above.

## Papercut Recorded

The GPUI preview's icon set has no media transport glyphs, so `play`, `pause`,
`volume-2`, `volume-x`, `maximize-2` and `minimize-2` resolve to nothing in
AudioPlayer and VideoPlayer. Both the old tier and the node backend render the
empty button box, so this predates the migration.
