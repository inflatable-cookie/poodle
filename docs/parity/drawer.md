<!-- parity consv=ok gpui=6 jetstream=6 specimen=gap -->
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

- [ ] Hardcoded shadow color literals `hsla(0.0, 0.0, 0.0, 0.12)` / `0.08` at `drawer.rs:186,192` — resolve `elevation-dialog` from a shadow token, not raw HSLA + raw `px(8.0)`/`px(24.0)` blur/offset.
- [ ] Hardcoded backdrop color `hsla(0.0, 0.0, 0.5)` at `drawer.rs:263` — contract backdrop is `color.background.overlay`; resolve via `spec.backdrop_fill_token()` (Jetstream does this), not a raw 0.5 alpha.
- [ ] Hardcoded surface min-width `px(rem_to_px(28.0))` at `drawer.rs:166` — contract `min(28rem, 100vw)`; the 28rem is a literal, and there is no `top`/`bottom` edge override to `min(24rem,…)` (only left/right width handled; edge sizing for top/bottom missing).
- [ ] No `size` handling — `effective_size` resolved at `drawer.rs:136` then unused; contract §8 size table (header title font-size xs→xl) not applied. Title uses flat `heading_size` (`drawer.rs:208`).
- [ ] No header bottom-margin / actions row — surface stacks title+description+content with a flat `gap(stack_gap)` (`drawer.rs:182`); contract header `margin-bottom: space-stack-md` and `.drawer__actions` row (flex-end, wrap) are absent.
- [ ] Main-area fallback text `px(rem_to_px(0.75))` at `drawer.rs:251` — hardcoded 0.75rem; preview-scaffold leak inside the component, resolve from a caption token.
- accepted: no ARIA (gpui has no accessibility API) — `role="dialog"`/`aria-modal` not emittable.
- accepted: edge slide motion curve differs (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] Ad-hoc panel sizing `rem_to_px(18.0|32.0|24.0)` at `drawer.rs:32-36` — contract surface is `min(28rem,100vw)` (left/right) and `min(24rem,100vh)` (top/bottom), not a per-`size` 18/24/32rem switch. Resolve fixed dims from contract values, not a size heuristic.
- [ ] Hardcoded panel inner gap `rem_to_px(0.75)` at `drawer.rs:48` and header gap `rem_to_px(0.25)` at `drawer.rs:82` — contract header gap is `0.375rem`; resolve from space tokens.
- [ ] Title font is `size_font_rem(size) + 0.1875` literal offset at `drawer.rs:19` — contract header title is `1rem` (size-table driven); drop the ad-hoc +0.1875rem.
- [ ] Renders a close-`x` icon in the header (`drawer.rs:39-43,106`) — not in contract anatomy (no close button part); Svelte has no close icon. Remove or add to contract.
- [ ] No `margin-bottom` between header and body, no `.drawer__actions` footer row — contract Actions part absent; `actions` snippet equivalent never composed.
- [ ] No `size` table application (header title font-size per size) and no density-to-gap mapping beyond panel padding.
- accepted: no ARIA channel (`role="dialog"`).
- accepted: open/close + focus trap + scroll-lock interaction lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Right edge (trigger + title + description + actions Cancel/Save), Left edge (trigger + title, no actions). Interactive open/close.
- GPUI covers: Right edge (title + description + content + main-area Cancel/Save), Left edge (edge + title + content). Interactive via `overlay_state`. — missing: actions rendered as `main_content` rather than a footer actions row (no `.drawer__actions` part demonstrated).
- Jetstream covers: With title and content, With description, Empty content. — missing: **Left edge** variant, **actions footer** group, interactive open/close (static render only).

## Notes

- The big spread is the two Rust targets diverging on surface sizing: GPUI uses literal 28rem min-width with no top/bottom edge sizing; Jetstream uses a per-`size` 18/24/32rem switch unrelated to the contract's edge-based `min(28rem)`/`min(24rem)`. Both should resolve from the contract's edge-anchored dimensions.
- Jetstream's close-`x` icon is the only extra anatomy part across targets; neither contract nor Svelte has it.
- consv=ok: the dvh/per-edge-border/treatment-token deltas are doc-sync items, not functional Svelte bugs.
