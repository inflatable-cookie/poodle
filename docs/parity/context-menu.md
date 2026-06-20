<!-- parity consv=ok gpui=2 jetstream=5 specimen=gap -->
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

All contract props (`items`, `open`, `defaultOpen`, `anchorPoint`, `size`, `sizeRole`, `density`, `ariaLabel`) and callbacks (`onOpenChange`, `onAction`) are present in Svelte with matching types/defaults. Root carries `role="button"`, `tabindex="0"`, `aria-haspopup="menu"`, `data-size`, `data-density`; overlay handled by MenuSurface. No divergence.

- `items` is required in contract §3 but Svelte defaults to `[]` (safe fallback). Cosmetic only — no fix needed.

## GPUI gap (vs Svelte + contract)

Delegates to `Menu` for item rendering; applies absolute positioning at anchor.

- [ ] No viewport clamping. Svelte clamps overlay to 8px edge padding (`ContextMenu.svelte:74-92`); GPUI positions raw at anchor with no collision adjustment. Add edge-clamp on render.
- [ ] No focus restoration to invocation target after close. Svelte tracks trigger for focus return; GPUI ContextMenu spec doesn't. Track + restore.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` forwarded via MenuSpec but not emitted on overlay role.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded `min_width = rem_to_px(10.0)` at `context_menu.rs:23` — contract specifies 14rem min-width; resolve from `ContextMenuSpec` min-width token, not raw 10.0.
- [ ] Hardcoded `item_px = rem_to_px(0.75)` at `context_menu.rs:24` — contract item padding-inline is 0.5rem; resolve from token, not raw 0.75.
- [ ] No size scaling — xs/sm/md/lg/xl from contract §3 not applied to item dimensions/font.
- [ ] No density support — compact/default/comfortable not applied to item gaps.
- [ ] No `shortcutLabel` (meta column) rendering — contract §2 anatomy requires it; `js_context_menu` never reads item shortcut.
- accepted: no ARIA channel (Jetstream emits no runtime roles).
- accepted: separator `kind` + open/close interaction live in preview `main.rs` event loop.

## Specimen parity

- Svelte covers: main right-click target + action tracking, full item set (Cut/Copy/Paste, separators, Select all, disabled Delete with shortcut labels), size variants, density variants.
- GPUI covers: same right-click target, same item set, size variants, density variants — full parity.
- Jetstream covers: "Default" + "Extended items" groups only. — missing: **size variants**, **density variants**, **separators**, **disabled state**, **shortcut labels**.

## Notes

- `consv=ok`: contract and Svelte are aligned for this component.
- Biggest gap is the Jetstream min-width/padding token violations (10rem vs 14rem, 0.75rem vs 0.5rem) plus missing size/density/shortcut surface.
