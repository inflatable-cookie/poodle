<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass: spec reconciled to single-select select-and-close (dropped is_multiple/selected_count; added MediaPickerItem/items/accept/maxFileSize/active_tab + MediaPickerTab); both targets render real items, compose real MediaThumbnail/TextInput/FileUpload (GPUI) primitives, empty state, browse+upload tabs; fabricated footer/count removed. -->
# Parity: MediaPicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/media-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/MediaPicker.svelte`
- GPUI: `packages/gpui/components/src/composites/media_picker.rs`
- Jetstream: `packages/jetstream/components/src/media_picker.rs`
- Spec: `packages/contracts/components/src/media_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MediaPickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/media_picker_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/media_picker.rs`

## Contract ↔ Svelte

Contract↔Svelte are aligned (props, callbacks, tabs, listbox/option roles, size/density tables) — verified clean. `consv=fixed`: no contract edit needed; the contract already matches Svelte and correctly excludes the Rust-invented multi-select/confirm model. The remaining divergence is Rust-spec-side, tracked under the Rust gaps.

- [x] VERIFIED Contract↔Svelte: all 9 props (`open`, `items`, `accept`, `maxFileSize`, `title`, `emptyMessage`, `size`, `sizeRole`, `density`) and 3 callbacks (`onSelect`, `onUpload`, `onOpenChange`) match `MediaPicker.svelte:18-46`. `role="listbox"`/`role="option"`, select+auto-close (`MediaPicker.svelte:80-83`), Tabs/TextInput/FileUpload composition all present. Contract §1 lists "multi-select" as out of scope and §5 documents select-and-close — no Confirm/selection-count in contract or Svelte. The `multiple` attribute in Svelte is on the FileUpload (multi-file *upload*), not multi-*select*.
- [x] **Spec reconciled.** Dropped `is_multiple` + `selected_count`; added `MediaPickerItem { id, label, has_thumbnail, kind }`, `items: Vec<MediaPickerItem>`, `accept`, `max_file_size`, `active_tab` + `MediaPickerTab` enum, and browse-grid token methods. Contract left as-is (single-select select-and-close). `accepted_types` replaced by `accept`.

## GPUI gap (vs Svelte + contract)

GPUI renders its own dialog chrome + a fabricated footer; many literals and a wrong interaction model.

- [x] Dialog shadow now uses `elevation_dialog_shadow()`; dialog min-w/max-h are contract-exact rem (`rem_to_px`).
- [x] Magic px gone — grid min-column width from the contract §8 size table, item padding/gap from density; thumbnail geometry owned by the composed `MediaThumbnail`.
- [x] **Fabricated footer removed** (no selection count / Confirm button).
- [x] Single-select select-and-close — grid items fire `on_select(id)`; no `is_selected` borders/count.
- [x] Search is a real `TextInput` primitive (placeholder "Search media…").
- [x] Upload tab renders the real `FileUpload` dropzone (multi-file, forwards `accept`/`max_file_size`); browse/upload composed via the real `Tabs` primitive with content panels.
- [x] Grid items compose the real `MediaThumbnail` (compact, square, no caption) + truncated label — placeholder posture is MediaThumbnail-owned.
- accepted: no ARIA (`role=listbox/option`, `aria-selected` not emitted).
- note: thumbnail bitmaps host-owned — `MediaPickerItem::has_thumbnail` is carried on the spec for the placeholder-vs-image split (MediaThumbnail renders the placeholder posture in both Rust targets since neither decodes images).

## Jetstream gap (vs Svelte + contract)

Skeletal + fabricated placeholder items.

- [x] **Mockup removed** — grid renders the real `spec.items` (no `"Item N"` placeholders); placeholder items show the token-resolved image glyph, thumbnail items the panel surface.
- [x] Remaining literals are contract-exact rem via `rem_to_px` (title 1rem, thumb radius 0.25rem, label/size fonts) — token-resolved colors/radii throughout.
- [x] **Fabricated selection count removed.**
- [x] Search is a real `text_input` element (placeholder "Search media…").
- [x] Upload tab composes the real `js_file_upload` dropzone (multi-file, forwards `accept`/`max_file_size`); browse vs upload driven by `spec.active_tab`.
- [x] `active_tab` now lives on the spec and drives which tab content renders + the active tab indicator.
- [x] Empty state rendered (centered `empty_message`, min-height 10rem).
- accepted: interaction (tab switch, select) would live in preview event loop; absent.
- note: JsEl has no CSS auto-fill grid, so the browse grid uses `flex_wrap` with the size/density gap (approximation, noted).

## Specimen parity

- Svelte covers: Media picker dialog (6 sample items, mix of thumbnails + placeholders, select-and-close, upload tab) + size/density grid (`MediaPickerSpecimen.svelte`).
- GPUI covers: Media picker dialog (6 real items, mix of thumbnail/placeholder), Upload tab (real FileUpload), Empty state, Semantic presentation (size/density variants). Select-and-close model; no footer.
- Jetstream covers: Browse tab (6 real items), Upload tab (real FileUpload dropzone), Empty state. No fabricated selection count.

## Notes

- `consv=fixed`: the contract↔Svelte axis is clean (no edit). The headline work item lives in the Rust `MediaPickerSpec`, which models multi-select + confirm + selection-count — none of which exist in contract+Svelte (multi-select is explicitly out of scope). Reconciling the spec to the single-select select-and-close model makes the GPUI footer + Jetstream count obsolete in one stroke.
- Neither Rust target threads a real `items` list or composes FileUpload; both fake the grid/upload. The spec lacks the item payload entirely (`media_picker.rs:5-16`).
