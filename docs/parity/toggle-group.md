<!-- parity consv=fixed gpui=4 jetstream=6 specimen=gap -->
# Parity: ToggleGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/toggle-group.md`
- Svelte (authoritative): `packages/svelte/components/src/ToggleGroup.svelte`
- GPUI: `packages/gpui/components/src/primitives/toggle_group.rs`
- Jetstream: `packages/jetstream/components/src/toggle_group.rs`
- Spec: `packages/contracts/components/src/toggle_group.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ToggleGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/toggle_group.rs` · jetstream `packages/jetstream/preview/src/specimens/toggle_group.rs`

## Contract ↔ Svelte

Multiple §8 token values were stale vs Svelte; Svelte is authoritative — contract fixed.

- [x] FIXED **Item background mismatch.** §8 said `surface 72% / background-elevated`; Svelte resolves `surface 93% / text-primary` (`ToggleGroup.svelte:165-168`). Contract §8 item background → `surface 93%, text-primary` (with treatment-var fallback). Surface-elevation note + Tier-2 updated.
- [x] FIXED **Selected background mismatch.** §8 said flat `accent-base 22% / transparent`; Svelte layers a `linear-gradient(accent 22% → accent 22%)` over the unselected fill (`:183-197`), keeping the surface base. Contract selected background → gradient tint layered over the item fill. GPUI notes + Tier-2 updated.
- [x] FIXED **Font-size source.** §8 hardcoded `0.75rem`; Svelte uses `var(--poodle-typography-label-size)` (`:173`). Contract → label-size token. Tier-2 updated.
- [x] FIXED **Treatment-variable indirection.** Documented `--poodle-treatment-interactive-*` (border/fill/radius/shadow/border-active) as the brand-raised theming layer with color-mix fallbacks in §8 tables + §9 Svelte Notes.
- `value` default: contract §3 `undefined`; Svelte `undefined` (`:26`) — matches. `defaultValue` `null` matches. No change.
- [ ] (spec, not contract↔Svelte) **`allowDeactivation` absent from Rust spec.** Contract §3/§5 document it and Svelte implements it (`:71-73`); `ToggleGroupSpec` (`toggle_group.rs:34-45`) lacks the `allow_deactivation` field. Contract side is correct — add the spec field in code (left for Rust pass).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `allowDeactivation` support — single-select cannot clear to `null` (spec field absent; builder has no `allow_deactivation`). Re-clicking the active item is a no-op.
- [ ] Roving-arrow handler **changes selection** rather than only moving focus (`toggle_group.rs:199-213` calls `arrow_handler` which is the change callback). Svelte arrows move roving focus without committing; GPUI commits on every arrow. **Decouple focus movement from selection.**
- [ ] Selected fill divergence from Svelte gradient: GPUI mixes `accent` over `item_fill` at 0.22 (`:121`) which approximates Svelte, but unselected `item_fill` already uses the correct `surface 93%, text-primary` mix (`:117`) — keep, just confirm against the gradient layering.
- [ ] No multiple-mode role/state distinction surfaced (radio vs button) — accepted-ARIA bucket, but selection logic still treats all modes identically via `is_selected`; verify multiple-mode toggle-off works through the change callback.
- accepted: no ARIA (gpui has no accessibility API) — radiogroup/group + aria-checked/pressed not emitted.
- accepted: transition timing (180ms ease) not modeled (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] **DONE: item background** — now `surface.mix(text_primary, 0.93)` (Svelte color-mix(surface 93%, text-primary)), was the stale surface/elevated 72%. Locked by `item_fill_uses_svelte_surface_text_mix` probe test.
- [x] **DONE: selected fill** — now `accent.mix(item_fill, 0.22)` (accent tinted over the item fill), was accent-over-transparent.
- [ ] Gap is the ad-hoc `control_space_x_rem(density) * 0.5` heuristic (`:38`). Contract/Svelte gap is density-driven (`0.1875 / 0.25 / 0.375 rem`); GPUI matches Svelte exactly. **Resolve from density directly, drop `* 0.5`.**
- [ ] Hardcoded `rem_to_px(0.25)` item-height reduction (`:42`), `rem_to_px(0.75)` font-size (`:63`), `rem_to_px(0.0625)` border-width (`:64`) — font-size should resolve from the label-size token (Svelte uses `typography-label-size`); the 0.25/0.0625 literals belong in tokens.
- [ ] No `font-family` token applied — contract §8 + Svelte set `typography-label-family`; Jetstream sets weight/size only.
- [ ] No `allowDeactivation` (spec field absent; builder cannot clear single-select).
- [ ] No interaction/callback — `js_toggle_group` is render-only, no click/key handler and no `on_change` channel (GPUI at least wires click + arrows). Toggle is not interactive.
- [ ] No size override in specimen path and no roving-focus modeling.
- accepted: no ARIA channel (radiogroup/group, aria-checked/pressed).
- accepted: interaction wiring may live in preview loop, but here there is no wiring at all.

## Specimen parity

- Svelte covers: Single selection (+ live hint), Four options, Multiple selection (+ hint), Disabled, Sizes snippet, Densities snippet (`ToggleGroupSpecimen.svelte`).
- GPUI covers: Single (+ live value), Four options, Multiple, Semantic role offsets, Disabled group, **Disabled item** (broader than Svelte). — missing: Sizes group, Densities group.
- Jetstream covers: Single, Multiple, With disabled item, Fully disabled. — missing: **Four options**, **Sizes**, **Densities**, and uses B/I/U glyph options instead of the contract Grid/List/Board set.

## Notes

- `consv=gap` driver is stale §8 token math (item background `72%/elevated` → should be `93%/text-primary`; selected tint should layer over fill, not transparent) plus the missing `allow_deactivation` spec field. Per "Svelte is parity authority", update the contract for the first two and add the spec field for the third.
- Jetstream is the worse target: wrong fill formulas (copies stale contract), no interaction, hardcoded dimensions. GPUI is token-clean but couples arrow-focus to selection.
- Specimen content drift: Jetstream's B/I/U set is fine functionally but diverges from the contract's documented Grid/List/Board / alignment specimens — minor.
