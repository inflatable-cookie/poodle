<!-- parity consv=fixed gpui=4 jetstream=4 specimen=gap | pass: real search TextInput, token-resolved candidate/drill geometry, weight 500, color-mix selected/base bg, breadcrumb accent fixed, drill-row transparent base -->
# Parity: RelationPicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/relation-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/RelationPicker.svelte`
- GPUI: `packages/gpui/components/src/composites/relation_picker.rs`
- Jetstream: `packages/jetstream/components/src/relation_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/RelationPickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/relation_picker_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/relation_picker.rs`

## Contract ↔ Svelte

Svelte carries a large undocumented prop/snippet/callback surface. Svelte is authoritative — add all of it to the contract §3/§5.

- FIXED — Svelte props not in contract §3 (`selectedItems`, `searchPlaceholder` `"Search picker results"`, `filters` `PickerFilterConfig[]`, `filterValues`, `stateTitle`, `stateMessage`, `footerNote`, `showFooter` default `true`, `showSelectionSummary` default `true`) all added to contract §3 props; `PickerFilterConfig`/`PickerFilterOption` types added to §3 Types; filter controls added to the §2 Toolbar anatomy.
- FIXED — snippet `renderItem: Snippet<[item, selected]>` added to contract §3 Snippets table.
- FIXED — callback `onFilterChange: (key, value) => void` added to contract §5.
- FIXED — `item.disabled` support: `disabled?: boolean` added to the §3 `PickerItem` type, a §8 "Candidate Item Disabled" subsection (`data-disabled`/`aria-disabled`/`disabled` attr, `opacity 0.55`, skip-toggle), and a §6 a11y note.
- `selectedIds` / `query` defaults (`undefined`) already match. OK.
- Anatomy `[DrillEmpty]` + `[FormActions]` already present. OK.
- FIXED — candidate label weight `500` added to the §8 Candidate Copy table (`strong font-weight: 500`).

## GPUI gap (vs Svelte + contract)

- [x] FIXED — candidate + drill label weight now `FontWeight::MEDIUM` (500) via the shared `LABEL_WEIGHT` const (was SEMIBOLD/600).
- [x] FIXED — candidate/drill geometry resolves from size/density tokens: `relation_picker_item_x/y/gap_rem(size)` + `relation_picker_list_gap_rem(density)` (new `presentation` helpers mirroring the Svelte size/density tables). No flat `0.75/0.5/0.125` literals.
- [x] FIXED — selected-row bg is `color_mix(accent, transparent, 0.10)` and base is `color_mix(surface, transparent, 0.86)`; selected border `color_mix(accent, transparent, 0.60)`. Matches the contract §8 / Svelte `color-mix` exactly (no alpha-blend hack, no `elevated` base).
- [x] FIXED — real search field: `TextInput::from_spec` with `input_type="search"`, leading `search` icon, clear button, and the current query as value. Replaces the faked icon+query div.
- [ ] No `filters` / `footerNote` / `showFooter` / `showSelectionSummary` support — `RelationPickerSpec`/builder lack these props (would need additive spec fields; out of scope this pass).
- [ ] No keyboard nav — Arrow/Home/End/Escape/Backspace candidate + drill-back navigation absent. Preview-loop: interaction lives in the consumer event loop alongside `on_select`/`on_drill_enter`.
- [ ] Drill-list/candidate hover + focus-visible states (contract §8) not rendered. Preview-loop: GPUI hover/focus are runtime states owned by the consumer.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`, `aria-pressed`, `aria-describedby`, `data-selected` from contract §6 not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED — all hardcoded pixel literals replaced by token-resolved geometry: candidate/drill padding from `relation_picker_item_x/y_rem(size)`, inner gap from `relation_picker_item_gap_rem(size)`, list gap from `relation_picker_list_gap_rem(density)`, title/desc fonts from `relation_picker_title/desc_size_rem(size)`, label/crumb fonts from `typography.label.size`. Chevron sized in rem (`0.875`).
- [x] FIXED — candidate + drill label weight now `500` (`LABEL_WEIGHT`).
- [x] FIXED — selected-row bg is `color_mix(accent, transparent, 0.10)`; base is `color_mix(surface, transparent, 0.86)`; selected border `color_mix(accent, transparent, 0.60)`. True color-mix per contract §8.
- [x] FIXED — breadcrumb item color bug gone: breadcrumb items now resolve `color.accent.base` and render as id-tagged buttons (`poodle-relation-crumb-{i}`) at weight 500.
- [x] FIXED — real search input via `js_text_input` (`input_type="search"`, leading `search` icon, clear button, query as value). No faked icon+label box.
- [x] FIXED — drill-row base is transparent (`glam::Vec4::ZERO`) matching Svelte `.drill-list__button`; drill rows are id-tagged hit targets (`poodle-relation-drill-{id}`). Drill-empty "No items found" state added.
- [x] FIXED — dead `theme`/`surface`/`elevated` args pruned from `drill_row`; remaining args are all live.
- [ ] No `filters` / `footerNote` / `showFooter` / `showSelectionSummary` support — spec lacks these props (additive spec fields, out of scope this pass).
- [ ] No keyboard nav (Arrow/Home/End/Escape/Backspace) and no drill-back/breadcrumb/candidate click wiring. Preview-loop: the `poodle-relation-{drill,crumb,candidate,drill-back}-*` ids are emitted as hit targets, but the handlers belong in the preview `main.rs` event loop and are absent.
- [ ] Hover/focus states (contract §8 `:hover`/`:focus-visible`) not rendered. Preview-loop: runtime states owned by the event loop (JsEl has no static focus modifier).
- accepted: no ARIA channel (Jetstream has no a11y tree).

## Specimen parity

- Svelte covers: Multiple selection (pre-selected), Single selection, Drill-down (2 levels + finalItems), Sizes (xs–xl), Densities — plus live `onSelectionChange` echo.
- GPUI covers: Multiple, Single, Loading state, Drill-down (interactive via drill-path state), Semantic presentation (chrome/comfortable). — missing: **Sizes** and **Densities** variant groups (Svelte sweeps all five sizes; GPUI shows one Sm/chrome combo).
- Jetstream covers: With selection, No selection, Single selection, Loading, Drill-down (static path). — missing: **Sizes** and **Densities** sweeps; drill-down is fixed-path (no interactive navigation).

## Notes

- Both Rust targets reuse the real `PickerShell`, `SelectionSummary`, `Checkbox`, `Button` composites, so shell/summary/footer structure is broadly faithful — the gaps are concentrated in the search field (faked), candidate styling (token violations + wrong weight/mix), and interactivity (no keyboard, no live search).
- `PickerItemSpec` (`packages/contracts/components/src/composite_types.rs:535`) has no `disabled` field, so neither Rust target can express Svelte's disabled-candidate state until the spec gains it.
- `consv=fixed`: the undocumented Svelte surface (`filters`, `selectedItems`, `searchPlaceholder`, `stateTitle/Message`, `footerNote`, `showFooter`, `showSelectionSummary`, `renderItem`, `onFilterChange`, `item.disabled`) is now fully reconciled into the contract (§2/§3/§5/§6/§8) per "Svelte is parity authority".
