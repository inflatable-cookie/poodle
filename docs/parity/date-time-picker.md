<!-- parity consv=fixed gpui=2 jetstream=4 specimen=gap -->
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
- [ ] Hardcoded pixel literals in the mock overlay: `px(rem_to_px(1.75))` cell heights (`date_time_picker.rs:256,304,334`), `gap(px(rem_to_px(0.125)))` (lines 235,249,251), `h(px(1.0))` separator (line 286). None resolve from size/space tokens.
- [ ] Hardcoded shadow literals `hsla(0.0,0.0,0.0,0.10)`/`0.06` + `px(16.0)`/`px(4.0)` at `date_time_picker.rs:355-365`. Contract surface shadow = `var(--poodle-elevation-overlay)`; resolve from elevation token.
- [ ] Indicator renders `calendar` Icon (`date_time_picker.rs:203`) not the `▾` chevron Svelte/contract use. Align glyph.
- [ ] Time label typography wrong: contract §8 requires label-family, `0.6875rem`, weight 600, `0.04em` tracking, uppercase. GPUI renders plain `"Time"` at `label_size`, no weight/tracking/transform (`date_time_picker.rs:296-299`). Apply the contract time-label tokens.
- [ ] Body/time-section gaps wrong: contract body gap `0.875rem`, time-section gap `0.375rem`; GPUI uses `space.stack.md/sm` ad-hoc (`date_time_picker.rs:229,373`). Resolve to the contract gap values.
- accepted: no ARIA (gpui has no accessibility API) — haspopup/expanded/dialog-role not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No calendar+time overlay + no open-state handling: `js_date_time_picker` emits only the trigger; never reads open state. Overlay/interaction must live in preview event loop — none present in the specimen, so the composed Calendar + TimeInput surface is unreachable.
- [ ] Partial-value display joins `date` + `time` with a space and trims (`date_time_picker.rs:43-49`); does not emit Svelte's `"Select time"`/`"Select date"` partial prompts. Match Svelte's partial-value strings.
- [ ] Trigger gap is `rem_to_px(0.75)` literal (`date_time_picker.rs:63`) — fine numerically; resolve from a content-gap token for token-form consistency. Low priority.
- accepted: no ARIA channel for haspopup/expanded/dialog role.
- accepted: calendar + time-field surface + interaction live in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default, With default value, Disabled, Sizes, Densities (`DateTimePickerSpecimen.svelte`).
- GPUI covers: Default (open-toggle → fake overlay), With default value (open-toggle), Disabled, Sizes, Densities. — open-state demonstrates a mockup, not real Calendar/TimeInput.
- Jetstream covers: With value, Placeholder, Disabled. — missing: **Sizes** and **Densities** groups; no open/overlay demonstration.

## Notes

- The GPUI mock overlay is the single biggest defect across all three assigned components: it hides the missing Calendar/TimeInput composition behind hardcoded placeholder UI, which CLAUDE.md flags as "worse than no specimen". Until the spec resolves a real composed surface, the open-state block should be gutted to the real primitives or left closed.
- `consv=gap` driver: contract size-table per-size padding overrides (orthogonality violation, not in Svelte) plus undocumented partial-value prompt strings.
