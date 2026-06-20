<!-- parity consv=ok gpui=4 jetstream=6 specimen=gap -->
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

GPUI renders a custom overlay only (no native mode) — accepted per Known Delta. Real gaps:

- [ ] Hardcoded hover shadow color literal `hsla(0.0, 0.0, 1.0, 0.10)` at `select.rs:278` and two dropdown shadow literals `hsla(0.0,0.0,0.0,0.10)`/`hsla(...,0.06)` at `select.rs:430-440` — resolve from elevation/shadow tokens, not raw HSLA.
- [ ] Hardcoded blur/offset px literals in the dropdown `BoxShadow` (`px(4.0)`, `px(16.0)`, `select.rs:431-438`) — derive from an elevation token.
- [ ] No `clearable` / clear button — contract anatomy includes Clear Button (`aria-label="Clear selection"`); spec/`Select` builder has no clearable path.
- [ ] Option `icon` and `description` fields not rendered — contract anatomy has Option Icon + Option Description; GPUI renders only `option.label` (`select.rs:576`).
- accepted: no native `<select>` mode (Known Delta — GPUI has no native select).
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded panel offset literal `height + 2.0` at `select.rs:81` — the `2.0`px gap should be a stack/inline-spacing token (GPUI uses `space.stack.sm`).
- [ ] Hardcoded panel dims: `min_width = rem_to_px(10.0)` and `max_height = rem_to_px(15.0)` at `select.rs:188-189` — should resolve from `size.select.minWidth` / `size.menu.maxHeight` tokens (GPUI resolves these). Also `menu_min_width` prop is ignored.
- [ ] Hover/active not applied to trigger background — only `border_color` shifts on hover (`select.rs:142`); contract focus-within also changes background + box-shadow.
- [ ] No `clearable` / clear button.
- [ ] Option `icon` and `description` fields not rendered (only label + a trailing `check` for selected).
- [ ] Validation state not wired — `ValidationState` is in the spec and the specimen passes `Invalid`, but `js_select` never reads `spec.validation_state` to recolor the border (GPUI does, `select.rs:201-208`).
- accepted: no native mode; interaction (open/select/search) lives in preview event loop.
- accepted: no ARIA channel.

## Specimen parity

- Svelte covers: flat default, native, grouped, searchable, freeform, custom-trigger snippet, clearable, disabled, size matrix, density matrix (`SelectSpecimen.svelte`). Contract mandates 3 (flat / grouped / disabled) — Svelte exceeds.
- GPUI covers: Native (default), Custom non-searchable, Searchable, Searchable-with-groups, Freeform, Rich option rendering, Clearable, Native grouped, Disabled, Validation states, Sizes (11 groups). — missing: nothing material vs contract; broadest coverage of the three.
- Jetstream covers: Default(placeholder), With value, Disabled, Open state, Searchable, Invalid validation, Ghost variant. — missing: **Grouped options** (contract-mandated specimen), and the **Invalid validation** group renders without effect because `js_select` ignores validation state (see Jetstream gap).

## Notes

- Many `rem_to_px(<literal>)` calls in both Rust impls (option padding `0.375`/`0.5`rem, group-label padding) mirror the Svelte CSS, which itself uses hardcoded rem values for these dropdown internals — acceptable as faithful reproduction, but the trigger/panel *dimension* tokens (min-width, max-height, control-height-derived gap) MUST come from named tokens; flagged above where they don't.
- GPUI is the de-facto reference for the custom-overlay path; Jetstream lags it on clearable, icon/description, and validation wiring.
