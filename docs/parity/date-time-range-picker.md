<!-- parity consv=fixed gpui=1 jetstream=1 specimen=ok -->
<!-- specimen: BOTH targets done. GPUI + Jetstream specimens each cover Default, With default
     range, static "Open (range calendar + start/end time)" (composes the real range Calendar +
     paired START/END TimeInput — no fake grid), Disabled, Sizes, Densities; both previews build
     clean (pass 42). -->
<!-- pass 41: Jetstream overlay built — composes real Calendar(range) + paired
     START/END TimeField sections (mirrors GPUI + date_time_picker.rs). Indicator
     now size-scaled (date_picker_indicator_font_rem); min-width/gaps are
     contract-exact rem; surface = elevated 98% / border 72% / shadow_md preset.
     Added current_open() to DateTimeRangePickerSpec (additive). 8 render_probe
     tests cover trigger + open-state composition. Remaining Jetstream: hover-blend
     literal (shared helper). Remaining GPUI: route 72% surface-border alpha via color_mix. -->
<!-- pass 22: overlay shadow now elevation_overlay_shadow() (token). Remaining GPUI: route the 72% surface-border alpha through color_mix. -->
<!-- pass 17: GPUI overlay rebuilt — fake range grid + fake time fields + invented
     Today/Done bar replaced with composed Calendar(range) + two TimeFields (START/END).
     Mock literals removed. Remaining GPUI: elevation-overlay shadow token
     (cross-cutting) + route the 72% surface-border alpha through color_mix. -->
# Parity: DateTimeRangePicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-time-range-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DateTimeRangePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_time_range_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_time_range_picker.rs`
- Spec: `packages/contracts/components/src/date_time_range_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DateTimeRangePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_time_range_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_time_range_picker.rs`

## Contract ↔ Svelte

Props, anatomy, ARIA, and callbacks match (`value`, `defaultValue`, `open`, `defaultOpen`, `placeholder`, `weekStartsOn`, `locale`, `size`, `sizeRole`, `density`, `disabled`, `ariaLabel`; `onValueChange`, `onOpenChange`; trigger `aria-haspopup="dialog"` / `aria-expanded` / `aria-controls`; surface `role="dialog"`). Size table reconciled. FIXED.

- [x] **Size table** rewritten to Svelte's absolute trigger heights (`xs:1.5rem`, `sm:1.75rem`, `md:control-height-md`, `lg:2.75rem`, `xl:3.25rem`) and the per-size `padding` column dropped; horizontal padding moved to a Density adjustments table (`0 calc(control-x ∓ 0.125rem)`). FIXED.
- [x] **Per-size indicator font-size** added (`xs:0.625rem … xl:0.875rem`). FIXED.
- Indicator glyph: Svelte renders `▾`; contract anatomy wording ("disclosure chevron") kept. Cosmetic, no contract change.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] **DONE: overlay rebuilt to composed primitives.** Fake range grid + fake time fields + invented Today/Done bar deleted; now `Calendar::from_spec(...)` in `CalendarMode::Range` (seeded from start/end) + two composed `TimeField`s (START/END TIME). Build clean.
- [x] **DONE: invented action bar removed.**
- [ ] Hardcoded grid/cell literals: `gap(px(rem_to_px(0.125)))` (`:253,267,269`), cell/time/done `h(px(rem_to_px(1.75)))` (`:274,325,364`) — resolve from tokens once real primitives replace the mockup.
- [ ] Separator `h(px(1.0))` raw pixel (`:304`) — use a border-width token.
- [ ] Shadow uses raw `hsla(0.0, 0.0, 0.0, 0.10/0.06)` + `px(4.0)/px(16.0)/px(1.0)` literals (`:385-394`). Contract maps box-shadow to `elevation.overlay`; resolve the elevation token instead of an inline two-layer shadow.
- [ ] Surface border `Hsla { a: border.a * 0.72, ..border }` inline alpha (`:382`) — route the 72% border-mix through `color_mix`/token helper.
- [ ] Time label typography not matched: contract requires label-family, **0.6875rem**, weight 600, 0.04em tracking, **uppercase** (`time_field` label uses plain `label_size`, no weight/tracking/transform; `:316-321`). Apply the time-label token treatment.
- accepted: no ARIA (gpui has no accessibility API) — trigger haspopup/expanded/controls + dialog role not emitted.
- accepted: overlay absolute-positioning posture is platform-owned (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] **DONE: overlay built (composed primitives).** `js_date_time_range_picker` now renders the Surface → Body → Calendar(range) + Times Row composition when `current_open()`. Real `js_calendar` in `CalendarMode::Range` (seeded from start/end dates) + two `js_time_field`s in labelled START/END Time Sections. Mirrors GPUI + the sibling `date_time_picker.rs`. Surface = elevated 98% over panel, border 72% alpha, `shadow_md()` preset (JsEl box-shadow gap). 8 render_probe tests cover the composition.
- [x] **DONE: min-width / gaps are contract-exact rem.** `min_w(rem_to_px(18.0))` is the contract `18rem` (no `dateTimeRangePicker.minWidth` token exists; GPUI fallback-resolves the same value); the trigger `gap(rem_to_px(0.75))` is the contract trigger gap. Contract-exact `rem_to_px` is not a hardcode violation.
- [x] **DONE: indicator size-scaled.** Now uses `date_picker_indicator_font_rem` (xs 0.625 … xl 0.875), the shared sibling-picker indicator ladder. `chevron-down` icon vs Svelte's `▾` glyph is the established accepted icon substitution.
- [ ] Hover blend uses `fill_c.mix(elevated_c, 0.14)` — the contract hover is `color-mix(surface 86%, elevated)`; `0.14` = `1 − 0.86`, the inverted ratio the runtime `Color::mix` helper expects. Shared with every sibling Jetstream picker; a named 86%-semantics helper would be a cross-cutting cleanup.
- accepted: no ARIA channel for haspopup/expanded (documented pattern).
- accepted: overlay interaction (open/close, calendar, time fields) lives in the preview event loop, not the component.
- JsEl gap: time-label letter-spacing (0.04em tracking) — runtime has no letter-spacing; label is pre-uppercased, weight 600, 0.6875rem, text-secondary all applied.

## Specimen parity

- Svelte covers: Default, With default range, Disabled; plus size and density snippets (`DateTimeRangePickerSpecimen.svelte`).
- GPUI covers: Default (toggle-open wired), With default range (toggle-open), **Open (range calendar + start/end time)** (static open), Disabled; plus size/density via `specimen_layout` (`date_time_range_picker.rs`). — GPUI specimen complete; open state composes the REAL range `Calendar` + paired START/END `TimeInput` sections (the historic mock overlay was replaced in pass 17). Jetstream pending engine recovery.
- Jetstream covers: With range, Placeholder, Disabled (`date_time_range_picker.rs`). — missing: **size and density groups** (Svelte/GPUI both demo these via snippets/specimen_layout); **open-overlay state** (trigger-only, so no surface shown).

## Notes

- `consv=gap` driver: the contract §8 size table (calc-based heights + per-size padding) does not match Svelte's absolute heights + density-only padding, and the contract omits the per-size indicator font-size Svelte ships. Both belong in the contract per "Svelte is parity authority."
- `DateTimeRangePickerSpec` is well-modeled — it carries the full prop surface (`value`, `default_value`, `open`, `default_open`, `placeholder`, `week_starts_on`, `locale`, `is_disabled`, `aria_label`, size/role/density) matching the contract. The Rust gaps are rendering-side, not spec-side.
- The historic GPUI mocked overlay (fake grid + invented Today/Done bar) was rebuilt on the real `Calendar`(range) + paired `TimeInput` primitives in pass 17 — this issue is resolved. The static "Open (range calendar + start/end time)" specimen group renders that real composed surface.
