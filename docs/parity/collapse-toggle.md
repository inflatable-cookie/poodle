<!-- parity consv=fixed gpui=4 jetstream=6 specimen=gap -->
# Parity: CollapseToggle

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/collapse-toggle.md`
- Svelte (authoritative): `packages/svelte/components/src/CollapseToggle.svelte`
- GPUI: `packages/gpui/components/src/primitives/collapse_toggle.rs`
- Jetstream: `packages/jetstream/components/src/collapse_toggle.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CollapseToggleSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/collapse_toggle.rs` · jetstream `packages/jetstream/preview/src/specimens/collapse_toggle.rs`

## Contract ↔ Svelte

Mostly aligned; the divergences are in the size/density padding model — contract over-specifies vs what Svelte applies.

- Contract §8 size table gives per-size **full `padding`** (`xs` 0.0625rem … `xl` 0.25rem). Svelte applies these as full `padding` for `xs`/`lg`/`xl` (lines 95–105) — OK. The `md` row falls through to base `0.125rem`; prose already notes `sm`/`md` share base. Fine, no change.
- [x] FIXED Contract §8 density table now reads `padding-inline` (not full `padding`), matching Svelte (lines 108-109) and the density-orthogonality rule; added a note that density does not touch vertical padding/height. Tier 2 checklist line updated to `padding-inline`.
- [x] FIXED Compact density row now marked `0.125rem (= base, no change)`, reflecting Svelte's redundant `padding-inline: 0.125rem` for compact.
- Otherwise props, anatomy (`<button>` + Icon), icon-direction logic, `aria-expanded`/`aria-label` defaults, `data-collapsed`/`data-direction`/`data-size`/`data-density` all match contract exactly.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Padding hardcoded flat `px(rem_to_px(0.125))` at `collapse_toggle.rs:127` — ignores the size table (`xs`/`lg`/`xl`) and density (`comfortable` 0.375rem). Resolve padding from a size/density-derived token, not a constant `0.125`.
- [ ] Square-button model: `button_size = icon_size + padding * 2.0` (`:128`) forces equal w/h instead of `padding` on a content-sized inline-flex; with the flat padding this also makes size/density inert on dimensions.
- [ ] Icon forced to `IconSize::Sm` (`:166`) regardless of `resolved_size`; Svelte passes `size={resolvedSize}` so the chevron scales with size. Pass the effective size to the Icon.
- [ ] Hover/active reuse `hover_fill` for `active` (`:160-161`); contract has no active state — Svelte only defines `:hover`. Drop the `active` branch (cosmetic, low priority).
- accepted: no ARIA (gpui has no accessibility API) — `aria-expanded`/`aria-label` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Wrong sizing model: button sized to `control_height_rem(effective_size)` at `collapse_toggle.rs:18` — contract §7 says "sized to icon plus padding", no control-height. Size to icon + resolved padding instead.
- [ ] No padding token at all — element has no `pl`/`pr`/`pt`/`pb`; relies on the oversized square. The contract §8 padding (size + density `padding-inline`) is unimplemented.
- [ ] No hover state (`background` surface-hover, `color` text-default) — contract §8 Root-hover unimplemented (`js_collapse_toggle` never sets a hover treatment).
- [ ] No focus ring — `.focusable()` set (`:26`) but no `outline`/ring color from `focus_ring_color_token()`; contract §6 requires accent focus ring at 0.0625rem offset.
- [ ] No background/`border-radius` is wrong-rooted: `radius` resolved (`:13`) and applied, but no transparent idle `background` token and no density (`comfortable`) branch.
- [ ] Disabled only drops opacity (`:34-37`); contract disabled also sets `cursor: default` — Jetstream leaves `cursor_pointer()` from `:27` active under disabled.
- accepted: click/keyboard toggle lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Directions (4, interactive toggle + state label), Disabled (left/right), Sizes (xs–xl via snippet), Densities (compact/default/comfortable via snippet).
- GPUI covers: Directions (4, interactive), Disabled (left/right), Sizes + Densities via `specimen_layout`. — parity OK.
- Jetstream covers: per-direction collapsed-vs-expanded pairs (4), Disabled (2). — missing: **Sizes** group, **Densities** group, and interactive toggle/state label (no event loop wiring shown).

## Notes

- `consv=fixed`: the contract density table now correctly reads `padding-inline` (was claiming full `padding`); this was a density-orthogonality contract bug, not a Svelte bug. Remaining gpui/jetstream todos are code-side.
- GPUI/Jetstream both collapse size and density into a fixed square; neither reproduces the icon+padding intrinsic sizing. This is the root cause behind their missing size/density visual differentiation.
