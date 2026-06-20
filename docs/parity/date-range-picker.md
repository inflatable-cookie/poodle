<!-- parity consv=gap gpui=3 jetstream=3 specimen=gap -->
# Parity: DateRangePicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-range-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DateRangePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_range_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_range_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DateRangePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_range_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_range_picker.rs`

## Contract ↔ Svelte

Props, callbacks, ARIA, anatomy align. Size-table is wrong vs Svelte.

- Size table (§8) lists per-size `padding` overrides for every size (`xs`…`xl`, e.g. `0 calc(var(--poodle-space-control-x) - 0.125rem)`). Svelte applies **no** per-size padding — only per-size `font-size` and trigger-height (`DateRangePicker.svelte:263-287`). Density owns horizontal padding (lines 289-290), not size. The contract's per-size padding overrides violate the size/density orthogonality rule in CLAUDE.md. **Fix: delete the `padding` rows from the contract §8 size table; keep them under density.**
- Size table specifies `min-height` as `calc(control-height ± Xrem)`; Svelte hardcodes absolute rems (`1.5rem`/`1.75rem`/`2.75rem`/`3.25rem`). Same resolved values; **Fix: reconcile token-form in contract to match Svelte.**
- Mid-selection display: Svelte shows `"<start> – End date"` when only start is chosen (`DateRangePicker.svelte:63`). Contract §4 describes "start displayed, overlay remains open" but does not specify the "– End date" suffix string. **Fix: document the partial-range display string in contract §4.**

## GPUI gap (vs Svelte + contract)

- [ ] Indicator renders a `calendar` Icon (`date_range_picker.rs:201`) instead of the `▾` chevron Svelte/contract use. Align glyph across targets.
- [ ] Hardcoded shadow literals `hsla(0.0,0.0,0.0,0.10)`/`0.06` + `px(16.0)`/`px(4.0)` at `date_range_picker.rs:247-257`. Contract surface shadow = `var(--poodle-elevation-overlay)`; resolve from elevation token, not raw HSLA + float px.
- [ ] Partial-range display uses `"<start> – …"` ellipsis (`date_range_picker.rs:148`) where Svelte uses the literal `" – End date"` text. Match Svelte's partial string.
- accepted: no ARIA (gpui has no accessibility API) — haspopup/expanded/dialog-role not emitted.
- accepted: overlay renders as flow-child (`wrapper.child(overlay)`), anchored-below positioning is a platform delta.

## Jetstream gap (vs Svelte + contract)

- [ ] No calendar overlay + no open-state handling: `js_date_range_picker` only emits the trigger; never reads `spec.current_open()`. Interaction/overlay must live in preview event loop — confirm a handler exists, else the "open" range-calendar is unreachable (none present in the specimen).
- [ ] Partial-range display joins with `"<start> - <end-or-...>"` (`date_range_picker.rs:43-49`) using `"..."` for the missing bound; Svelte uses `" – End date"`. Match Svelte's partial string + en-dash separator (uses ASCII hyphen here).
- [ ] Trigger gap is `rem_to_px(0.75)` literal (`date_range_picker.rs:63`); fine numerically but should resolve from a content-gap token like other token reads. Low priority — flag for token-form consistency.
- accepted: no ARIA channel for haspopup/expanded/dialog role.
- accepted: range-calendar surface + selection interaction live in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default, With default range, Disabled, Sizes, Densities (`DateRangePickerSpecimen.svelte`).
- GPUI covers: Default (open-toggle), With default range (open-toggle), Disabled, Sizes, Densities. — full parity.
- Jetstream covers: With range, Placeholder, Disabled. — missing: **Sizes** and **Densities** groups; no open/overlay demonstration.

## Notes

- Jetstream range picker correctly applies `min_w(rem_to_px(16.0))` and a disabled branch — both of which the Jetstream `date_picker` lacks. Treat this file as the reference shape when fixing Jetstream `date_picker`.
- `consv=gap` driver: contract size-table carries per-size padding overrides that violate size/density orthogonality and that Svelte does not implement.
