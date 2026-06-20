<!-- parity consv=fixed gpui=9 jetstream=10 specimen=gap -->
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

- [ ] Candidate label weight wrong — uses `FontWeight::SEMIBOLD` (600) at `relation_picker.rs:494`; Svelte is `500`. Drill label same issue at `relation_picker.rs:393`.
- [ ] Hardcoded spacing literals: `px(rem_to_px(0.125))` row-gap (`relation_picker.rs:124`) and inner gaps (`:390,:409,:490`), `pad_x = px(rem_to_px(0.75))` / `pad_y = px(rem_to_px(0.5))` (`:125-126`) — Svelte item padding is `0.375rem 0.5rem` (md) via `--relation-picker-item-y/x` and scales by size. Resolve from size/density tokens, not flat rem.
- [ ] Selected-row background is an alpha-blend hack `accent.a*0.10 + surface.a*0.90` over `surface` (`relation_picker.rs:450-453`); Svelte is `color-mix(accent 10%, transparent)` over the base item bg. Approximation diverges from `color-mix`.
- [ ] Unselected row bg uses `elevated`+`surface` blend (`relation_picker.rs:455-458`); Svelte base is `color-mix(surface 86%, transparent)` (`RelationPicker.svelte:896`). Wrong base color.
- [ ] No search `TextInput` — renders a static fake search box (icon + query text in a div, `relation_picker.rs:189-222`); Svelte composes the real `TextInput type="search"` with clear + keydown. No typing, clear, or describedBy.
- [ ] No `filters` / `searchPlaceholder` / `footerNote` / `showFooter` / `showSelectionSummary` support — spec/builder lack these props.
- [ ] No keyboard nav — Arrow/Home/End/Escape/Backspace candidate + drill-back navigation absent (interaction is click-only via `on_select`/`on_drill_enter`).
- [ ] Drill-list/candidate hover + focus-visible states from contract §8 not rendered (no `:hover` bg on candidate rows, no focus ring).
- [ ] No `data-selected` / `aria-pressed` equivalent for selection state beyond visual border.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label`, `aria-pressed`, `aria-describedby` from contract §6 not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Pervasive hardcoded pixel literals throughout `relation_picker.rs`: `text_size(12.0)`/`text_size(14.0)`/`text_size(11.0)`/`text_size(13.0)`, `pl(12.0)`/`pr(12.0)`/`pt(8.0)`/`pb(8.0)`, `gap(8.0)`/`gap(4.0)`/`gap(2.0)`, `w(14.0)`/`h(14.0)`, `pl(10.0)`/`pr(10.0)` — every dimension is a raw float (e.g. `:204-214`, `:233-279`, `:310-355`). Resolve all from size/density tokens.
- [ ] Candidate + drill label weight wrong — `text_weight(600)` at `relation_picker.rs:342,255`; Svelte is `500`.
- [ ] Item padding hardcoded `12/8` (`relation_picker.rs:314-321`); Svelte md item is `0.375rem 0.5rem` and scales by size. No size/density scaling at all.
- [ ] Selected-row bg `tint(accent, 0.10)` (`relation_picker.rs:300`) replaces the base item color entirely instead of layering `color-mix(accent 10%, transparent)` over `color-mix(surface 86%)`. Unselected uses `tint(surface, 0.86)` which is closer but still not a true mix.
- [ ] Breadcrumb item color bug — `accent_or(text_primary, text_primary)` at `relation_picker.rs:164` always passes `text_primary`, never the accent color; Svelte breadcrumb items are `--poodle-color-accent-base`. Resolve `color.accent.base` and pass it.
- [ ] No real search input — static icon + query label (`relation_picker.rs:184-216`); no `TextInput`, no clear/keydown.
- [ ] No `filters` / `searchPlaceholder` / `footerNote` / `showFooter` / `showSelectionSummary` support.
- [ ] No keyboard nav (Arrow/Home/End/Escape/Backspace) and no drill-back wiring — `Back` button (`relation_picker.rs:142`) has no handler; interaction must live in preview `main.rs` event loop and is currently absent.
- [ ] Drill-row bg `tint(elevated, 0.88)` (`relation_picker.rs:245`) does not match Svelte drill button (transparent, `:hover color-mix(surface 60%)`). No hover/focus states.
- [ ] `theme` param unused in `drill_row`/`candidate_row` (leading `_`-style dead arg) — search/candidate colors are passed in but hover/focus tokens never resolved.
- accepted: no ARIA channel (Jetstream has no a11y tree); interaction click/keyboard handlers belong in preview event loop.

## Specimen parity

- Svelte covers: Multiple selection (pre-selected), Single selection, Drill-down (2 levels + finalItems), Sizes (xs–xl), Densities — plus live `onSelectionChange` echo.
- GPUI covers: Multiple, Single, Loading state, Drill-down (interactive via drill-path state), Semantic presentation (chrome/comfortable). — missing: **Sizes** and **Densities** variant groups (Svelte sweeps all five sizes; GPUI shows one Sm/chrome combo).
- Jetstream covers: With selection, No selection, Single selection, Loading, Drill-down (static path). — missing: **Sizes** and **Densities** sweeps; drill-down is fixed-path (no interactive navigation).

## Notes

- Both Rust targets reuse the real `PickerShell`, `SelectionSummary`, `Checkbox`, `Button` composites, so shell/summary/footer structure is broadly faithful — the gaps are concentrated in the search field (faked), candidate styling (token violations + wrong weight/mix), and interactivity (no keyboard, no live search).
- `PickerItemSpec` (`packages/contracts/components/src/composite_types.rs:535`) has no `disabled` field, so neither Rust target can express Svelte's disabled-candidate state until the spec gains it.
- `consv=fixed`: the undocumented Svelte surface (`filters`, `selectedItems`, `searchPlaceholder`, `stateTitle/Message`, `footerNote`, `showFooter`, `showSelectionSummary`, `renderItem`, `onFilterChange`, `item.disabled`) is now fully reconciled into the contract (§2/§3/§5/§6/§8) per "Svelte is parity authority".
