<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap -->
<!-- specimen: GPUI specimen done (added static "Open (calendar + time)" group composing the
     real Calendar + TimeInput; full contract state coverage, no fake grid); Jetstream pending
     engine recovery — specimen=gap kept until Jetstream Sizes/Densities/open land. -->
<!-- pass 41: GPUI indicator glyph aligned to chevron-down (contract §2/§8 disclosure
     chevron, matches sibling pickers) — last GPUI todo closed. Jetstream built out: the
     overlay now composes the REAL Calendar + a Time Section (contract Time Label + composed
     TimeInput) via current_open(), partial-value prompts ("Select time"/"Select date") match
     Svelte, and the indicator font resolves from the per-size token ladder. Spec gained an
     additive current_open() resolver (mirrors current_value). Probe tests added (jetstream):
     placeholder/complete/partial display, chevron indicator, closed-has-no-overlay, open
     composes calendar+time, time-label typography, sizes, disabled. Both targets build/test
     clean. Stale GPUI body items (px(rem_to_px(...)) cell sizes, shadow literals, time-label
     typography, body/time gaps) were already fixed in passes 17/22 — reclassified below. -->
<!-- pass 22: overlay shadow now elevation_overlay_shadow() (token). -->
<!-- pass 17: GPUI overlay rebuilt — fake 6×7 grid + "Today/Done" bar replaced with
     composed Calendar::from_spec + TimeField; mock px literals removed; time-label
     typography (0.6875rem/600/uppercase) + body/time gaps (0.875/0.375rem) applied.
     Remaining GPUI: elevation-overlay shadow token (cross-cutting), indicator glyph. -->
# Parity: DateTimePicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-time-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DateTimePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_time_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_time_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DateTimePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_time_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_time_picker.rs`

## Contract ↔ Svelte

Props, callbacks, ARIA, anatomy align. Size-table reconciled to Svelte. FIXED.

- [x] Deleted the per-size `padding` overrides from §8; moved horizontal padding to a Density adjustments table (`0 calc(control-x ∓ 0.125rem)`). FIXED.
- [x] Size table `min-height` rewritten to Svelte's absolute rems + added per-size indicator font-size. FIXED.
- [x] Documented partial-value prompt strings in §4: `"Select time"` (date set, no time) / `"Select date"` (time set, no date). FIXED.

## GPUI gap (vs Svelte + contract)

- [x] **DONE: overlay rebuilt to composed primitives.** Fake 6×7 grid + weekday header + fake Time box + invented Today/Done bar deleted; now `Calendar::from_spec(...)` (seeded from value.date/visible_month) + composed `TimeField` + contract Time label. Mock px literals removed; time-label typography + body/time gaps applied. Build clean.
- accepted: the old "mock overlay" `px(rem_to_px(1.75))` cell heights / `gap(px(rem_to_px(0.125)))` / `h(px(1.0))` separator no longer exist — the mock was deleted in pass 17 when the overlay was rebuilt to composed `Calendar::from_spec` + `TimeField`. The surviving `rem_to_px(0.875)`/`rem_to_px(0.375)` body/time gaps and `rem_to_px(0.6875)` label size are contract-exact rem (NOT px literals).
- [x] **DONE (pass 22):** Shadow literals replaced by `elevation_overlay_shadow()` (token-driven).
- [x] **DONE:** Indicator was the `calendar` Icon; now `chevron-down` (contract §2/§8 disclosure chevron, matching the sibling date/time pickers).
- [x] **DONE (pass 17):** Time label typography — now `0.6875rem`, weight SEMIBOLD, text-secondary, uppercased `"TIME"`. (`0.04em` tracking is a CSS-only refinement absent from GPUI's text API — accepted.)
- [x] **DONE (pass 17):** Body gap `0.875rem` + time-section gap `0.375rem` now resolved via `rem_to_px` to the contract values (not `space.stack.*`).
- accepted: no ARIA (gpui has no accessibility API) — haspopup/expanded/dialog-role not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] **DONE:** Overlay + open-state handling built out. `js_date_time_picker` now reads `spec.current_open()` (additive spec resolver) and, when open, composes the REAL `js_calendar` (seeded from the value's date/visible-month) + a Time Section — contract Time Label (`0.6875rem`/600/secondary/uppercase) above the composed `js_time_field` (seeded from the value's time) — inside the sibling overlay surface (elevated-98%-over-panel bg, 72%-alpha border, `shadow_md` preset, panel padding). Body gap `0.875rem`, time-section gap `0.375rem`. No mockup.
- [x] **DONE:** Partial-value prompts now match Svelte — date set / no time → `"<date> Select time"`; time set / no date → `"Select date <time>"`; complete → `"<date> <time>"`; empty → placeholder.
- accepted: Trigger gap is `rem_to_px(0.75)` — contract-exact rem (trigger gap `0.75rem`); not a px-literal violation. Indicator font now resolves from the per-size token ladder (`date_picker_indicator_font_rem`).
- accepted: no ARIA channel for haspopup/expanded/dialog role.
- accepted: open/close + outside-click + Escape + calendar/time selection live in the preview event loop, not the component (mirrors the DatePicker build).

## Specimen parity

- Svelte covers: Default, With default value, Disabled, Sizes, Densities (`DateTimePickerSpecimen.svelte`).
- GPUI covers: Default (open-toggle), With default value (open-toggle), **Open (calendar + time)** (static open, composes the real `Calendar` + `TimeInput` section), Disabled, Sizes, Densities. — GPUI specimen complete; open-state demonstrates the REAL composed Calendar + TimeInput (the historic fake overlay was already replaced in pass 17). Jetstream pending engine recovery.
- Jetstream covers: With value, Placeholder, Disabled. — missing: **Sizes** and **Densities** groups; no open/overlay demonstration.

## Notes

- The historic GPUI mock overlay was replaced in pass 17 — the open state now composes the real `Calendar` + `TimeField` (TimeInput). The static "Open (calendar + time)" specimen group renders that real composed surface for review. (This note previously flagged the mock overlay as the biggest defect; that defect is resolved.)
- `consv=gap` driver: contract size-table per-size padding overrides (orthogonality violation, not in Svelte) plus undocumented partial-value prompt strings.
