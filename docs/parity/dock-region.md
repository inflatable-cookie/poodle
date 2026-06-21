<!-- parity consv=ok gpui=2 jetstream=2 specimen=gap -->
<!-- pass 49: built out both targets. Additive DockRegionSpec: DockSizing (Flexible/Static),
     DockCollapsedPosture (IconStrip/Hidden), DockEmphasis (Standard/Quiet/Strong), can_accept_panel
     (render-only proxy for Svelte's canAcceptPanel callback). Both render collapsedPosture,
     emphasis (quiet transparent / strong accent-32% border), canAcceptPanel drop overlay, static
     mode, compact/icon-only tabs. GPUI dropped px(4/36/32) — strip dims from density tokens.
     Jetstream tab_gap/border/active-tint → tokens. 6 probe tests; specs 61, jet 184, gpui clean.
     Tab activation/collapse/DnD = preview-loop; JsEl no dashed-border (solid+8% approx). -->
# Parity: DockRegion

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/dock-region.md`
- Svelte (authoritative): `packages/svelte/components/src/DockRegion.svelte`
- GPUI: `packages/gpui/components/src/composites/dock_region.rs`
- Jetstream: `packages/jetstream/components/src/dock_region.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DockRegionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/dock_split.rs` · jetstream `packages/jetstream/preview/src/specimens/dock_region.rs`

## Contract ↔ Svelte

All contract props, callbacks, and snippets present in Svelte with matching types/defaults: `edge`, `sizing`, `collapsible`, `collapsed`, `collapsedPosture`, `emphasis`, `items`, `value`, `size`, `sizeRole` (`"chrome"`), `density`, `ariaLabel`, `canAcceptPanel`; callbacks `onValueChange`/`onCollapsedChange`/`onClose`/`onReorder`/`onPanelDrop`; snippets `panel`, `children`. Anatomy complete (Region/Strip/Tabs/CollapseToggle/Body/Stack/Drop Zone). No divergence.

## GPUI gap (vs Svelte + contract)

Spec adds `tabs_placement` (not in Svelte); missing several contract props.

- [ ] No `collapsedPosture` (icon-strip vs hidden) — contract §3 requires both modes; collapsed renders a flat 4px gutter only. Add posture to spec + render.
- [ ] No `emphasis` (standard/quiet/strong) — absent from spec; add and apply to region treatment.
- [ ] No `canAcceptPanel` drop-validation callback — cross-region DnD validation missing from spec.
- [ ] No click-to-expand from collapsed — contract: clicking a tab while collapsed fires both `onValueChange` and `onCollapsedChange`.
- [ ] No compact-mode detection — Svelte uses a strip ResizeObserver to switch to icon-only tabs; GPUI tabs are fixed.
- [ ] Hardcoded collapsed-gutter floats `px(4.0)` at `dock_region.rs:147` and `:149` — resolve from a size token.
- [ ] Hardcoded strip dimensions `px(36.0)` (`dock_region.rs:175`, vertical strip width) and `px(32.0)` (`:183`, horizontal strip height) — resolve from control-height/strip tokens.
- accepted: no ARIA (gpui has no accessibility API) — no section role / aria-label / roving tabindex; Svelte's Tabs-primitive ARIA not mirrored.

## Jetstream gap (vs Svelte + contract)

Simplified horizontal-strip-only impl; many contract behaviors absent.

- [ ] No `collapsedPosture` — only binary collapsed; add icon-strip + hidden modes.
- [ ] No `emphasis` variants.
- [ ] No `canAcceptPanel` / cross-region drag-and-drop.
- [ ] No click-to-expand from collapsed.
- [ ] No static-mode stacking (`sizing="static"` Stack/Stack Item anatomy).
- [ ] No compact (icon-only) tab mode; no tab icons, closable, or reorderable tabs.
- [ ] No `aria_label` rendered on the region root (Jetstream emits no role either — note once).
- [ ] Hardcoded `tab gap = rem_to_px(0.25)` at `dock_region.rs:18` — resolve from a density/gap token.
- [ ] Hardcoded active-tab tint `tint(accent, 0.18)` at `dock_region.rs:25` — resolve the active fill from a token, not a raw 0.18 alpha.
- [ ] Hardcoded `border(1.0)` at `dock_region.rs:31` — resolve border width from a token.
- accepted: interaction (collapse toggle, tab switch, DnD) lives in preview `main.rs` event loop.

## Specimen parity

- Svelte covers: flexible expanded (icon+label tabs, body, closable/reorderable), flexible collapsed icon-strip (vertical icon tabs), interactive collapse toggle with click-to-expand, bottom-edge dock, cross-region DnD with validation, static horizontal, static vertical, size variants, density variants.
- GPUI covers: static horizontal/vertical, flexible expanded, flexible collapsed icon-strip, collapse toggle (external button), right edge, bottom edge, SplitView horizontal/vertical. — missing: **cross-region DnD**, **size variants**, **density variants**, **icon-only compact mode**, **CollapseToggle primitive** (toggle is an external button), **closable/reorderable tabs**.
- Jetstream covers: left dock with tabs, empty dock, active selection, bottom dock, top dock, collapsed state. — missing: **expanded with body**, **interactive collapse toggle**, **right edge**, **cross-region DnD**, **static mode**, **size variants**, **density variants**, **icon-only compact**, **closable/reorderable tabs**, **collapsed-posture variants**, **emphasis variants**.

## Notes

- `consv=ok`: contract and Svelte aligned.
- Both Rust targets share the same structural deficit: collapsed-posture, emphasis, and cross-region DnD are unimplemented. Jetstream is further behind (no static mode, no compact tabs).
- GPUI specimen file is `dock_split.rs` (also hosts SplitView demos), not `dock_region.rs`.
