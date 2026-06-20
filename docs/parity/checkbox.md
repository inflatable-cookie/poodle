<!-- parity consv=ok gpui=4 jetstream=5 specimen=gap -->
# Parity: Checkbox

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/checkbox.md`
- Svelte (authoritative): `packages/svelte/components/src/Checkbox.svelte`
- GPUI: `packages/gpui/components/src/primitives/checkbox.rs`
- Jetstream: `packages/jetstream/components/src/checkbox.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CheckboxSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/checkbox.rs` · jetstream `packages/jetstream/preview/src/specimens/checkbox.rs`

## Contract ↔ Svelte

Props, anatomy, states, and ARIA all align. Indicator/mark md values match: contract states literal `1.125rem`/`0.875rem`; Svelte computes `calc(icon-md ± 0.125rem)` and `icon-md = 1.0rem`, so they resolve identically.

- Indicator radius scales per size in Svelte (xs `0.1875` → xl `0.4375rem`, lines 194/204/216/227). Contract §8 only states the md radius `0.3125rem` and gives no per-size radius. Minor: Svelte is authoritative. **Fix: add the per-size radius column to contract §8 size table.**
- Density gap values: Svelte compact `0.375rem` (line 176), comfortable `space-inline-md` (line 179). Contract §8 lists only the default root gap `space-inline-sm`. Not a divergence, just undocumented. **Fix: note density gap values in contract.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded radius literals `px(rem_to_px(0.1875/0.25/0.3125/0.375/0.4375))` at `checkbox.rs:123-127` — resolve from a radius token, not raw rem floats.
- [ ] Indicator uses raw icon-token size (`size.icon.{xs..xl}`) for width/height at `checkbox.rs:123-127`; Svelte adds `+0.125rem` to each (indicator = icon + 0.125rem). GPUI indicators are ~2px too small per size. **Fix: add the `+0.125rem` indicator offset.**
- [ ] `selectedColor` only applies to fill (`checkbox.rs:137-142`); contract §4 + Svelte (lines 147-150) also tint the **border** to the selected color. GPUI sets `border_color(accent)` only because accent == fill — but a custom `selectedColor` border path is implicit; verify border tracks `selected_color`, not just fill.
- [ ] Focus ring is an approximated border+shadow (`checkbox.rs:211-222`); contract §8 specifies an `outline` with `outline-offset: 0.125rem`. Document as approximation or align offset.
- accepted: no ARIA (gpui has no accessibility API) — `aria_checked="mixed"`, role, accessible name not emitted (contract §10 expects native a11y tree; runtime limit here).

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded radius literal `rem_to_px(0.3125)` at `checkbox.rs:100` and border-width `rem_to_px(0.0625)` at `checkbox.rs:102` — resolve from radius/border-width tokens, not raw rem floats.
- [ ] Indicator radius is fixed at `0.3125rem` for all sizes (`checkbox.rs:100`); Svelte scales radius per size (`0.1875`→`0.4375rem`). **Fix: apply per-size radius.**
- [ ] Indicator size omits the Svelte `+0.125rem` offset — `indicator_size_rem` (lines 25-35) returns icon-default sizes (xs `0.875`, sm `1.0`) where Svelte renders `icon + 0.125rem`. Slots are ~2px small per size.
- [ ] Gap is `control_space_x_rem(density)` (`checkbox.rs:82`) rather than the contract `space-inline-sm` token; does not match Svelte's compact `0.375` / default `space-inline-sm` / comfortable `space-inline-md` ladder.
- [ ] `selected_color` override unsupported — `indicator_fill_token()` resolved directly (`checkbox.rs:78`); no path reads `spec.selected_color` for custom checked fill/border.
- accepted: no ARIA channel (role / `aria-checked=mixed` / readonly) — documented runtime limit.
- accepted: interaction (toggle, Space key, readOnly revert) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Default (3 interactive), States (disabled unchecked/checked, mixed, readOnly), Custom selected color (green/amber), Sizes, Densities (`CheckboxSpecimen.svelte`).
- GPUI covers: Default (3 interactive w/ on_change), States (all four), Custom selected color (green/amber), Sizes, Densities. — missing: nothing material; full coverage.
- Jetstream covers: States (unchecked/checked/mixed), Disabled (unchecked/checked). — missing: **readOnly** state, **Custom selected color** group, **Sizes** group, **Densities** group (`jetstream/.../checkbox.rs`).

## Notes

- `consv=ok`: every contract prop, anatomy part, state, and ARIA attribute is present in Svelte; the two flagged items are contract-documentation omissions (per-size radius, density gaps), not behavioral divergences.
- The recurring Rust gap is the missing `+0.125rem` indicator offset (both GPUI and Jetstream read icon tokens directly), making indicators slightly undersized vs Svelte at every size.
- `selectedColor` border tinting and per-size radius scaling are the two cross-platform visual gaps to prioritize.
