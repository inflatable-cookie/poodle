<!-- parity consv=gap gpui=8 jetstream=8 specimen=gap -->
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

Contract↔Svelte themselves are aligned (props, callbacks, tabs, listbox/option roles, size/density tables). `consv=gap` is driven by a Rust spec that invents a different model than contract+Svelte.

- Contract↔Svelte: all props (`open`, `items`, `accept`, `maxFileSize`, `title`, `emptyMessage`, `size`, `sizeRole`, `density`) and callbacks (`onSelect`, `onUpload`, `onOpenChange`) match `MediaPicker.svelte:18-46`. `role="listbox"`/`role="option"`, select+auto-close (`MediaPicker.svelte:80-83`), Tabs/TextInput/FileUpload composition all present. **ok.**
- **Spec model divergence**: `MediaPickerSpec` (`media_picker.rs:5-16`) carries `is_multiple` + `selected_count` (multi-select), but contract §1 explicitly lists "multi-select" as **out of scope** and Svelte has no multi-select, no Confirm button, no selection count. The Rust spec models a multi-select-with-confirm picker that does not exist in the reference. **Fix: drop `is_multiple`/`selected_count`; the picker selects-and-closes on click per Svelte.** Marked `consv=gap` to force this reconciliation.
- Spec is also missing the real `items`/`accept`/`maxFileSize` payload — it carries only `accepted_types: Option<String>` and no item list, so both Rust impls cannot render real items.

## GPUI gap (vs Svelte + contract)

GPUI renders its own dialog chrome + a fabricated footer; many literals and a wrong interaction model.

- [ ] Hardcoded HSLA shadow literals: `hsla(0.0, 0.0, 0.0, 0.12)` and `hsla(0.0, 0.0, 0.0, 0.08)` (`media_picker.rs:160,166`) — resolve from `spec.shadow_token()` (which exists, `media_picker.rs:63`) not raw HSLA.
- [ ] Hardcoded px literals: dialog `min_w(px(480.0))`/`max_h(px(520.0))` (`:174-175`), shadow offsets/blur `px(8.0)`/`px(24.0)`/`px(2.0)` (`:161-169`), item `w(px(72.0)).h(px(72.0))` (`:361-362`), `max_w(px(80.0))` (`:383`), border `px(2.0)`/`px(1.0)` (`:355`) — all magic; resolve from size/tokens (contract §8 thumb-size table is size-driven).
- [ ] **Fabricated footer** — selection-count text + "Confirm" button (`:405-456`) not in contract/Svelte (select-and-close model). Remove.
- [ ] Wrong selection model — renders `is_selected` borders + count; contract is single-click-to-select-and-close.
- [ ] Search is a static text div (`:283-301`) not a real TextInput; no filtering.
- [ ] Upload tab does not render FileUpload — tab toggles but no dropzone exists (only the browse grid renders regardless of `active_tab`).
- [ ] Grid items render a generic `image` Icon placeholder; no `thumbnailUrl` image path, no placeholder-SVG distinction (contract §8 thumbnail vs placeholder).
- [ ] Confirm button bg uses raw `accent` + `gpui::white()` text instead of accent fill + on-accent text token.
- accepted: no ARIA (`role=listbox/option`, `aria-selected` not emitted).

## Jetstream gap (vs Svelte + contract)

Skeletal + fabricated placeholder items.

- [ ] **MOCKUP VIOLATION**: grid renders 4 hardcoded placeholder items `format!("Item {}", i + 1)` (`media_picker.rs:83-102`) — no real `items` flow through the spec. Per CLAUDE.md "No Mockups". Either thread real items through the spec or leave the grid unimplemented.
- [ ] Hardcoded px literals: title `rem_to_px(1.0)` (`:47`), search padding `rem_to_px(0.5)`/`0.25` (`:70-71`), thumb-radius `rem_to_px(0.25)` (`:93`), placeholder label `rem_to_px(0.8125)` (`:99`) — magic numbers.
- [ ] **Fabricated selection count** — "N selected" footer (`:106-111`) not in contract/Svelte.
- [ ] Search is a static label (`:66-76`), not a TextInput; no filtering.
- [ ] Upload tab does not render FileUpload — tabs are non-functional buttons; browse grid always shown.
- [ ] Tabs render but `active_tab` is not in the spec, so switching has no effect.
- [ ] No empty state (`empty_message` ignored).
- accepted: interaction (tab switch, select) would live in preview event loop; absent.

## Specimen parity

- Svelte covers: Media picker dialog (6 sample items, mix of thumbnails + placeholders, select-and-close, upload tab) + size/density grid (`MediaPickerSpecimen.svelte`).
- GPUI covers: Media picker dialog (sample thumbnails), Semantic presentation (size/density variants) (`media_picker_specimen.rs`). — missing: upload tab content, real images, select-and-close (shows confirm footer instead).
- Jetstream covers: Browse tab, With-selections (`with_selected_count(2)`) (`media_picker.rs:13-25`). — missing: real items (fabricated), upload tab, select interaction. "With selections" demonstrates the non-contract multi-select model.

## Notes

- `consv=gap` driver: the Rust `MediaPickerSpec` models multi-select + confirm + selection-count, none of which exist in contract+Svelte (multi-select is explicitly out of scope). Reconciling the spec to the single-select select-and-close model is the headline work item — it makes the GPUI footer + Jetstream count obsolete in one stroke.
- Neither Rust target threads a real `items` list or composes FileUpload; both fake the grid/upload. The spec lacks the item payload entirely (`media_picker.rs:5-16`).
