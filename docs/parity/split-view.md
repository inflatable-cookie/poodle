<!-- parity consv=fixed gpui=6 jetstream=6 specimen=gap -->
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

- [ ] **Hardcoded px literals throughout the divider/toggles** — `size(px(16.0))`, `rounded(px(8.0))`, divider `w(px(8.0))`, line `w(px(1.0))`, `gap(px(4.0))` (`split_view.rs:272-273, 369, 378, 391`). Contract divider is `0.5rem`; toggle sizing should derive from the `CollapseToggle` primitive. Resolve from tokens / rem helpers.
- [ ] **Does not compose `CollapseToggle` primitive** — hand-builds circular chevron buttons (`split_view.rs:300-358`) instead of delegating to `CollapseToggle` per contract §2/§7. Visual + behavioral drift from the primitive.
- [ ] **Collapse callbacks can only collapse, never expand** — both toggles fire `handler(true, ...)` unconditionally (`split_view.rs:321, 350`); contract toggles flip state (Expand when already collapsed). Pass `!is_collapsed`.
- [ ] **No keyboard resize / ResizeHandle composition** — divider is a bare hover-highlighted `div` with `cursor_col_resize` (`split_view.rs:398-417`); it does not embed the `ResizeHandle` primitive, so arrow-key resize (contract §6) and separator semantics are absent.
- [ ] **No rail-collapse / drag-to-collapse** — `*CollapsedSize`, `collapse*BelowSize`, 2%/98% thresholds, preserved ratio are not implemented (spec carries the fields but the renderer ignores them). Contract §8 Rail-Collapse / Drag-To-Collapse tables.
- [ ] **No fixed-collapsed (railed) content mounting** — collapsed panes are dropped entirely (`if !is_primary_collapsed`, `split_view.rs:195,424`); contract railed state keeps content mounted at a pinned size.
- accepted: no ARIA (gpui has no accessibility API); drag physics platform-owned.

## Jetstream gap (vs Svelte + contract)

Jetstream composes the real `js_resize_handle` and maps orientation correctly (`Horizontal` split → `Vertical` handle line, `split_view.rs:58-61`). Panes use ratio via `flex_grow`, fixed size, and collapse-to-zero.

- [ ] **No collapse toggles** — `show_collapse_primary`/`secondary` ignored; no `CollapseToggle` composition (contract §2 Toggles/CollapseToggle). Cannot collapse/expand via UI.
- [ ] **No min-size constraints** — `min_primary_size`/`min_secondary_size` unused (panes hardcode `min_w(0.0)`/`min_h(0.0)`, `split_view.rs:21-25`); contract applies min-width/height inline when set.
- [ ] **Vertical orientation uses `w()` for fixed/collapsed sizing** — `spec.primary_size` always applied as `.w(px)` and collapse as `.w(0.0)` (`split_view.rs:19,23,38,41`) regardless of orientation; for a vertical split this should be height. Axis-incorrect for fixed/collapsed panes in vertical mode.
- [ ] **No rail-collapse / drag-to-collapse / keyboard resize** — `*CollapsedSize`, `collapse*BelowSize`, thresholds, ratio streaming all absent (interaction lives in preview event loop, but none is wired and the handle emits no callbacks).
- [ ] **Collapsed pane content dropped** — collapse renders a zero-width div; railed (content-mounted) state not supported.
- [ ] **Depends on the inverted `ResizeHandle`** — `js_resize_handle` itself renders orientation inverted (see `resize-handle.md`); here the split_view's compensating mapping happens to produce a correct line, but the two bugs are coupled — fixing one without the other will flip the divider. Note the coupling.
- accepted: interaction (drag, keyboard, collapse) lives in preview `main.rs` event loop; no ARIA channel.

## Specimen parity

- Svelte covers: Basic horizontal, Basic vertical, Horizontal w/ collapse toggles, Vertical w/ collapse toggles, **Nested splits (IDE)**, **Disabled**. Six groups.
- GPUI covers: all six matching groups (Basic h/v, h/v collapse toggles, Nested, Disabled) — full group parity, though the toggles are hand-built and collapse is one-way.
- Jetstream covers: Horizontal split, Vertical split, **Primary-only**. — missing: **collapse-toggle groups** (h + v), **Nested splits**, **Disabled**. No collapse story at all.

## Notes

- The biggest cross-target theme: GPUI and Jetstream both treat collapse as render-state without the rail/drag-collapse lifecycle and (GPUI) one-way toggles. Reaching Tier-1 parity requires composing the real `CollapseToggle` + `ResizeHandle` primitives and porting the rail/threshold logic from Svelte (`SplitView.svelte:179-260`).
- `consv=fixed`: Toggles-cluster contract drift resolved (gap → 0.125rem, surface styling added, phantom pointer-events + z-index dropped, justify-content added). Behavioral contract↔Svelte parity was otherwise already clean.
- GPUI specimen `region`/`frame` helpers hand-compute HSLA swatches and px (`split_view_specimen.rs:16-51`) — specimen stand-ins for `<Region>`, not component code. Jetstream specimen `group()` hardcodes `text_size(11.0)` — specimen chrome.
