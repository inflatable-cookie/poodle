<!-- parity consv=ok gpui=2 jetstream=2 specimen=gap -->
<!-- pass 49: built out both targets. Additive DockRegionSpec: DockSizing (Flexible/Static),
     DockCollapsedPosture (IconStrip/Hidden), DockEmphasis (Standard/Quiet/Strong), can_accept_panel
     (render-only proxy for Svelte's canAcceptPanel callback). Both render collapsedPosture,
     emphasis (quiet transparent / strong accent-32% border), canAcceptPanel drop overlay, static
     mode, compact/icon-only tabs. GPUI dropped px(4/36/32) — strip dims from density tokens.
     Jetstream tab_gap/border/active-tint → tokens. 6 probe tests; specs 61, jet 184, gpui clean.
     Tab activation/collapse/DnD = preview-loop; JsEl no dashed-border (solid+8% approx). -->
<!-- specimen note: GPUI specimen done (dock_split.rs) — added emphasis (quiet/standard/strong),
     collapsed hidden posture, compact top-edge icon-strip, canAcceptPanel drop affordance, and
     cross-region two-dock DnD groups; real DockRegion only, no fakes; gpui/preview builds 0 errors.
     Jetstream pending engine recovery. specimen=gap held — Jetstream half unverifiable while
     engine is build-blocked. -->
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
- GPUI covers: static horizontal/vertical, flexible expanded, flexible collapsed icon-strip, collapse toggle (external button), right edge, bottom edge, **emphasis (quiet/standard/strong)**, **collapsed hidden posture**, **compact tabs (top-edge icon-strip)**, **canAcceptPanel drop affordance**, **cross-region drag-and-drop (two side-by-side regions, both `can_accept_panel`)**, SplitView horizontal/vertical. **GPUI specimen done** — every contract §13 specimen state plus the §4 emphasis / collapsed-posture / drop-affordance variants, all driven by the real `DockRegion` (no fakes). Jetstream pending engine recovery. — remaining (accepted GPUI deltas, not specimen gaps): CollapseToggle is an external button (no primitive), and auto-compact is static-only (no ResizeObserver in GPUI).
- Jetstream covers: left dock with tabs, empty dock, active selection, bottom dock, top dock, collapsed state. — missing: **expanded with body**, **interactive collapse toggle**, **right edge**, **cross-region DnD**, **static mode**, **size variants**, **density variants**, **icon-only compact**, **closable/reorderable tabs**, **collapsed-posture variants**, **emphasis variants**.

## Notes

- `consv=ok`: contract and Svelte aligned.
- GPUI specimen now exercises collapsed-posture (hidden + icon-strip), emphasis (quiet/standard/strong), and the cross-region drop affordance (`can_accept_panel` → drop-zone overlay) — the component already supports all three (`dock_region.rs`); they were just unexercised by the specimen. Jetstream is further behind (no static mode, no compact tabs) and is build-blocked.
- GPUI specimen file is `dock_split.rs` (also hosts SplitView demos), not `dock_region.rs`.
