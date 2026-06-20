<!-- parity consv=fixed gpui=2 jetstream=3 specimen=gap -->
<!-- NOTE: the GPUI "overlay mockup / empty calendar box / hsla shadow / trigger segment /
     invented dividers" bullets in the GPUI-gap section below are STALE — all fixed in
     pass 23 (composed Calendar+TimeField+TimeZoneSelect, elevation_overlay_shadow, trigger
     cleanup). Pass 25: shared spec remodeled flat value/time_zone → structured
     ZonedDateTimeValue + props (placeholder/defaultValue/open/weekStartsOn/locale/
     timeZoneOptions/ariaLabel); GPUI seeds Calendar/TimeField/TimeZoneSelect from the
     structured value (partial-value states now representable); Jetstream updated
     (build-unverified — renderer down). specs 53 pass, GPUI clean. Remaining GPUI:
     field-label typography + indicator glyph. -->
# Parity: DateTimeZonePicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-time-zone-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DateTimeZonePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_time_zone_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_time_zone_picker.rs`
- Spec: `packages/contracts/components/src/date_time_zone_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DateTimeZonePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_time_zone_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_time_zone_picker.rs`

## Contract ↔ Svelte

`consv=fixed`. Svelte matches the contract on props (`value`, `defaultValue`, `open`, `defaultOpen`, `placeholder`, `weekStartsOn`, `locale`, `timeZoneOptions`, `size`, `sizeRole`, `density`, `disabled`, `ariaLabel`), the `ZonedDateTimeValue`/`TimeZoneOption` types, anatomy (Calendar + Fields → Time field + Time-zone field), ARIA, and callbacks. The size table diverged (same pattern as the range picker) — now reconciled:

- [x] FIXED **Size table mismatch.** Contract §8 rewritten to Svelte's **absolute** heights (`xs:1.5rem`, `sm:1.75rem`, `md` via `--poodle-size-control-height-md`, `lg:2.75rem`, `xl:3.25rem`; `DateTimeZonePicker.svelte:334-358`); per-size padding column removed and replaced with a density-driven padding table (`:360-361`). Per-size trigger font-size (`sm:0.8125rem` added).
- [x] FIXED **Per-size indicator font-size.** Contract §8 now scales the indicator (`xs:0.625rem` … `xl:0.875rem`, base `0.75rem`; `:337,342,353,358`); Tier-2 checklist + §9 notes updated to match.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] DONE **Value model remodeled.** `poodle-specs` now has `ZonedDateTimeValue { date, time, time_zone }` (`types.rs`) and `DateTimeZonePickerSpec.value: Option<ZonedDateTimeValue>` + `default_value` with `current_value()`/`is_placeholder()` accessors. GPUI seeds Calendar from `value.date`, TimeField from `value.time`, TimeZoneSelect from `value.time_zone` — partial values representable.
- [x] DONE **Spec gained contract props.** Added `placeholder` (default `"Select date, time, and zone"`), `default_value`, `default_open`, `week_starts_on`, `locale`, `time_zone_options: Vec<TimeZoneOption>`, `aria_label` with `with_*` builders + `current_open()`, mirroring `DateTimeRangePickerSpec`. GPUI trigger uses `spec.placeholder` instead of a hardcoded string.
- [ ] **Overlay is a hand-coded mockup.** Contract §2 requires composing `Calendar` + `TimeInput` + `TimeZoneSelect`. GPUI hand-builds a weekday row + empty `min_h(11.25rem)` placeholder for the calendar (`:200-226`), a plain text time line (`:229-245`), and a plain text timezone line (`:248-268`). No real primitives. Violates "No Mockups" — replace with composed Calendar/TimeInput/TimeZoneSelect.
- [ ] Calendar body is an empty `div().min_h(px(rem_to_px(11.25)))` placeholder (`:226`) — literally a blank box. Remove once real Calendar composes in.
- [ ] Shadow uses raw `hsla(0.0, 0.0, 0.0, 0.10/0.06)` + `px(4.0)/px(16.0)/px(1.0)` literals (`:279-288`). Contract maps box-shadow to `elevation.overlay`; resolve the token.
- [ ] Surface border `Hsla { a: border.a * 0.72, ..border }` inline alpha (`:276`) — route the 72% mix through `color_mix`/token helper.
- [ ] **Field-label typography not matched.** Contract requires label-family, **0.6875rem**, weight 600, 0.04em tracking, **uppercase**; the "Time"/"Time zone" labels use plain `label_size` with no weight/tracking/transform (`:236-238,254-257`). Apply the field-label token treatment.
- [ ] Trigger shows the timezone as a separate inline segment between value and icon (`:143-150`) — not in the contract trigger anatomy (Value + Indicator only). Either fold the zone into the formatted value string or remove the extra segment.
- [ ] Surface section structure (calendar / divider / time / divider / timezone vertical stack with `border_b_1` dividers, `:294-298`) invents dividers not in the contract anatomy (contract Body → Fields, no rules). Remove invented dividers.
- accepted: no ARIA (gpui has no accessibility API).
- accepted: overlay positioning + timezone-registry ordering platform-owned (contract Known Deltas).

## Jetstream gap (vs Svelte + contract)

- [x] DONE (build-unverified) **Value model remodeled (shared spec).** Now reads `spec.current_value()` + `value.date/time/time_zone` and `spec.placeholder`; partial values representable. Renderer crate is down (external `encode.rs` break) so this is build-unverified — edits cross-checked against the new spec by reading.
- [x] DONE (build-unverified) **Contract props added to shared spec.** Jetstream now uses `spec.placeholder`; the other props (`default_value`/`open`/`week_starts_on`/`locale`/`time_zone_options`/`aria_label`) exist on the spec for runtime wiring.
- [ ] **Overlay not rendered** — trigger only (`:93-102`). Calendar + TimeInput + TimeZoneSelect surface deferred to runtime. Acceptable per the trigger-only pattern, but the composed surface must exist; tracked as a gap.
- [ ] Hardcoded `gap(rem_to_px(0.75))` (`:71`) and `min_w(rem_to_px(18.0))` (`:95`) — resolve from tokens (GPUI uses a `minWidth` token; mirror it).
- [ ] Hover blend raw `fill_c.mix(elevated_c, 0.14)` (`:42`) — confirm the helper matches contract `color-mix(surface 86%, elevated)` and route via a named helper.
- [ ] Indicator `chevron-down` icon (`:86`) not size-scaled, where Svelte scales the `▾` glyph per size. Note for visual parity.
- accepted: no ARIA channel (documented pattern).
- accepted: overlay interaction (open/close, composed fields) lives in the preview event loop.

## Specimen parity

- Svelte covers: Default, With default value (`{ date, time, timeZone }`), Disabled; plus size and density snippets (`DateTimeZonePickerSpecimen.svelte`).
- GPUI covers: Default (toggle-open, composed overlay), With default value, Disabled; plus size/density via `specimen_layout` (`date_time_zone_picker.rs`). Specimens now construct structured `ZonedDateTimeValue::new(date, time, time_zone)` per the contract `{ date, time, timeZone }` shape (Disabled shows the placeholder per contract). — Calendar/TimeField/TimeZoneSelect compose for real.
- Jetstream covers: With value, Placeholder, Disabled (`date_time_zone_picker.rs`) using structured `ZonedDateTimeValue` data. — missing: **size and density groups**; **open-overlay state** (trigger-only).

## Notes

- `consv=gap` driver: same size-table divergence as the range picker (contract calc-heights + per-size padding vs Svelte absolute heights + density-only padding) plus the missing per-size indicator scale.
- **Spec is the root problem here**, unlike the range picker. `DateTimeZonePickerSpec` is under-modeled: flat `value`/`time_zone` strings instead of `ZonedDateTimeValue`, and it lacks `placeholder`/`defaultValue`/`open`/`weekStartsOn`/`locale`/`timeZoneOptions`/`ariaLabel`. Both Rust targets and both specimens inherit the mismodel. Fix the spec first (add `ZonedDateTimeValue` + the missing fields), then the component/overlay work.
- Both GPUI pickers (range and zone) share the same mockup anti-pattern: invented overlay scaffolding (blank calendar boxes, fake fields, dividers, action bars) standing in for real composed primitives. Treat them together in the fix pass.
