<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
# Parity: DatePicker

> Pass (Rust): GPUI indicator switched to `chevron-down` icon (per-size font, text-secondary); contract §2 wording named the chevron. The earlier "hardcoded shadow literals" todo was stale — the surface already uses `elevation_overlay_shadow()` + `color_mix`. Surface-as-flow-child and missing outside-click remain accepted platform deltas. Jetstream rebuilt: chevron-down icon (was `📅`), root `min_w(14rem)`, `spec.placeholder` (no ellipsis), `current_value()`, disabled branch (opacity + `disabled(true)`), trigger gap `0.75rem`, per-size indicator font, and — per "No Mockups" — composes the REAL `js_calendar` surface when `current_open()` (token bg/border via `color_mix`, `shadow_md()` for elevation). Specimens add Sizes + Densities; With value / Disabled / Open now render their states. JsEl gap: no token→box-shadow path, `shadow_md()` preset substitutes for `elevation-overlay`.

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DatePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DatePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_picker.rs`

## Contract ↔ Svelte

Props, callbacks, ARIA, and anatomy all align. Size token-form reconciled. FIXED.

- [x] Size table (§8) rewritten to Svelte's absolute trigger heights (`xs:1.5rem`, `sm:1.75rem`, `md:control-height-md`, `lg:2.75rem`, `xl:3.25rem`) instead of `calc(±Xrem)`. FIXED.
- [x] Added per-size indicator font-size column (`0.625rem` xs … `0.875rem` xl). FIXED.
- [x] Added a Density adjustments table (trigger horizontal padding `0 calc(control-x ∓ 0.125rem)`); confirms no per-size padding. FIXED.
- Indicator: Svelte renders the `▾` chevron glyph at `font-size: 0.75rem` (md). Contract §2 wording kept. Rust-target glyph divergence is an implementation gap (see below), not a contract change.

## GPUI gap (vs Svelte + contract)

- [x] FIXED Indicator now renders the `chevron-down` Icon (was the `calendar` Icon) with the per-size indicator font (`date_picker_indicator_font_rem`) and `text-secondary` color, matching contract §2 + Svelte's `▾`. Contract §2 wording updated to name the chevron.
- [x] STALE/already-fixed The "hardcoded shadow literals" claim no longer holds: the calendar surface uses `crate::theme_ext::elevation_overlay_shadow()` (token-backed) + `color_mix` for border/background. No raw HSLA or float-px shadow literals remain in `date_picker.rs`.
- accepted: Surface rendered as a flow-child (flex-col + gap) rather than an absolute overlay — anchored-below overlay positioning is an accepted platform delta (§12 Known Deltas).
- accepted: outside-click dismissal lives in the preview event loop, not the component (toggle/Escape on the trigger; preview owns outside-click).
- accepted: no ARIA (gpui has no accessibility API) — `aria_haspopup`/`aria_expanded`/`role="dialog"` not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Indicator is now `ui_element::icon("chevron-down")` (was the `📅` emoji), sized by `date_picker_indicator_font_rem`, `text-secondary` color.
- [x] FIXED Root now wraps the trigger in `min_w(rem_to_px(14.0))` per contract §7/§8.
- [x] FIXED Display uses `spec.placeholder` (default `"Select date"`, no ellipsis) when no value.
- [x] FIXED Reads `spec.current_value()` (honors `default_value`); the "With value" specimen now shows the date.
- [x] FIXED Disabled branch added — `opacity(state.opacity.disabled)` + `disabled(true)`; "Disabled" specimen now renders the state.
- [x] FIXED Trigger gap is `rem_to_px(0.75)` (was `0.25`).
- [x] FIXED (No Mockups) When `current_open()`, composes the REAL `js_calendar` inside a token-resolved surface (radius.surface, `color_mix(elevated 98%, panel)` bg, border-default×0.72, `shadow_md()`, panel-x/y padding). No fake grid.
- accepted: no ARIA channel for haspopup/expanded/dialog role.
- accepted: open/close, outside-click, Escape, and date-selection interaction live in the preview event loop, not the component.
- JsEl gap: the runtime has no token→box-shadow resolution; `shadow_md()` preset stands in for `elevation-overlay` on the surface.

## Specimen parity

- Svelte covers: Default (with selected-value readout), With default value, Disabled, plus Sizes + Densities snippets (`DatePickerSpecimen.svelte`).
- GPUI covers: Default (open-toggle + selected readout, interactive), With default value (open-toggle), Disabled, Sizes, Densities. — closest parity of the three.
- Jetstream covers: Default (placeholder), With default value, Open (real composed Calendar), Disabled, Sizes (xs..xl), Densities — labels aligned to the contract/Svelte set; all groups render the real `js_date_picker` + `DatePickerSpec` (Open composes the real `js_calendar` surface, no fake grid). "With default value" + "Disabled" render their states (`current_value()` / `is_disabled`). Specimen parity holds. No "invalid" group: `DatePickerSpec` has no invalid/error field and the contract defines none — not a renderable state, not faked.

## Notes

- The chevron-vs-calendar-icon indicator split is the cleanest cross-target inconsistency: Svelte `▾`, GPUI `calendar` icon, Jetstream `📅`. Contract §2 says "icon"/"chevron" ambiguously; pick one glyph, fix the contract wording, align all three.
- `consv=gap` driver is the size-table token-form mismatch (absolute rem in Svelte vs `calc()` in contract) plus the undocumented per-size indicator font-size steps.
