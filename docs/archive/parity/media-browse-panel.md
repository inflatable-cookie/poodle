<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- pass 18: GPUI rebuilt — composed real MediaThumbnail (square, compact) + Callout
     (danger error) + Button (load-more) instead of fakes; grid min-columns corrected
     (lg 12.5→12, xl 14→13); dropped ad-hoc alpha math (item bg now color-mix(panel 92%));
     added hover/focus states + caption-size meta font; items focusable/clickable.
     Build clean. Remaining GPUI: onSelect callback (preview-loop) + thumbnail image URL. -->
<!-- pass 41: meta-font bug fixed on BOTH targets — meta/state copy now resolves the new
     `MediaBrowsePanelSpec::meta_font_token()` (= typography.label.size, 13px = 0.8125rem).
     The old GPUI `typography.caption.size` was UNMAPPED in the GPUI adapter → resolved to
     0px (invisible meta); Jetstream had a `rem_to_px(0.8125)` literal. Item border/bg/
     radius/hover-bg/focus-border now route through additive spec token methods on both
     targets (no inline token strings). Jetstream: lg/xl min-column 12.5/14→12/13rem;
     grid + actions rows gained the contract `margin-top = grid-gap`. Jetstream gains a
     7-test render_probe module (states, load-more switch, meta=13px guard, lg min-column).
     contracts test + gpui build + jetstream media_browse_panel tests all pass. -->
<!-- specimen note: GPUI specimen done (added Loading-more group; browse/loading/error/empty
     already present; real MediaBrowsePanel only, no fakes; gpui/preview builds 0 errors);
     Jetstream pending engine recovery. specimen=gap held — Jetstream half unverifiable while
     engine is build-blocked. -->
<!-- specimen flip: engine now builds. Jetstream specimen aligned to GPUI coverage — browse grid
     + load-more, loading/error/empty states, loading-more, semantic compact+prominent; real
     js_media_browse_panel (composes real js_media_thumbnail/js_callout/js_button), no fakes.
     Both previews build 0 errors. specimen=gap → ok. -->
# Parity: MediaBrowsePanel

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/media-browse-panel.md`
- Svelte (authoritative): `packages/svelte/components/src/MediaBrowsePanel.svelte`
- GPUI: `packages/gpui/components/src/composites/media_browse_panel.rs`
- Jetstream: `packages/jetstream/components/src/media_browse_panel.rs`
- Spec: `packages/contracts/components/src/media_browse_panel.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MediaBrowsePanelSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/media_browse_panel_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/media_browse_panel.rs`

## Contract ↔ Svelte

Aligned. Svelte matches the contract on props, anatomy, states, and token tables.

- Props: `loading`/`error`/`items`/`hasMore`/`emptyMessage`/`loadMoreLabel`/`size`/`sizeRole`/`density` all present with matching defaults (Svelte `lines 29-41`). `sizeRole` default `"control"` matches contract §3. OK.
- Size table: Svelte grid-min `xs 8.5 / sm 10 / md 11 / lg 12 / xl 13` (`lines 116-130`) matches contract §8 exactly. OK.
- Density table: Svelte grid-gap/item-gap/item-pad (`lines 132-142`) match contract §8 (`compact 0.375/0.25/0.5`, default `stack-sm/0.375/0.75`, comfortable `0.75/0.5/0.875`). OK.
- States: loading copy "Loading media..." (`line 56`), error via `Callout tone="danger" announceMode="polite"` (`line 59`), empty message, ready grid, load-more "Loading..." switch (`line 97`) all match contract §4. OK.
- Anatomy: item is a real `<button type="button">` with `MediaThumbnail` (compact, square) + label + optional meta (`lines 67-90`). OK.

## GPUI gap (vs Svelte + contract)

- [x] DONE Real `MediaThumbnail` — grid items compose `MediaThumbnail::from_spec` (compact = no caption, square aspect ratio) per contract §2/§7. No placeholder rectangle.
- [x] DONE Real `Callout` for error — error state composes `Callout::from_spec` (danger tone) per contract §2/§6.
- [x] DONE Real `Button` for load-more — `Button::from_spec` (secondary variant, disabled while loading, label switches to "Loading...").
- [x] DONE Grid min-column lg/xl = 12/13rem (pass 18). Verified against contract §8.
- [x] DONE Items focusable — `div().id(...).focusable()` with hover/focus styling (onSelect click handler remains a preview-loop concern; component takes no callback, which is correct for GPUI).
- [x] DONE Item colors via `color_mix(... transparent)` — item bg `color-mix(panel 92%)`, hover bg `color-mix(elevated 90%)`; border full-alpha `border-subtle`. No ad-hoc `* 0.5` alpha math. As of pass 41 these resolve through additive `MediaBrowsePanelSpec` token methods (`item_bg_token`/`item_hover_bg_token`/`item_border_token`/`item_focus_border_token`/`item_radius_token`).
- [x] DONE Hover/focus → `border-focus` + elevated-90% bg (pass 18).
- [x] DONE Meta font (pass 41) — was `typography.caption.size`, which is UNMAPPED in the GPUI adapter and resolved to **0px** (invisible meta). Now `spec.meta_font_token()` = `typography.label.size` = 13px = contract-exact `0.8125rem`.
- accepted (Tier-3): grid is `flex_wrap`, not CSS auto-fill grid — layout freedom; items lack a max so very wide screens differ. Note only.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [x] DONE Grid min-column lg/xl — fixed `12.5/14.0` → `12.0/13.0` (contract §8). render_probe test floors a single item at the lg viewport and asserts 192px (and NOT the stale 200px).
- [x] DONE Meta font — `rem_to_px(0.8125)` literal replaced by `resolve_px(theme, spec.meta_font_token())` (= `typography.label.size` = 13px). render_probe test guards meta text-size = 13px.
- [x] DONE Item border/radius/bg now route through the additive spec token methods (`item_border_token`/`item_radius_token`/`item_bg_token`); item bg is `tint(panel_bg, 0.92)`. NOTE: `tint` mixes toward transparent (alpha-scaled), matching `color-mix(panel 92%, transparent)`.
- [x] DONE Actions + grid rows gained the contract `margin-top = grid-gap` (contract §8 Grid + Actions).
- accepted: ARIA channel absent; item `onSelect` interaction belongs in preview event loop (no select wiring in component, which is correct for Jetstream).

## Specimen parity

- Svelte covers: Browse grid (+load more), Loading, Error, Empty (`MediaBrowsePanelSpecimen.svelte` per contract §12).
- GPUI covers: Browse panel (+load more), States (loading/error/empty), **Loading more** (items + loading + has_more → disabled "Loading..." Button), Semantic presentation (size+density, sizeRole prominent). **GPUI specimen done** — composes the real `MediaBrowsePanel` (which itself wraps real `MediaThumbnail`/`Callout`/`Button`); no fakes. Full contract state coverage. Jetstream pending engine recovery.
- Jetstream covers: With items, Loading more, Loading, Error, Empty, Sizes (sm/md/lg), Densities (compact/default/comfortable) — broadest coverage; uses real MediaThumbnail/Callout/Button. — missing: nothing; hardcoded `text_size(11.0)` group labels (specimen-local).

## Notes

- GPUI now composes the real `MediaBrowsePanel` component (rebuilt in pass 18), which internally uses real `MediaThumbnail` (compact, square), `Callout` (danger), and `Button` (secondary, "Loading..." while loading-more). The old "bare-div fakes" critique is resolved — the specimen has no hand-rolled geometry; it only drives spec props. Specimen audited: zero fakes.
- Jetstream correctly composes `js_media_thumbnail`, `js_callout`, `js_button` — the right pattern; remaining gaps are token-resolution polish.
- Spec field `thumbnail_url` exists on `MediaBrowseItem` but neither Rust target renders an actual image (both show the kind-based thumbnail shell only). Svelte renders an `<img>` when `thumbnailUrl` is present (contract §2 image part). Image rendering is a shared gap but lower priority (specimens pass no URLs). Flag low.
- The `lg=12.5 / xl=14.0` min-column bug is identical in both Rust targets — likely copy-paste; fix both.
