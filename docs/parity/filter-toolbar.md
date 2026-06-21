<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap | pass: toggle dims/radius + actions gap + sticky shadow + density gap/padding all token-resolved on GPUI, header focus ring added; Jetstream gained collapse chevron, density-aware padding/gaps, summary flex-grow, token summary font; jetstream probe tests added -->
# Parity: FilterToolbar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/filter-toolbar.md`
- Svelte (authoritative): `packages/svelte/components/src/FilterToolbar.svelte`
- GPUI: `packages/gpui/components/src/composites/filter_toolbar.rs`
- Jetstream: `packages/jetstream/components/src/filter_toolbar.rs`
- Spec: `packages/contracts/components/src/filter_toolbar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FilterToolbarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/filter_toolbar_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/filter_toolbar.rs`

## Contract ↔ Svelte

Divergences in the density token table — Svelte authoritative, contract is wrong. Reconciled.

- [x] FIXED **Density `default` root gap**: contract §8 density table said `var(--poodle-space-stack-sm)`; Svelte `[data-density="default"]` override (`FilterToolbar.svelte:273`) uses `var(--poodle-space-inline-sm)`. Contract density table now `space-inline-sm`. (Base un-scoped Root rule keeps `space-stack-sm`, matching Svelte base line 168.)
- [x] FIXED **Density `compact` root gap**: contract said `0.375rem`; Svelte uses `0.25rem` (`FilterToolbar.svelte:264`). Contract compact gap → `0.25rem`.
- [x] FIXED **Density padding-block**: default density padding now `0.75rem 1rem` (block inline) matching Svelte literals (lines 274-275); compact `0.5rem 0.75rem`, comfortable `1rem 1.25rem` already matched. Noted in §8 as the documented panel-internal-padding compositional exception.
- **`data-collapsed` value**: contract §8 lists `data-collapsed`; Svelte sets it to `collapsible && collapsed` (line 62). Consistent, no fix, noted for GPUI/Jetstream.
- Anatomy: expanded-collapsible header renders as `<button aria-expanded>` (Svelte lines 101-127). Contract §2/§6 already document this. OK.
- `summary` flex: contract §8 summary `flex: 1`; Svelte matches (line 206). OK.

## GPUI gap (vs Svelte + contract)

- [x] DONE Toggle dims/radius token-resolved — hit-area = `toggle_size_token` (`size.icon.md`), radius = `toggle_radius_token` (`radius.control`). No raw px.
- [x] DONE Actions gap token-resolved — `actions_gap_token` (`space.inline.xs`, 0.25rem).
- [x] DONE Sticky shadow token-resolved — `elevation_surface_shadow()` (`elevation.surface`); hand-built hsla/offset/blur removed.
- [x] DONE Header focus ring — when collapsible the header row is `.id().focusable().focus(border(focus_ring_width) + border_color(focus_ring))` using `focus_ring_width_token`/`focus_ring_color_token`. (No ARIA channel; visible focus-visible outline reproduced.)
- [x] DONE Density-aware root gap/padding — `density_gap_rem`/`gap_token` for gap, `padding_block_rem`/`padding_inline_rem` for padding (contract §8 density table).
- [x] DONE Controls grid gap by density — `density_controls_gap_rem`/`controls_gap_token` (compact 0.25rem, comfortable `space.inline.md`).
- [x] DONE Header semantics — collapse toggle + summary + actions render in the header; `on_toggle` is still caller-wired (preview-loop) per contract §5. Toggle interactivity is preview-loop, not the component.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`aria-expanded` not emitted.
- note (specimen, preview crate — not touched here): collapse toggle still not interactive in preview (no `on_toggle` wired) and Densities group absent; that's a preview-loop / specimen follow-up.

## Jetstream gap (vs Svelte + contract)

- [x] DONE Collapse chevron rendered — when `spec.collapsible`, header draws a token-sized toggle (`size.icon.md` hit-area, `radius.control`) with `chevron-down` (expanded) / `chevron-right` (collapsed) by `is_grid_visible()`.
- [x] DONE Summary font-size — resolves from `summary_font_size_rem` (contract §8 size table) via the spec, not inline match literals.
- [x] DONE Density applied — separate `padding_inline_rem`/`padding_block_rem` (px/py, not a single `.p`), density-aware root gap (`density_gap_rem`/`gap_token`) and controls gap.
- [x] DONE Controls grid gap by density — `density_controls_gap_rem`/`controls_gap_token`.
- [x] DONE Summary `.grow()` so the actions slot anchors right (Svelte summary `flex:1` / actions `margin-left:auto`); empty-summary case reserves a grow spacer.
- note: focus ring / button semantics — JsEl has no focus-ring channel on a plain div here; the header toggle interaction (collapse click) belongs in the preview event loop, not the component (contract §5). Chevron + region semantics present; ring is a preview/JsEl-runtime gap.
- accepted: ARIA channel absent; interaction (collapse click) belongs in preview event loop, not the component.

## Specimen parity

- Svelte covers: Responsive grid, Collapsible with actions (expanded, bindable), Explicit collapsed state, With secondary slot, Sizes, Densities (`FilterToolbarSpecimen.svelte`).
- GPUI covers: Responsive grid, Sizes, Collapsible+actions (expanded), Explicit collapsed, With secondary — missing: **Densities** group; collapse toggle is not interactive (no `on_toggle` wired, so collapsed↔expanded cannot be exercised).
- Jetstream covers: Expanded with summary, Collapsed with actions, With secondary, Empty — missing: **Sizes** group, **Densities** group; uses stand-in `chip` labels not real TextInput/Select (acceptable: primitives noted as dependency), hardcoded `text_size(11.0)` group labels + `text_size(11.0)` Refresh action (specimen literal).

## Notes

- Spec default `collapsed = true` (`filter_toolbar.rs:48`) intentionally differs from Svelte prop default `collapsed = false` (Svelte line 30) — the Rust spec comment says it defaults collapsed to match Svelte, but Svelte's default is `false`. Minor: spec default should be `false` to truly match. Flag low-priority.
- GPUI/Jetstream both lack the responsive `@media (max-width:640px)` 1-column collapse — accepted Tier-3 implementation freedom (grid mechanics internal), not a todo.
- `columns` prop is legacy/unused for the grid in all three (auto-fit), per contract §3 — no action.
