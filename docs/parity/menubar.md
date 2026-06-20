<!-- parity consv=fixed gpui=3 jetstream=6 specimen=gap -->
# Parity: Menubar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/menubar.md`
- Svelte (authoritative): `packages/svelte/components/src/Menubar.svelte`
- GPUI: `packages/gpui/components/src/primitives/menubar.rs`
- Jetstream: `packages/jetstream/components/src/menubar.rs`
- Spec: `packages/contracts/components/src/menubar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MenubarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/menubar.rs` · jetstream `packages/jetstream/preview/src/specimens/menubar.rs`

## Contract ↔ Svelte

Props/roles/keyboard align; the size table is where contract and Svelte diverge.

- [x] FIXED — **Size table mismatch**: contract §8 specified concrete per-size trigger/item `min-height` and `padding`. Svelte's size variants (`Menubar.svelte:412-452`) only override `font-size` and leave `min-height`/`padding` pinned to the `--poodle-size-control-height` / `--poodle-space-control-x` tokens for every size. Reconciled contract §8 size table to font-size-only stepping (trigger xs `0.6875`, lg `0.875`, xl `0.9375`; item xs `0.75`, lg `0.9375`, xl `1`; sm/md inherit base); min-height/padding rows now documented as pinned to control-height/control-x.
- [x] FIXED — Contract trigger `font-size: 0.75rem` / item `font-size: 0.875rem` now cite `--poodle-typography-label-size` / `--poodle-typography-body-size` tokens (`Menubar.svelte:338,382`); trigger `font-weight` cites `--poodle-typography-label-weight`; trigger base min-height/padding and item base min-height/padding cite the control tokens.
- All props (`value`, `defaultValue`, `items`, `size`, `sizeRole`=`chrome`, `density`, `ariaLabel`), callbacks, roles (`menubar`/`menuitem`/`menuitemcheckbox`/`menuitemradio`/`separator`), `aria-haspopup`/`aria-expanded`/`aria-controls`, and the full keyboard map match. `sizeRole` default `chrome` matches contract (unlike Menu).

## GPUI gap (vs Svelte + contract)

GPUI builds the list chrome + roving trigger strip well (token-resolved, color-mix via opacity, hover/active/disabled/focus), then delegates the dropdown to the `Menu` component.

- [ ] Inherits Menu's hardcoded HSLA shadow literals via the delegated dropdown (`menu.rs:201,207`) — see menu.md; the menubar overlay shadow is therefore also non-token.
- [ ] Hardcoded px: trigger-row `gap(px(rem_to_px(0.125)))` and `p(px(rem_to_px(0.1875)))` (`menubar.rs:144-145`) — contract list gap `0.125rem` / padding `0.1875rem`; these are correct values but inlined as rem literals rather than resolved from `space.*` tokens. Add tokens or justify.
- [ ] Overlay anchors below the whole wrapper (`menubar.rs:199-220`) via a flex column, not absolutely positioned under the specific trigger group; contract overlay is `position: absolute; left: 0` under its group with `0.25rem` gap. Acceptable as a GPUI layout delta but note: dropdown does not align to the active trigger's left edge.
- accepted: no ARIA (roles/aria-* not emitted); roving-focus keyboard nav across triggers is render-driven by `current_value` rather than internal focus state (interaction lives in preview event loop).

## Jetstream gap (vs Svelte + contract)

Renders only the trigger strip — no list chrome, no dropdown overlay at all.

- [ ] **No list chrome** — root is a bare `flex_row` (`menubar.rs:25`); contract §8 List requires border (`border-subtle 72%`), `radius-surface`, panel-96% bg, `0.1875rem` padding, `0.125rem` gap. None applied except gap.
- [ ] **No overlay/dropdown** — `js_menubar` never renders `current_menu().items`; clicking a trigger has no submenu. The entire submenu half of the contract (items, separators, meta, roles) is unimplemented.
- [ ] Hardcoded px: `pad_y = rem_to_px(0.25)` (`menubar.rs:15`) magic; trigger weight uses raw `600`/`400` (`:34`) instead of a label-weight token.
- [ ] No `radius.control` on triggers, no focus-ring treatment.
- [ ] `current_value` highlights a trigger but there is no way to show the open menu — open state is visually indicated on the trigger only.
- [ ] No size-table dimensions (min-height/padding only via `control_space_x_rem`; trigger min-height not set).
- accepted: interaction (trigger click, item nav) lives in preview event loop.

## Specimen parity

- Svelte covers: Application menu bar with File/Edit/View, each with shortcut items + separators; interactive open/switch (`MenubarSpecimen.svelte`).
- GPUI covers: File/Edit/View with shortcuts + separators, tracked open value + selected action (`menubar.rs`). — covers trigger strip + dropdown; matches contract specimen.
- Jetstream covers: Default (File open), Edit open, With disabled entry (`menubar.rs:46-57`). — triggers render with open/disabled states, but **no dropdown items render** (impl has no overlay), so the submenu specimen content is invisible. `specimen=gap`.

## Notes

- `consv=fixed`: contract §8 size table reconciled to Svelte (font-size-only stepping; min-height/padding pinned to control tokens); base trigger/item font/weight/dimension rows now cite tokens instead of resolved literals.
- Jetstream is the weakest target: trigger strip only, no list chrome, no dropdown — roughly half the contract unimplemented. GPUI is close to parity (chrome + delegated Menu dropdown); its main debt is the inherited shadow literal and overlay-anchoring delta.
