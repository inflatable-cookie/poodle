<!-- parity consv=fixed gpui=2 jetstream=2 specimen=gap -->
<!-- pass 45: Jetstream rebuilt to match GPUI — each option composes js_card (interactive/
     selected, Card owns the fill — dropped selected_fill swap) with a radio indicator (ring +
     dot from indicator_size_rem/dot_size_rem/indicator_border_rem), title + optional description;
     single-select via current_value; density grid gap; per-item/group disabled. 3 probe tests
     (title, selected≠unselected, custom indicator size); jet 169. Selection/arrow-nav = preview-loop. -->
<!-- pass 24: GPUI rebuilt — options compose Card::from_spec(interactive/selected)
     (mirrors card_toggle_group); dropped invented 12%-accent fill (Card owns it);
     indicator/dot/font dims from additive spec helpers (indicator_size_rem/dot_size_rem/
     title_font_rem/description_font_rem/indicator_border_rem) not px literals; radius.pill,
     header gap space.inline.sm. Build clean, specs 53 pass. -->
# Parity: CardRadioGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/card-radio-group.md`
- Svelte (authoritative): `packages/svelte/components/src/CardRadioGroup.svelte`
- GPUI: `packages/gpui/components/src/composites/card_radio_group.rs`
- Jetstream: `packages/jetstream/components/src/card_radio_group.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CardRadioGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/card_radio_group_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/card_radio_group.rs`

## Contract ↔ Svelte

Class names and a couple of prop defaults diverge. Svelte authoritative.

- [x] FIXED Class prefix: contract §2 anatomy, all §8 token-table headers, the §8 data-attributes table, and the focus-visible/density `:global` selectors now use the `poodle-` prefix (`.poodle-card-radio-group*`, `:global(.poodle-card)`).
- [x] FIXED `value` default: contract §3 props table now lists `string | null | undefined`, default `undefined`, and §3 Controlled And Uncontrolled documents the `undefined` = uncontrolled / defined = controlled split.
- [x] FIXED `onValueChange` added to the §3 props table (and the Tier 1 checklist line reworded from "event name" to "onValueChange callback name").
- [x] FIXED §9 Svelte Notes: dropped the `createEventDispatcher`/`change` note; now states selection flows through `onValueChange` + `bind:value`.
- [x] FIXED (extra) Density table: comfortable gap was `1rem`, Svelte is `0.875rem`; compact card padding was full `0.5rem`, Svelte sets only `padding-inline: 0.5rem`; comfortable had a phantom `1rem` card-padding override Svelte does not emit. Reconciled to Svelte.
- Snippet type name: contract §3 calls the item type `CardRadioItem` and that matches Svelte's TS type, OK. No divergence there.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Does not compose the `Card` primitive — builds a raw `div` (`card_radio_group.rs:144-163`). Contract §2 requires the `Card` primitive with `interactive`/`selected`; selected styling is hand-rolled via `color_mix` instead.
- [ ] Hardcoded indicator dims `.w(px(18.0)).h(px(18.0))` at `card_radio_group.rs:102-103` — must resolve per-size from a token (contract size table: xs 0.875rem … xl 1.375rem); GPUI ignores size entirely for the indicator.
- [ ] Hardcoded `.rounded(px(999.0))` at `card_radio_group.rs:104,116` and `.min_w(px(200.0))` at `:149` — pill radius + min-width are raw literals, not tokens.
- [ ] Inner dot fixed at `rem_to_px(0.375)` (`card_radio_group.rs:114-115`) — does not apply the per-size dot table (xs 0.25rem … xl 0.5rem); resolve from size.
- [ ] `color_mix(accent, unselected_fill, 0.12)` at `card_radio_group.rs:83` invents a selected fill; contract/Svelte selected state uses `Card`'s selected token + accent indicator, not a 12% accent tint. Use `selected_fill_token()`.
- [ ] No `columns` support — uses `flex_wrap()` (`card_radio_group.rs:86`) instead of an N-column grid; contract §3 `columns: 1|2|3|4` is unimplemented.
- [ ] No arrow-key roving-tabindex navigation (contract §6) — `.focusable()` is set but no key handler; arrow nav + wrap + skip-disabled missing.
- [ ] Hardcoded header/content gaps `.gap(px(rem_to_px(0.5)))` at `card_radio_group.rs:126,159` — density-fixed 0.5rem is correct per contract but should resolve via a content-gap token, not a raw rem literal.
- accepted: no ARIA (gpui has no accessibility API) — `role="radiogroup"`/`role="radio"`/`aria-checked` cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No `Card` primitive composition — raw `div` card (`card_radio_group.rs:68-74`); contract §2 requires the `Card` primitive (interactive/selected).
- [ ] Hardcoded card padding `.pl/pr/pt/pb(rem_to_px(0.75))` at `card_radio_group.rs:73-74` — should resolve from the density-driven Card padding token, not raw `0.75`.
- [ ] Hardcoded border `.border(1.0)` at `card_radio_group.rs:71` and pill `.rounded(999.0)` at `:87,96` — raw literals; resolve border width + pill radius from tokens.
- [ ] Hardcoded disabled opacity `card.opacity(0.48)` at `card_radio_group.rs:77` — must use `resolve_opacity(theme, "state.opacity.disabled")`, not literal `0.48`.
- [ ] Indicator border `.border(rem_to_px(0.125))` at `card_radio_group.rs:91,100` is a raw rem literal — contract border `0.125rem` should come from a token.
- [ ] No selected-card fill change beyond `selected_fill_token()`; OK, but border does not switch to accent when checked (Svelte/contract: checked card border-color → accent). Selected card border stays `border_color` (`card_radio_group.rs:71`).
- [ ] No `columns` support — `flex_wrap()` (`card_radio_group.rs:62`) instead of an N-column grid.
- [ ] No `on_change` / selection callback channel at all — `js_card_radio_group` takes no handler; interaction must live in preview `main.rs` event loop (currently absent → component is render-only, not selectable).
- [ ] No keyboard navigation (arrow roving tabindex, contract §6) — not modeled.
- accepted: no ARIA channel (`role`/`aria-checked` not expressible).
- accepted: interaction (click/key handling) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Plan selection (2col), Instance size (3col, no initial value), Disabled group (3col), plus `sizes` + `densities` snippets, plus live "Selected:" readout (`CardRadioGroupSpecimen.svelte`).
- GPUI covers: Plan selection, Instance size, Disabled, sizes + densities via `specimen_layout`, live selection readout, interactive `on_change` (`card_radio_group_specimen.rs`). — missing: `columns` is not exercised (component ignores it), so 2-vs-3 column visual parity is unverifiable.
- Jetstream covers: "With selection", "No selection" only (`jetstream/.../card_radio_group.rs`). — missing: **Disabled group**, **sizes** group, **densities** group, **columns** demonstration, live selection readout.

## Notes

- GPUI/Jetstream `ChoiceOption` carries `label` (matches Svelte `CardRadioItem.label`); no `title` mismatch here (contrast with CardToggleGroup).
- Selected-fill modeling differs three ways: Svelte/contract lean on `Card`'s selected token + accent indicator; GPUI invents a 12% accent `color_mix`; Jetstream uses `selected_fill_token()` (elevated surface). Reconcile all three onto the Card-selected token path.
- `consv=fixed`: all contract↔Svelte bookkeeping resolved (`poodle-` class prefix, `value` default `undefined`, `onValueChange` in props table, dropped `createEventDispatcher`/`change` note, density-table values). Remaining gpui/jetstream todos are code-side.
