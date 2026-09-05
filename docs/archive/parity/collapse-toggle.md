<!-- parity consv=fixed gpui=0 jetstream=1 specimen=ok pass=41 -->
<!-- specimen=ok: jetstream specimen now adds With-label, Sizes (xs–xl), Densities groups
     to match GPUI/Svelte. GPUI preview builds clean. The poodle-jetstream-preview crate (incl.
     these specimens) compiled clean too, but a full Jetstream preview build is currently blocked
     by an external sibling-repo break: /Dev/projects/jetstream/crates/jetstream-platform/src/gpu.rs:61
     (.ok_or on a Result during an in-progress wgpu upgrade) — unrelated to Poodle specimens.
     Component is icon-only per anatomy, so "with label" is a host-side text label beside the
     toggle (matches Svelte specimen). -->
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

- [x] FIXED Padding now resolves from `spec.padding_rem()` (size table) for vertical and `spec.padding_inline_rem()` (density) for horizontal via `px`/`py` — no flat constant.
- [x] FIXED Dropped the square `button_size` model; the button is now a content-sized inline-flex (`px`/`py` only), so size/density drive real dimensions.
- [x] FIXED Icon now uses `spec.effective_icon_size()` (scales with the effective control size) instead of forced `IconSize::Sm`.
- [x] FIXED Dropped the `active` branch; only `:hover` remains, matching the contract/Svelte.
- accepted: no ARIA (gpui has no accessibility API) — `aria-expanded`/`aria-label` not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Dropped the `control_height_rem` square; the button is now sized to icon + resolved padding (`px`/`py`), per contract §7.
- [x] FIXED Padding now applied: `px(spec.padding_inline_rem())` + `py(spec.padding_rem())` (size table + density inline), no longer relying on a fixed square.
- [x] FIXED Hover treatment added via `.hover(|s| s.bg(hover_fill).text_color(hover_text))` (surface-hover bg + default text color).
- [ ] JsEl gap: no `focus`/`outline` primitive, so the contract §6 accent focus ring is not rendered. `.focusable()` is still set for the runtime's own focus handling. (Approximated — noted.)
- [x] FIXED Idle background is transparent (JsEl `button` default); the density `comfortable` branch is now expressed through `padding_inline_rem()`.
- [x] FIXED Disabled now sets `.cursor_default()` (and drops to opacity 0.4); `cursor_pointer()` only applies in the enabled branch.
- accepted: click/keyboard toggle lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Directions (4, interactive toggle + state label), Disabled (left/right), Sizes (xs–xl via snippet), Densities (compact/default/comfortable via snippet).
- GPUI covers: Directions (4, interactive), Disabled (left/right), Sizes + Densities via `specimen_layout`. — parity OK.
- Jetstream covers: per-direction collapsed-vs-expanded pairs (4), With-label (host-side text label beside toggle, both states), Disabled (2), Sizes (xs–xl), Densities (compact/default/comfortable). — parity OK. Interactive toggle wiring still lives in the preview event loop (not the pure-render specimen).

## Notes

- `consv=fixed`: the contract density table now correctly reads `padding-inline` (was claiming full `padding`); this was a density-orthogonality contract bug, not a Svelte bug.
- Pass 41: both targets now resolve the icon+padding intrinsic sizing. Added additive `CollapseToggleSpec` helpers — `effective_size()`, `padding_rem()` (size table), `padding_inline_rem()` (density inline, height-preserving), `effective_icon_size()` — and `icon_size_token()` now scales with the effective size (was a fixed `SIZE_ICON_SM`). GPUI: content-sized inline-flex, scaled chevron, hover-only, no active. Jetstream: same geometry + hover + disabled cursor; Jetstream probe tests cover chevron direction-by-state, per-direction names, density widens inline padding only (height unchanged), size scales vertical padding, disabled still renders chevron. Lone open todo is the Jetstream focus ring (no JsEl `outline` primitive).
