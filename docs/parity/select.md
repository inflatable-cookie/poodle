<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- pass: specimen backfill via real js_select/Select builders. Jetstream select.rs
     +Grouped (open: Fruits/Vegetables/Grains section headers + disabled Spinach option,
     contract-mandated), +Clearable (value → clear x), +Selected+open (checkmark indicator),
     +size matrix (xs→xl, with_size). GPUI select.rs +size matrix. Both token-resolved via
     spec — no hand-rolled dropdown boxes. Both previews build clean. -->

<!-- pass: gpui hover-shadow literal dropped (no Svelte basis) + option description + clearable clear button; jetstream panel-offset/dims tokens + menu_min_width + hover bg + clearable + description + validation wiring; both probe/build-verified -->
# Parity: Select

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/select.md`
- Svelte (authoritative): `packages/svelte/components/src/Select.svelte`
- GPUI: `packages/gpui/components/src/primitives/select.rs`
- Jetstream: `packages/jetstream/components/src/select.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SelectSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/select.rs` · jetstream `packages/jetstream/preview/src/specimens/select.rs`

## Contract ↔ Svelte

Svelte implements the full contract surface (native + custom modes, searchable, freeform, grouped, lazy `loadOptions`/`loadKey`, clearable, `menuMinWidth`, ghost variant, validation states, all three snippet props, hidden form input, size/density). No material divergence found; both directions consistent. `consv=ok`.

- Minor: contract §8 root uses `grid-template-columns: minmax(0,1fr) auto`; Svelte uses `minmax(0,1fr)` (single column) with an absolutely-positioned indicator (lines 724-725, 775-784). Same visual result, implementation detail — note only, no fix.

## GPUI gap (vs Svelte + contract)

GPUI renders a custom overlay only (no native mode) — accepted per Known Delta. All real gaps closed:

- [x] FIXED hover shadow literal `hsla(0.0, 0.0, 1.0, 0.10)` at the trigger removed — Svelte Select has **no `:hover` box-shadow** on the trigger (only bg/border shift; focus-within owns the shadow). The white-highlight shadow was a GPUI invention with no Svelte basis; dropped. The bg/border hover shift is retained.
- [x] ALREADY OK the dropdown listbox shadow uses `crate::theme_ext::elevation_overlay_shadow()` (token-resolved `ELEVATION_OVERLAY`); the `hsla`/`px(4.0)`/`px(16.0)` literals the prior pass flagged at lines 430-440 were already removed.
- [x] FIXED `clearable` clear button — pill-backed `x` icon rendered in the trigger when `clearable && value selected && !disabled`; fires `on_change(default_value)`. (Svelte `handleClear` → `clearValue = defaultValue ?? ""`.)
- [x] FIXED option `description` rendered as a secondary line below the label (text-secondary, `rem_to_px(0.6875)` — Svelte exact).
- note: option `icon` is **not renderable** — `ChoiceOption` (shared spec) has no `icon` field (only `description`). Adding one is a Svelte-authority-driven spec change out of scope for this pass; flagged as a spec-data gap, not a rendering gap.
- accepted: no native `<select>` mode (Known Delta — GPUI has no native select).
- accepted: no ARIA (gpui has no accessibility API).
- preview-loop: open/select/keyboard wiring lives in the preview event loop (`on_toggle`/`on_change`/`on_search_change` channels exist on the builder).

## Jetstream gap (vs Svelte + contract)

All real gaps closed:

- [x] FIXED panel offset literal `height + 2.0` → `height + resolve_px(theme, "space.stack.sm")` (GPUI parity).
- [x] FIXED panel dims resolve from tokens: `size.select.minWidth` / `size.menu.maxHeight`; `menu_min_width` prop now honored (CSS length parsed via `parse_css_length_to_px`, overrides the default min-width).
- [x] FIXED trigger hover now shifts both border (toward text) **and** background (toward elevated), matching the contract focus-within direction.
- [x] FIXED `clearable` clear button — pill-backed `x` icon in the trigger when `clearable && value selected && !disabled`.
- [x] FIXED option `description` rendered as a stacked secondary line (text-secondary, `rem_to_px(0.6875)`); row height relaxes for described rows.
- [x] FIXED validation state wired — `js_select` now reads `spec.validation_state` and recolors the closed trigger border (Invalid→danger, Valid→success, Pending→accent), mirroring GPUI + TextInput.
- note: option `icon` not renderable — no `icon` field on `ChoiceOption` (spec-data gap, same as GPUI).
- accepted: no native mode; interaction (open/select/search) lives in preview event loop.
- accepted: no ARIA channel.

## Specimen parity

- Svelte covers: flat default, native, grouped, searchable, freeform, custom-trigger snippet, clearable, disabled, size matrix, density matrix (`SelectSpecimen.svelte`). Contract mandates 3 (flat / grouped / disabled) — Svelte exceeds.
- GPUI covers: Native (default), Custom non-searchable, Searchable, Searchable-with-groups, Freeform, Rich option rendering, Clearable, Native grouped, Disabled, Validation states, **Sizes (xs→xl)**. — missing: nothing material vs contract; broadest coverage of the three.
- Jetstream covers: Default(placeholder), With value, Disabled, Open state, Searchable, Invalid validation, Ghost variant, **Grouped (open: section headers + disabled option)**, **Clearable (clear x)**, **Selected+open (checkmark)**, **Sizes (xs→xl)**. — all three contract-mandated specimens (flat / grouped / disabled) now covered; validation now wired (see Jetstream gap — fixed).

## Notes

- Many `rem_to_px(<literal>)` calls in both Rust impls (option padding `0.375`/`0.5`rem, group-label padding) mirror the Svelte CSS, which itself uses hardcoded rem values for these dropdown internals — acceptable as faithful reproduction, but the trigger/panel *dimension* tokens (min-width, max-height, control-height-derived gap) MUST come from named tokens; flagged above where they don't.
- GPUI is the de-facto reference for the custom-overlay path; Jetstream lags it on clearable, icon/description, and validation wiring.
