<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok -->
<!-- pass 41: both targets compose real ResizeHandle + CollapseToggle; GPUI toggles flip state (two-way) and divider derives from rem helpers; Jetstream adds collapse toggles, min-size, and axis-correct fixed/collapsed sizing for vertical. Remaining open = rail-collapse/drag-collapse lifecycle (spec lacks *CollapsedSize / collapse*BelowSize). Probe tests added; specimens gain collapse-toggle groups. -->

# Parity: SplitView

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/split-view.md`
- Svelte (authoritative): `packages/svelte/components/src/SplitView.svelte`
- GPUI: `packages/gpui/components/src/composites/split_view.rs`
- Jetstream: `packages/jetstream/components/src/split_view.rs`
- Spec: `packages/contracts/components/src/split_view.rs`
- Composed primitives: `ResizeHandle`, `CollapseToggle`
- Specimens: svelte `packages/svelte/preview/src/specimens/SplitViewSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/split_view_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/split_view.rs`

## Contract ↔ Svelte

Svelte implements the full prop surface (orientation, ratio/defaultRatio, min/fixed sizes, collapse states, rail-collapse `*CollapsedSize`, drag-collapse `collapse*BelowSize`, toggles, size/density), flex computation table, drag-to-collapse + rail thresholds, and composes `ResizeHandle` + `CollapseToggle`. Divergences are contract-side staleness in the Toggles styling.

- [x] FIXED **Toggles `gap`** — contract §8 said `gap: 0.25rem`; Svelte uses `gap: 0.125rem` (`SplitView.svelte:388`). Contract → 0.125rem.
- [x] FIXED **Toggles surface styling** — added the toggle cluster `padding: 0.125rem`, `border-radius: var(--poodle-radius-pill)`, `background` color-mix (panel 92% / elevated), and `box-shadow` ring (`:389-396`) to contract §8.
- [x] FIXED **`pointer-events` rule** — Svelte's `.toggles` has **no** pointer-events rule (`:383-397`); dropped the phantom rows from contract.
- [x] FIXED **`z-index: 1`** — Svelte omits it; dropped from contract. Added `justify-content: center` to match Svelte (`:387`).
- Flex table, drag thresholds (2%/98%), rail hysteresis (+8px), clamping [0.05, 0.95], toggle visibility rules, toggle direction by orientation — all match. No behavioral divergence.

## GPUI gap (vs Svelte + contract)

GPUI renders panes with ratio/fixed/min allocation, collapse hiding, a divider with hover, and collapse-toggle buttons. Layout logic is largely faithful.

- [x] FIXED **Hardcoded px in divider/toggles** — divider now sized `px(rem_to_px(0.5))` on the split axis; toggle cluster gap/padding `rem_to_px(0.125)`; the `px(16.0)`/`rounded(px(8.0))`/`px(1.0)` line literals are gone (handle + toggles own their sizing).
- [x] FIXED **Does not compose `CollapseToggle`** — now `CollapseToggle::from_spec` with direction by orientation (Left/Right or Up/Down), collapsed state, disabled, and dynamic aria-label ("Collapse/Expand primary/secondary").
- [x] FIXED **Collapse callbacks one-way** — toggles now fire `handler(!is_collapsed, …)` so they flip state.
- [x] FIXED **No ResizeHandle composition** — divider now embeds `ResizeHandle::from_spec` (orientation inverse of split axis, `aria_value_now=ratio`, disabled forwarded), giving separator semantics + the keyboard-resizable affordance.
- [ ] **No rail-collapse / drag-to-collapse** — `*CollapsedSize`, `collapse*BelowSize`, 2%/98% thresholds, preserved ratio are not implemented. **Spec gap**: `SplitViewSpec` has no `primary_collapsed_size` / `secondary_collapsed_size` / `collapse_*_below_size` fields. Out of scope this pass (additive spec expansion + drag lifecycle); the drag/threshold logic lives in the preview event loop.
- [ ] **No fixed-collapsed (railed) content mounting** — collapsed panes are still dropped; coupled to the rail-collapse spec gap above (needs `*CollapsedSize` to know the pinned size).
- accepted: no ARIA exposure (gpui has no accessibility API); drag physics platform-owned. Two-way toggle + keyboard-resizable separator now provided by the composed primitives.

## Jetstream gap (vs Svelte + contract)

Jetstream composes the real `js_resize_handle` and maps orientation correctly (`Horizontal` split → `Vertical` handle line, `split_view.rs:58-61`). Panes use ratio via `flex_grow`, fixed size, and collapse-to-zero.

- [x] FIXED **No collapse toggles** — now composes `js_collapse_toggle` per `show_collapse_*`, with contract visibility rules (primary hidden when secondary collapsed and vice-versa) and direction by orientation (Left/Right or Up/Down). Cluster overlays the handle inline (immediate-mode has no absolute centering — approximate, noted).
- [x] FIXED **No min-size constraints** — `min_primary_size` / `min_secondary_size` now applied inline (`min_w` for horizontal, `min_h` for vertical) when set and the pane is not collapsed.
- [x] FIXED **Vertical orientation axis** — fixed and collapsed sizing now applied on the correct axis: `w()` for horizontal, `h()` for vertical (probe test `split_view_vertical_fixed_primary_uses_height_axis`).
- [ ] **No rail-collapse / drag-to-collapse / keyboard resize wiring** — `*CollapsedSize`, `collapse*BelowSize`, thresholds, ratio streaming still absent. **Spec gap** (same as GPUI): fields not on `SplitViewSpec`. Interaction lives in the preview event loop; the handle + toggles render but emit no callbacks. Out of scope this pass.
- [ ] **Collapsed pane content dropped** — legacy collapse hides the pane (0-size on the split axis), matching the contract's non-railed collapse; railed content-mounting needs the rail-collapse spec fields above.
- note: **coupled to inverted `ResizeHandle`** — `js_resize_handle` renders orientation inverted (see `resize-handle.md`); the split_view's compensating mapping (Horizontal split → `Orientation::Vertical` handle) still produces the correct line. Fixing the handle bug requires flipping this mapping in lock-step.
- preview-loop: drag, keyboard, collapse interaction lives in the preview event loop; no ARIA channel.

## Specimen parity

- Svelte covers: Basic horizontal, Basic vertical, Horizontal w/ collapse toggles, Vertical w/ collapse toggles, **Nested splits (IDE)**, **Disabled**. Six groups.
- GPUI covers: all six matching groups (Basic h/v, h/v collapse toggles, Nested, Disabled) — full group parity, though the toggles are hand-built and collapse is one-way.
- Jetstream covers: Horizontal split, Vertical split, **Horizontal + Vertical collapse-toggle groups**, Primary-only. Collapse toggles now render via the composed `js_collapse_toggle`. Still missing vs GPUI: **Nested splits**, **Disabled** (lower priority — no spec gap).

## Notes

- The biggest cross-target theme: GPUI and Jetstream both treat collapse as render-state without the rail/drag-collapse lifecycle and (GPUI) one-way toggles. Reaching Tier-1 parity requires composing the real `CollapseToggle` + `ResizeHandle` primitives and porting the rail/threshold logic from Svelte (`SplitView.svelte:179-260`).
- `consv=fixed`: Toggles-cluster contract drift resolved (gap → 0.125rem, surface styling added, phantom pointer-events + z-index dropped, justify-content added). Behavioral contract↔Svelte parity was otherwise already clean.
- GPUI specimen `region`/`frame` helpers hand-compute HSLA swatches and px (`split_view_specimen.rs:16-51`) — specimen stand-ins for `<Region>`, not component code. Jetstream specimen `group()` hardcodes `text_size(11.0)` — specimen chrome.
