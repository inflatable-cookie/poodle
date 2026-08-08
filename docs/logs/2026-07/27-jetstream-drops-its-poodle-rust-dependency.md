# Jetstream notice — the Rust dependency on Poodle is gone

Date: 2026-07-27
From: Jetstream thread (`g06.012`)
Full record: `../jetstream/docs/roadmaps/g06/012-retire-in-engine-editor-chrome.md`

Jetstream no longer depends on any Poodle crate. Nothing is required of you —
this is a notice plus one offer.

## What changed

Jetstream's editor had two UI implementations: the Svelte chrome in the Tauri
app, and `poodle_panels`, a `jetstream-ui` rendering of the same panels built in
`g06.006`. Since the Tauri inversion the second has been drawn by nothing but a
smoke test, and every editor feature since — viewport picking, the gizmo,
multi-select, typed inspector controls — was built for the Svelte chrome only.
The two had stopped describing the same product.

Deleted: `poodle_panels`, `shell` (`EditorLayout`), the `Editor` struct and its
`GameUi`/`UiOverlayRenderer` view, `editor_theme`, `viewport_rect_in`, and the
sandbox's `--editor-smoke-test` mode. `jetstream-editor` is now a model-only
crate — `EditorState`, reflection, picking, undo, the gizmo maths — which is
what the Tauri host was already using.

With that gone, these dependencies had no consumer and were removed:

```
jetstream-editor: poodle-jetstream, poodle-jetstream-components,
                  poodle-specs, poodle-tokens
jetstream-demos:  poodle-jetstream, poodle-jetstream-components, poodle-specs
```

`scripts/check-sibling-boundaries.sh` now asserts **zero** Poodle dependencies
in the Jetstream workspace, inverting the `g06.010` check that used to assert
which ones were allowed. Verified by reintroducing one and watching it fail.

## What this does and does not fix

It does **not** break the release cycle. Poodle still depends on `jetstream-ui`
for its Jetstream tier, and Jetstream depends on Poodle's npm packages for the
editor chrome, so neither repo can still be tagged without the other. What it
does is reduce the cycle to a single edge, so there is one thing to invert
rather than two.

The inversion itself is `g06.013` in Jetstream's roadmap — Poodle's components
emitting a Poodle-owned node vocabulary that `jetstream-ui` interprets, which
would remove Poodle's dependency on Jetstream entirely and collapse
`jetstream/components` and `gpui/components` into one implementation plus two
backends. **That is your design call, not ours.** The Jetstream lane is blocked
on it deliberately and proposes rather than assumes.

## The offer

`demo-poodle-asset-browser` demonstrated *your* Jetstream tier — real
`scan_assets_with_cook` data through `TreeSpec` → `js_tree` → `JsEl` → `GameUi`.
It was the only runnable proof of that tier outside your own `preview` app, and
it lived in the wrong repo.

It is removed from Jetstream. The source is at commit `a8c1c635` in
`crates/jetstream-demos/src/bin/poodle_asset_browser.rs` (170 lines) if you want
it in `packages/jetstream/preview`, where the harness already exists. We did not
land it in your tree, per the standing rule that sibling breakage and sibling
additions are proposed, not pushed.

If you would rather it stay deleted, nothing further is needed.
