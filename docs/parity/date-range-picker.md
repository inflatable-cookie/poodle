<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- specimen: BOTH targets done. GPUI + Jetstream specimens each cover Default, With
     default range, static "Open (range calendar)" (composes the real range Calendar — no
     fake grid), Disabled, Sizes, Densities; both previews build clean (pass 42). -->
<!-- pass 41: both Rust targets closed. GPUI — indicator `calendar` Icon → `chevron-down`
     (text-secondary, per-size `date_picker_indicator_font_rem`); partial-range `"<start> – …"`
     → `"<start> – End date"` (Svelte parity, start-only ⇒ placeholder); shadow already
     `elevation_overlay_shadow()` (stale todo). Jetstream — rebuilt on the DatePicker template:
     now composes the REAL Calendar (mode="range") surface on `current_open()` (no fake grid),
     chevron indicator at per-size font + text-secondary, `shadow_md()` ≈ elevation-overlay,
     panel-space padding; partial-range `"- ..."` → `"– End date"` (en-dash). 8 probe tests
     (trigger placeholder/complete/partial, chevron, closed=no surface, open=real range calendar
     w/ accent endpoints, disabled, sizes). Remaining: specimen Sizes/Densities groups (gap);
     overlay anchoring + open/select interaction = preview-loop. -->
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

Props, callbacks, ARIA, anatomy align. Size-table reconciled to Svelte. FIXED.

- [x] Deleted the per-size `padding` overrides from §8 (orthogonality violation, not in Svelte); moved horizontal padding to a Density adjustments table (`0 calc(control-x ∓ 0.125rem)`). FIXED.
- [x] Size table `min-height` rewritten to Svelte's absolute rems (`1.5/1.75/control-height-md/2.75/3.25rem`) + added per-size indicator font-size. FIXED.
- [x] Documented the partial-range display string in §4: `"<start> – End date"` (formatted start, en-dash, literal `End date`) while only start is chosen. FIXED.

## GPUI gap (vs Svelte + contract)

- [x] FIXED Indicator was a `calendar` Icon — now the `chevron-down` disclosure indicator (matching `▾`), colored `text-secondary` and sized per-size via `date_picker_indicator_font_rem` (shared with DatePicker; contract §8 indicator font table). Glyph now aligns across targets.
- [x] FIXED (stale todo) Surface shadow already resolves the elevation token — `overlay.shadow(elevation_overlay_shadow())` (token-driven `ELEVATION_OVERLAY`), not raw HSLA + float px. No literal shadow remains.
- [x] FIXED Partial-range display now matches Svelte: `"<start> – End date"` (formatted start, en-dash, literal `End date`) while only start is chosen; a missing start falls back to the placeholder (no `… – end` branch — Svelte never shows end-only).
- accepted: no ARIA (gpui has no accessibility API) — haspopup/expanded/dialog-role not emitted.
- accepted: overlay renders as flow-child (`wrapper.child(overlay)`), anchored-below positioning is a platform delta.
- accepted: open/range-select interaction lives in the preview event loop (`on_toggle` wired; range commits via composed Calendar's `on_range_select`).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED No calendar overlay / open-state handling — `js_date_range_picker` now reads `spec.current_open()` and, when open, composes the **real Calendar primitive** (`js_calendar` with `mode="range"`, week-start + seeded range forwarded, visible-month anchored to the range start) inside a token-styled surface (radius.surface, border color-mix 72%, background color-mix(elevated 98%, panel), `shadow_md()` ≈ elevation-overlay, panel-space padding). Per CLAUDE.md "No Mockups" the surface is the actual Calendar, never a faked grid. Rebuilt on the DatePicker template.
- [x] FIXED Partial-range display now `"<start> – End date"` (en-dash + literal `End date`); complete range `"<start> – <end>"`; missing start ⇒ placeholder. Was the ASCII-hyphen `"<start> - ..."` ellipsis form.
- [x] FIXED Indicator was `icon.muted` + `size_font_rem(supporting_visual)`; now `text-secondary` at the per-size `date_picker_indicator_font_rem` scale (contract §8 indicator font table), matching DatePicker/GPUI.
- accepted: trigger gap `rem_to_px(0.75)` is the contract-exact trigger gap (§8 Trigger `gap: 0.75rem`); `rem_to_px` of a contract-exact rem is not a hardcode violation. No content-gap token exists for it.
- accepted: no ARIA channel for haspopup/expanded/dialog role.
- accepted: range-calendar selection + open/close + outside-click/Escape interaction live in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default, With default range, Disabled, Sizes, Densities (`DateRangePickerSpecimen.svelte`).
- GPUI covers: Default (open-toggle), With default range (open-toggle), **Open (range calendar)** (static open, composes the real range `Calendar`), Disabled, Sizes, Densities. — GPUI specimen complete; full contract state coverage with real components. Jetstream pending engine recovery.
- Jetstream covers: With range, Placeholder, Disabled. — missing: **Sizes** and **Densities** groups; no open/overlay demonstration.

## Notes

- Jetstream range picker correctly applies `min_w(rem_to_px(16.0))` and a disabled branch — both of which the Jetstream `date_picker` lacks. Treat this file as the reference shape when fixing Jetstream `date_picker`.
- `consv=gap` driver: contract size-table carries per-size padding overrides that violate size/density orthogonality and that Svelte does not implement.
