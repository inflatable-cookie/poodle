# `gpui-unofficial` Adoption Gates

Status: open — spike `g16.110` reports "adopt later"; three gates named,
two need an operator decision or an upstream fix
Captured: 2026-09-05
Owner: Chatterbox (planning)
Source: `../logs/2026-09/20260905-g16-110-gpui-unofficial-spike.md` (PR
#214), plus direct verification of the published crates on 2026-09-05

## What the spike proved

- AccessKit maps from the existing `poodle-node` accessibility record with
  no vocabulary change (`node-backend/src/a11y.rs`, 151 lines; roles,
  toggled state, click action registered). The API is the right target.
- The compile delta on the node backend is mechanical (rename table in the
  report). `Application::new()` is gone; the live preview needs the
  `gpui-platform` facade.
- Headless boards: `check:gpui` and `gpui:test` pass; `regressions:native`
  201/204 with three focus and overlay restore failures to port; consumer
  identity gate re-points cleanly. No GPL crate resolves at 1.19.0-pre.

## Gates

1. **`gpui-apple` cannot build from crates.io.** Its `build.rs:103` looks for
   the sibling gpui source at `CARGO_MANIFEST_DIR/../gpui-unofficial` to
   cbindgen shader types; cargo extracts the sibling as
   `gpui-unofficial-1.19.0-pre`, so the path never exists. The maintainer's
   2026-09-02 fix (#299) only renamed the path; the published 1.19.0-pre
   still carries it. This is a republish-pipeline defect: the crate must
   embed the headers it needs, or resolve the sibling through cargo
   metadata. Known: headless paths do not need `gpui-apple`; the live
   preview and the window-capture binary do. Next check: whether a later
   republish fixes it; otherwise an issue on `iamnbutler/gpui-unofficial`
   (outward action, operator to authorise; draft below).
2. **`bzip2-1.0.6` licence.** `libbz2-rs-sys` (via `http-client-gpui-unofficial`,
   a required dependency) carries the permissive bzip2 licence, which is not
   on `deny.toml`'s explicit allow list. Not GPL. Decision: add
   `bzip2-1.0.6` to the allow list after review (operator), or reject the
   route. No exception was added by the spike.
3. **Headless AccessKit tree read is a no-op.** `TestWindow::a11y_init` is
   the trait no-op and `debug_a11y_tree_json()` returns `None` in the
   in-memory test platform, so the A2 platform-tree proof cannot execute
   headlessly. Next check: the non-activating live window path (the same
   one window capture uses) can host the tree read; that is also the shape
   `effigy test:jetstream-ax` uses. Design that into the migration card.

## Also to port

Three `regressions:native` failures on the spike (nested popover trigger
restore, Select two-instance close restore, Tabs tooltip cancel on hover
removal) and the specimen-probe wall-clock budget (known papercut, fixed by
`g16.107`).

## Draft issue for `iamnbutler/gpui-unofficial` (not filed)

Title: `gpui-apple-gpui-unofficial` cannot build from crates.io: build.rs
expects sibling `../gpui-unofficial`. Body: published 1.19.0-pre
`build.rs:103` resolves `CARGO_MANIFEST_DIR/../gpui-unofficial`; a crates.io
extraction places the sibling at `gpui-unofficial-<version>`, so the metal
shader stitch fails before compile. Suggest the transform copy the needed
`src/{scene,geometry,color,window,platform}.rs` into the apple crate or
locate the sibling via `cargo metadata`. Repro: `cargo check` on a crate
depending on `gpui-platform-gpui-unofficial = "1.19.0-pre"` on macOS.

## Next check

When gate 1 has a published fix and the operator has decided gate 2,
compile the migration card (estimate from the spike: one substantial native
lane; 1–2 days mechanical port plus overlay/focus behaviour and the live
`Application` restore). Until then, A2 stays "route chosen, blocked
upstream"; A1 proceeds. Remove this note when the migration card is
compiled or the route is rejected.
