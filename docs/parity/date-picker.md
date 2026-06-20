<!-- parity consv=gap gpui=4 jetstream=6 specimen=gap -->
# Parity: DatePicker

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/date-picker.md`
- Svelte (authoritative): `packages/svelte/components/src/DatePicker.svelte`
- GPUI: `packages/gpui/components/src/primitives/date_picker.rs`
- Jetstream: `packages/jetstream/components/src/date_picker.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DatePickerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/date_picker.rs` · jetstream `packages/jetstream/preview/src/specimens/date_picker.rs`

## Contract ↔ Svelte

Props, callbacks, ARIA, and anatomy all align. Divergences are in the size token-form.

- Size table (§8) specifies `xs/sm/lg/xl` `min-height` as `calc(var(--poodle-size-control-height) ± Xrem)`. Svelte hardcodes absolute rem heights instead (`1.5rem`, `1.75rem`, `2.75rem`, `3.25rem` at `DatePicker.svelte:239,242,251,254`). Same resolved values for the default control-height, but Svelte's form ignores the token. **Fix: reconcile — either contract documents absolute rems or Svelte uses the calc form. Svelte authoritative → update contract size table to the absolute-rem values Svelte ships.**
- Indicator: contract §2 calls it a "decorative disclosure icon"; Svelte renders the `▾` chevron glyph (`DatePicker.svelte:149`) at `font-size: 0.75rem` per size. Consistent. Note both Rust targets deviate from this glyph (see gaps below).
- Svelte adds per-size indicator font-size steps (`0.625rem` xs … `0.875rem` xl, lines 241–256) not enumerated in the contract size table. **Fix: add indicator font-size column to contract §8 size table.**

## GPUI gap (vs Svelte + contract)

- [ ] Indicator deviates from Svelte: renders a `calendar` Icon (`date_picker.rs:206`) instead of the `▾` chevron glyph the contract anatomy + Svelte use. Either align GPUI to chevron or amend contract; pick one and make all three match.
- [ ] Hardcoded shadow literals: `hsla(0.0,0.0,0.0,0.10)` / `hsla(0.0,0.0,0.0,0.06)` plus `px(16.0)`/`px(4.0)` blur at `date_picker.rs:253-263`. Contract surface shadow is `var(--poodle-elevation-overlay)` — resolve from an elevation token, not raw HSLA + float px.
- [ ] Surface is rendered as a flow-child (`wrapper.child(cal_surface)`, `date_picker.rs:267`), not an absolutely-positioned overlay at `top: calc(100% + 0.375rem)`. Anchored-below overlay positioning is an accepted platform delta, but the calendar currently pushes layout instead of floating; confirm it visually anchors.
- [ ] No outside-click dismissal in the component (Svelte closes on document `mousedown` outside root). Toggle/Escape live on the trigger; outside-click is absent. Mark accepted if preview owns it, else add.
- accepted: no ARIA (gpui has no accessibility API) — `aria_haspopup`/`aria_expanded`/`role="dialog"` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Indicator is an emoji literal `📅` (`date_picker.rs:35`) — diverges from both the contract chevron and the sibling Jetstream pickers (range/datetime use `ui_element::icon("chevron-down")`). Replace with the registry chevron icon for cross-target + cross-component consistency.
- [ ] Missing root `min-width: 14rem` — `js_date_picker` returns the bare trigger with no `min_w` wrapper (`date_picker.rs:28`), unlike `js_date_range_picker`/`js_date_time_picker` which apply `min_w(rem_to_px(16.0))`. Add `min_w(rem_to_px(14.0))` per contract §7/§8 root.
- [ ] Display fallback hardcodes `"Select date..."` (`date_picker.rs:25`) instead of `spec.placeholder` (contract default `"Select date"`). Use `spec.placeholder`; also drop the trailing ellipsis that diverges from Svelte.
- [ ] Reads `spec.value` directly (`date_picker.rs:25`) instead of `spec.current_value()` — ignores `default_value`/`open` resolution that the spec exposes and that the specimen relies on (the "With value" specimen passes `with_default_value`, so the trigger shows the placeholder, not the date). Use `current_value()`.
- [ ] No disabled treatment: `is_disabled` is never read in `js_date_picker` — no opacity reduction, no `disabled(true)`. Contract requires reduced opacity (`state.opacity.disabled`) + non-interactive. The specimen's "Disabled" group renders identically to enabled. Add disabled branch (range/datetime impls already do this).
- [ ] Gap is `rem_to_px(0.25)` literal (`date_picker.rs:33`); contract trigger gap is `0.75rem`. Use `0.75` or a content-gap token.
- accepted: no ARIA channel for haspopup/expanded/dialog role.
- accepted: open-state calendar surface + interaction live in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default (with selected-value readout), With default value, Disabled, plus Sizes + Densities snippets (`DatePickerSpecimen.svelte`).
- GPUI covers: Default (open-toggle + selected readout, interactive), With default value (open-toggle), Disabled, Sizes, Densities. — closest parity of the three.
- Jetstream covers: Empty, With value, Disabled, Open. — missing: **Sizes** and **Densities** groups (Svelte/GPUI show both); "With value" + "Disabled" currently mis-render because the component ignores `current_value()`/`is_disabled` (see gaps), so they don't demonstrate their states.

## Notes

- The chevron-vs-calendar-icon indicator split is the cleanest cross-target inconsistency: Svelte `▾`, GPUI `calendar` icon, Jetstream `📅`. Contract §2 says "icon"/"chevron" ambiguously; pick one glyph, fix the contract wording, align all three.
- `consv=gap` driver is the size-table token-form mismatch (absolute rem in Svelte vs `calc()` in contract) plus the undocumented per-size indicator font-size steps.
