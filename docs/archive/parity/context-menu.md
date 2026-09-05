<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- specimen=ok: Jetstream specimen rebuilt to mirror GPUI + contract §8 (real js_context_menu) — editing menu (shortcuts/separators/disabled), danger + checkbox/radio surface, size matrix xs–xl, density matrix. Open/close + anchor positioning preview-loop bound. Both previews build clean. -->
<!-- pass 41: Jetstream rebuilt to match Menu item surface. min-width 10→14rem,
     item padding 0.75→0.5rem (size-table driven), per-size item metrics
     (min-height/padding/font xs–xl), density-aware item gap, shortcut (meta)
     column 0.6875rem, separators (border-subtle@72% 1px), checkbox/radio leading
     check indicator, disabled opacity (state-opacity-disabled), destructive
     (status.danger) label color, accent@16% hover, item radius control-0.125rem,
     overlay padding 0.25rem. Probe tests: items+shortcuts, separators, danger,
     size scaling. GPUI: surface complete via Menu delegation; its 2 remaining
     todos (viewport clamping, focus restoration) are runtime/preview-loop, not
     static render gaps — accepted. Specimen still gap (preview-loop wiring). -->
# Parity: ContextMenu

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/context-menu.md`
- Svelte (authoritative): `packages/svelte/components/src/ContextMenu.svelte`
- GPUI: `packages/gpui/components/src/primitives/context_menu.rs`
- Jetstream: `packages/jetstream/components/src/context_menu.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ContextMenuSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/context_menu.rs` · jetstream `packages/jetstream/preview/src/specimens/context_menu.rs`

## Contract ↔ Svelte

All contract props (`items`, `open`, `defaultOpen`, `anchorPoint`, `size`, `sizeRole`, `density`, `trigger`, `ariaLabel`) and callbacks (`onOpenChange`, `onAction`) are present in Svelte with matching types/defaults. Root carries `role="button"`, `tabindex="0"`, `aria-haspopup="menu"`, `data-size`, `data-density` when `trigger` is true; `trigger={false}` omits that host and renders only MenuSurface. No divergence.

- `items` is required in contract §3 but Svelte defaults to `[]` (safe fallback). Cosmetic only — no fix needed.

## GPUI gap (vs Svelte + contract)

Delegates to `Menu` for item rendering (full item surface: separators, size/density,
shortcuts, disabled opacity, destructive danger color, check indicators); applies
absolute positioning at anchor. No open static-render gaps.

- preview-loop: viewport clamping. Svelte clamps overlay to 8px edge padding (`ContextMenu.svelte:74-92`); GPUI positions raw at anchor. Contract Tier 3 marks collision/clamping internals as implementation-free — runtime concern, not a render gap.
- preview-loop: focus restoration to invocation target after close — runtime focus behavior, not a static render property.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` forwarded via MenuSpec but not emitted on overlay role.

## Jetstream gap (vs Svelte + contract)

All static-render gaps closed (pass 41). `js_context_menu` now mirrors the `js_menu`
item surface with the context-menu §8 token values.

- accepted: no ARIA channel (Jetstream emits no runtime roles).
- preview-loop: open/close interaction + anchor positioning live in the preview event loop.

## Specimen parity

- Svelte covers: main right-click target + action tracking, full item set (Cut/Copy/Paste, separators, Select all, disabled Delete with shortcut labels), size variants, density variants.
- GPUI covers: same right-click target, same item set, size variants, density variants — full parity.
- Jetstream covers: "Default" + "Extended items" groups only. — missing: **size variants**, **density variants**, **separators**, **disabled state**, **shortcut labels**.

## Notes

- `consv=ok`: contract and Svelte are aligned for this component.
- Biggest gap is the Jetstream min-width/padding token violations (10rem vs 14rem, 0.75rem vs 0.5rem) plus missing size/density/shortcut surface.
