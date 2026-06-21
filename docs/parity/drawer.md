<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok | specimen backfill: both targets now cover all four edge anchors (left/right/top/bottom) with header+description+body + real Button footer actions — previews build clean -->
# Parity: Drawer

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/drawer.md`
- Svelte (authoritative): `packages/svelte/components/src/Drawer.svelte`
- GPUI: `packages/gpui/components/src/primitives/drawer.rs`
- Jetstream: `packages/jetstream/components/src/drawer.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DrawerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/drawer.rs` · jetstream `packages/jetstream/preview/src/specimens/drawer.rs`

## Contract ↔ Svelte

Svelte tracks the contract closely; one minor mechanical divergence worth noting.

- Surface uses `100dvh` / `min(24rem, 100dvh)` (`Drawer.svelte:265,303`); contract §7/§8 say `100vh`. dvh is the better mobile value — **Fix: update contract §8 to `100dvh`.**
- Border applied per-edge (`border-inline-end-width` etc., `Drawer.svelte:284-298`) with base `border: 0 solid` rather than the contract's single all-around `0.0625rem solid` border (§8). Visually equivalent (only the inner edge shows) but the mechanism differs. **Fix: document the per-edge border approach in contract §8.**
- Surface fill/border/shadow read from `--poodle-treatment-surface-elevated-*` tokens with the contract color-mix as fallback (`Drawer.svelte:268-281`). Contract §8 lists only the color-mix. Treatment-token indirection is the intended path. **Fix: note treatment-token layer in contract §8.**
- All props, anatomy parts (root/backdrop/surface/header/title/description/body/actions), states, ARIA (`role="dialog"`, `aria-modal`, `aria-label`, backdrop `aria-label`), and keyboard (Escape, Tab trap) match the contract. consv=ok.

## GPUI gap (vs Svelte + contract)

- accepted: shadow uses `theme_ext::elevation_dialog_shadow()` which is already
  token-resolved from the typed `ELEVATION_DIALOG` table (offset/blur/color come
  from the token, not raw HSLA) — the old `:186,192` literals no longer exist.
- [x] Backdrop color → now `resolve_color(theme, spec.backdrop_fill_token())`
  (`color.background.overlay`), no raw `hsla(…,0.5)`. FIXED (pass 41).
- [x] Surface sizing → edge-anchored: left/right `min_w(28rem)` + `h_full`;
  top/bottom now `w_full` + `h(24rem)` (`min(24rem,100vh)`). Backdrop/inline
  rows also switch to column direction for top/bottom edges. FIXED.
- [x] `size` handling → header title font now resolves from the contract §8
  size table via new `presentation::drawer_title_font_rem` (md `1rem`), replacing
  the flat `heading_size`. FIXED.
- [x] Header bottom-margin / actions row → header is now a grid block with
  `margin-bottom: space-stack-md`; new `with_actions(...)` slot renders the
  `.drawer__actions` footer (flex-end, wrap, `margin-top: space-stack-md`). FIXED.
- [x] Main-area fallback text → now uses `typography.body.size`, not a hardcoded
  `0.75rem`. FIXED.
- accepted: no ARIA (gpui has no accessibility API) — `role="dialog"`/`aria-modal` not emittable.
- accepted: edge slide motion curve differs (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] Panel sizing → edge-anchored: left/right `w(28rem)` + `h_full`; top/bottom
  `h(24rem)` + `w_full`. Dropped the per-`size` 18/24/32rem switch. FIXED (pass 41).
- [x] Panel inner gap now `space.stack.sm`; header internal gap now the contract
  `0.375rem`. FIXED.
- [x] Title font now resolves from the contract §8 size table via
  `presentation::drawer_title_font_rem` (md `1rem`); dropped the ad-hoc
  `+0.1875rem` offset. FIXED.
- [x] Close-`x` icon removed — not in contract anatomy / not in Svelte. Header is
  now title + description only. FIXED.
- [x] Header `margin-bottom: space-stack-md` + new `actions` slot
  (`js_drawer_with_actions`) rendering the `.drawer__actions` footer (flex-end,
  wrap, `margin-top: space-stack-md`). FIXED.
- accepted: no ARIA channel (`role="dialog"`).
- accepted: open/close + focus trap + scroll-lock interaction lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Right edge (trigger + title + description + actions Cancel/Save), Left edge (trigger + title, no actions). Interactive open/close.
- GPUI covers: Right edge (title + description + content + main-area Cancel/Save), Left edge (edge + title + content). Interactive via `overlay_state`. — missing: actions rendered as `main_content` rather than a footer actions row (no `.drawer__actions` part demonstrated).
- Jetstream covers: With title and content, With description, **Right edge with actions** (Cancel/Save footer), **Left edge** (Navigation), Empty content. — remaining: interactive open/close (static render only; preview-loop).

## Notes

- The big spread is the two Rust targets diverging on surface sizing: GPUI uses literal 28rem min-width with no top/bottom edge sizing; Jetstream uses a per-`size` 18/24/32rem switch unrelated to the contract's edge-based `min(28rem)`/`min(24rem)`. Both should resolve from the contract's edge-anchored dimensions.
- Jetstream's close-`x` icon is the only extra anatomy part across targets; neither contract nor Svelte has it.
- consv=ok: the dvh/per-edge-border/treatment-token deltas are doc-sync items, not functional Svelte bugs.
