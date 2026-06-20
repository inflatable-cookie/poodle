<!-- parity consv=ok gpui=9 jetstream=4 specimen=gap -->
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

- [ ] Does NOT use `MediaThumbnail` — renders a bare placeholder rectangle `div().min_h(px(rem_to_px(6.0)))` with tinted elevated bg (`media_browse_panel.rs:134-139`). Contract §2 requires the `MediaThumbnail` part (compact, square). This is a fake/mockup shell — replace with the real `MediaThumbnail` composite.
- [ ] Does NOT use `Callout` for error — renders plain centered danger-colored text (`media_browse_panel.rs:90-100`). Contract §2/§6 require the `Callout` primitive (danger tone, `announceMode="polite"`). Use real Callout.
- [ ] Does NOT use `Button` for load-more — hand-builds a div with border + text (`media_browse_panel.rs:172-180`), hardcoded `.px(px(rem_to_px(0.75))).py(px(rem_to_px(0.375)))`. Contract §2 load-more is the `Button` primitive (secondary variant, disabled while loading). Use real Button.
- [ ] Grid min-column wrong for lg/xl — `Lg => 12.5`, `Xl => 14.0` at `media_browse_panel.rs:64-65`; contract §8 says `lg 12rem`, `xl 13rem`. Fix the match arms to 12.0 / 13.0.
- [ ] Items are not clickable buttons — rendered as plain `div`, no `on_click`, no `onSelect` callback wired (`media_browse_panel.rs:116-153`). Contract §5 `onSelect` + §6 real `<button>` semantics unmet. Add an id + click handler builder.
- [ ] Hardcoded alpha multipliers — border `border_subtle.a * 0.5` (`media_browse_panel.rs:123`), bg `surface_bg.a * 0.92` (`line 127`), thumbnail bg `elevated_bg.a * 0.72` (`line 137`). Contract uses `color-mix(... 92%/90%, transparent)` semantics; the `* 0.5` border alpha has no contract basis at all — item border is `0.0625rem solid border-subtle` (full alpha). Resolve via the color-mix helper, drop ad-hoc alpha math.
- [ ] Hover/focus border-focus + elevated-bg state not implemented — contract §8 item `:hover`/`:focus-visible` change border to `border-focus` and bg to elevated-90%. No interactive state on the div.
- [ ] Meta font uses `typography.label.size` (`media_browse_panel.rs:149`); contract §8 meta font-size `0.8125rem`. Resolve the correct token.
- [ ] Grid is `flex_wrap` not auto-fill grid — accepted Tier-3 layout freedom, but each item lacks a max so wide screens differ; note only.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [ ] Grid min-column wrong for lg/xl — `Lg => 12.5`, `Xl => 14.0` at `media_browse_panel.rs:23-24`; contract §8 `lg 12rem`, `xl 13rem`. Fix to 12.0 / 13.0.
- [ ] Hardcoded label/meta font `label_font = rem_to_px(0.8125)` at `media_browse_panel.rs:18`; contract meta font `0.8125rem` — resolve from a typography token rather than a literal.
- [ ] Item background via `tint(panel_bg, 0.92)` (`media_browse_panel.rs:97`) — verify `tint` reproduces `color-mix(panel 92%, transparent)` (transparent-mix), not a solid lighten; otherwise card translucency differs from Svelte.
- [ ] Load-more actions row missing top margin — contract §8 actions `margin-top: <grid gap>`; `actions` div (`media_browse_panel.rs:154-157`) has no gap/margin from the grid. Add grid-gap top spacing.
- accepted: ARIA channel absent; item `onSelect` interaction belongs in preview event loop (no select wiring in component, which is correct for Jetstream).

## Specimen parity

- Svelte covers: Browse grid (+load more), Loading, Error, Empty (`MediaBrowsePanelSpecimen.svelte` per contract §12).
- GPUI covers: Browse panel (+load more), States (loading/error/empty), Semantic presentation (size+density, sizeRole prominent) — broad. — missing: nothing vs contract specimen set; but thumbnails/load-more render as fakes (see GPUI gaps), so visual parity is broken despite specimen coverage.
- Jetstream covers: With items, Loading more, Loading, Error, Empty, Sizes (sm/md/lg), Densities (compact/default/comfortable) — broadest coverage; uses real MediaThumbnail/Callout/Button. — missing: nothing; hardcoded `text_size(11.0)` group labels (specimen-local).

## Notes

- **GPUI is the worst offender here**: it reimplements MediaThumbnail, Callout, and Button inline as bare divs with hardcoded geometry instead of composing the real primitives. Per CLAUDE.md "No Mockups, No Fakes" this specimen is arguably worse than unimplemented — the placeholder thumbnail rectangle and fake load-more button hide that the composition is incomplete. This is the top-priority fix for this component.
- Jetstream correctly composes `js_media_thumbnail`, `js_callout`, `js_button` — the right pattern; remaining gaps are token-resolution polish.
- Spec field `thumbnail_url` exists on `MediaBrowseItem` but neither Rust target renders an actual image (both show the kind-based thumbnail shell only). Svelte renders an `<img>` when `thumbnailUrl` is present (contract §2 image part). Image rendering is a shared gap but lower priority (specimens pass no URLs). Flag low.
- The `lg=12.5 / xl=14.0` min-column bug is identical in both Rust targets — likely copy-paste; fix both.
