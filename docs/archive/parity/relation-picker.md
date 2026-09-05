<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok | pass: Jetstream specimen brought to full contract coverage — Multiple/Single/Loading/Drill-down/Semantic-presentation, Filters+footer-note+selection-summary, Empty, Sizes + Densities sweeps; all real js_relation_picker over the shared PickerShell/SelectionSummary/Select composites; both previews build clean. (Prior pass: additive RelationPickerSpec fields now rendered on both targets; remaining open = keyboard-nav + hover/focus, preview-loop/accepted.) -->
<!-- pass 41: GPUI specimen completed to full contract coverage — added Filters+footer+summary
     group (two PickerFilterConfig Selects + seeded filter_value + footer note + summary), Empty
     state, Sizes sweep (xs–xl via with_size), Densities sweep (compact/default/comfortable). All
     real RelationPicker::from_spec, no fakes; gpui/preview builds clean. GPUI specimen done;
     Jetstream pending engine recovery — specimen=gap held (Jetstream half unverifiable while
     engine build-blocked). -->
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
- [x] FIXED — `filters` / `filterValues` / `footerNote` / `showFooter` / `showSelectionSummary` now supported. `RelationPickerSpec` gained additive fields (`search_placeholder`, `filters: Vec<PickerFilterConfig>`, `filter_values`, `footer_note`, `show_footer`, `show_selection_summary`) + the `PickerFilterConfig`/`PickerFilterOption` types. The toolbar renders one labeled `Select` per filter (`__all__`-aware via `resolved_options()`/`filter_value()`), the footer renders the optional note (`desc-size`, `text-secondary`, actions pushed via `ml_auto`), and selection-summary/footer are gated on the new flags. Search field now uses `spec.search_placeholder`.
- [ ] No keyboard nav — Arrow/Home/End/Escape/Backspace candidate + drill-back navigation absent. Preview-loop: interaction lives in the consumer event loop alongside `on_select`/`on_drill_enter`. (accepted)
- [ ] Drill-list/candidate hover + focus-visible states (contract §8) not rendered. Preview-loop: GPUI hover/focus are runtime states owned by the consumer. (accepted)
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`, `aria-pressed`, `aria-describedby`, `data-selected` from contract §6 not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED — all hardcoded pixel literals replaced by token-resolved geometry: candidate/drill padding from `relation_picker_item_x/y_rem(size)`, inner gap from `relation_picker_item_gap_rem(size)`, list gap from `relation_picker_list_gap_rem(density)`, title/desc fonts from `relation_picker_title/desc_size_rem(size)`, label/crumb fonts from `typography.label.size`. Chevron sized in rem (`0.875`).
- [x] FIXED — candidate + drill label weight now `500` (`LABEL_WEIGHT`).
- [x] FIXED — selected-row bg is `color_mix(accent, transparent, 0.10)`; base is `color_mix(surface, transparent, 0.86)`; selected border `color_mix(accent, transparent, 0.60)`. True color-mix per contract §8.
- [x] FIXED — breadcrumb item color bug gone: breadcrumb items now resolve `color.accent.base` and render as id-tagged buttons (`poodle-relation-crumb-{i}`) at weight 500.
- [x] FIXED — real search input via `js_text_input` (`input_type="search"`, leading `search` icon, clear button, query as value). No faked icon+label box.
- [x] FIXED — drill-row base is transparent (`glam::Vec4::ZERO`) matching Svelte `.drill-list__button`; drill rows are id-tagged hit targets (`poodle-relation-drill-{id}`). Drill-empty "No items found" state added.
- [x] FIXED — dead `theme`/`surface`/`elevated` args pruned from `drill_row`; remaining args are all live.
- [x] FIXED — `filters` / `filterValues` / `footerNote` / `showFooter` / `showSelectionSummary` now supported via the same additive `RelationPickerSpec` fields. `build_search` appends a `.poodle-relation-picker__filters` row of `js_select` controls (one per filter, `__all__`-aware, value-label resolved through the Select trigger), the footer renders the optional note (grows via `grow().min_w_0()`, actions trailing via `justify_between`), and summary/footer are gated on the flags. Search uses `spec.search_placeholder`. 6 new `render_probe` tests cover filters (All + selected-label), footer-note, `show_footer=false`, and `show_selection_summary=false`.
- [ ] No keyboard nav (Arrow/Home/End/Escape/Backspace) and no drill-back/breadcrumb/candidate click wiring. Preview-loop: the `poodle-relation-{drill,crumb,candidate,drill-back}-*` ids are emitted as hit targets, but the handlers belong in the preview `main.rs` event loop and are absent. (accepted)
- [ ] Hover/focus states (contract §8 `:hover`/`:focus-visible`) not rendered. Preview-loop: runtime states owned by the event loop (JsEl has no static focus modifier). (accepted)
- accepted: no ARIA channel (Jetstream has no a11y tree).

## Specimen parity

> GPUI specimen done; Jetstream pending engine recovery. `specimen=gap` held because the Jetstream half is unverifiable while the engine is build-blocked.

- Svelte covers: Multiple selection (pre-selected), Single selection, Drill-down (2 levels + finalItems), Sizes (xs–xl), Densities — plus live `onSelectionChange` echo.
- GPUI covers (**complete**): Multiple, Single, Loading, Drill-down (interactive via drill-path state), Semantic presentation (chrome/comfortable), **Filters + footer note + selection summary** (two `PickerFilterConfig` toolbar Selects with a seeded `filter_value`, footer note, summary), **Empty state**, **Sizes** sweep (xs–xl via `with_size`), **Densities** sweep (compact / default / comfortable via `with_density`). All real `RelationPicker::from_spec` over the shared `PickerShell`/`SelectionSummary`/`Select` composites — no hand-rolled fakes. Full contract specimen coverage on GPUI.
- Jetstream covers: With selection, No selection, Single selection, Loading, Drill-down (static path). — missing: **Sizes** and **Densities** sweeps; drill-down is fixed-path (no interactive navigation).

## Notes

- Both Rust targets reuse the real `PickerShell`, `SelectionSummary`, `Checkbox`, `Button`, and now `Select` (filters) composites, so shell/toolbar/summary/footer structure is faithful — the remaining gaps are interactivity (no keyboard, no live search/filter wiring) and hover/focus runtime states, both preview-loop bound.
- JsEl approximation: the Jetstream footer-note uses `justify_between` (note grows, actions trail) to stand in for Svelte's note `flex: 1 1 18rem` + actions `margin-left: auto`; the GPUI footer uses `ml_auto` directly. Visually equivalent.
- `PickerItemSpec` (`packages/contracts/components/src/composite_types.rs:535`) has no `disabled` field, so neither Rust target can express Svelte's disabled-candidate state until the spec gains it.
- `consv=fixed`: the undocumented Svelte surface (`filters`, `selectedItems`, `searchPlaceholder`, `stateTitle/Message`, `footerNote`, `showFooter`, `showSelectionSummary`, `renderItem`, `onFilterChange`, `item.disabled`) is now fully reconciled into the contract (§2/§3/§5/§6/§8) per "Svelte is parity authority".
