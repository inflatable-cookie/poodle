<!-- parity consv=fixed gpui=2 jetstream=1 specimen=gap | GPUI specimen done; Jetstream pending engine recovery. GPUI specimen now full: File/Edit/View triggers, open overlay with shortcuts+separators, checkbox + radio rows (View menu), disabled item (Edit), disabled trigger (Window), sizes + densities. -->
<!-- pass 41: Jetstream menubar rebuilt to full contract anatomy. Added: List chrome
     (border-subtle 72%, radius-surface, panel-96% bg, 0.1875rem padding, 0.125rem gap);
     Overlay/dropdown rendering current_menu().items — action rows (label + shortcut meta),
     checkbox/radio rows, and separators (border-subtle 72%, 0.0625rem, 0.25rem margin);
     overlay bg = elevated-98%-over-panel, border = border-default 72%, min-width 12rem,
     radius-surface, shadow_md (box-shadow approximation). Trigger now radius-control +
     min-height control-height; open/hover = accent 14% (was the wrong 12%); weight = a
     LABEL_WEIGHT=600 constant (dropped raw 600/400 magic) and text stays text-primary
     (dropped non-contract muted idle text). pad_y literal gone (height via min-height).
     Probe-tested (list chrome + open trigger fill + open-menu items/separators/shortcuts).
     Remaining Jetstream: focus-ring treatment (JsEl has only .focusable(), no ring channel)
     + box-shadow (shadow_md preset, not token-driven) — both accepted runtime limits.
     GPUI unchanged: 2 remaining are the inherited Menu shadow literal (lives in menu.rs,
     out of scope) + the overlay-anchoring layout delta (accepted). -->
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

- [ ] Inherits Menu's hardcoded HSLA shadow literals via the delegated dropdown (`menu.rs:201,207`) — see menu.md; the menubar overlay shadow is therefore also non-token. Lives in `menu.rs`, OUT OF SCOPE for menubar files; fix on the menu pass.
- [x] JUSTIFIED (pass 41) — trigger-row `gap(px(rem_to_px(0.125)))` / `p(px(rem_to_px(0.1875)))` (`menubar.rs:144-145`) are the contract's literal list gap `0.125rem` / padding `0.1875rem` (NOT token-backed values — `trigger_gap_token()` resolves to inline-sm 0.5rem, which is the wrong value and intentionally unused). `rem_to_px(<contract-exact rem>)` is not a hardcode violation per the parity rules; the Jetstream side inlines the same two rems for the same reason.
- [ ] Overlay anchors below the whole wrapper (`menubar.rs:199-220`) via a flex column, not absolutely positioned under the specific trigger group; contract overlay is `position: absolute; left: 0` under its group with `0.25rem` gap. Acceptable as a GPUI layout delta but note: dropdown does not align to the active trigger's left edge.
- accepted: no ARIA (roles/aria-* not emitted); roving-focus keyboard nav across triggers is render-driven by `current_value` rather than internal focus state (interaction lives in preview event loop).

## Jetstream gap (vs Svelte + contract)

Rebuilt to full contract anatomy (pass 41): List chrome + Trigger strip + Overlay dropdown.

- [x] FIXED (pass 41) — **List chrome** present: border 0.0625rem border-subtle 72%, radius-surface, panel-96% bg, 0.1875rem padding, 0.125rem gap (contract §8 List).
- [x] FIXED (pass 41) — **Overlay/dropdown** renders `current_menu().items`: action rows (label + shortcut/check meta), checkbox/radio rows, and separators. Overlay bg = color-mix(elevated 98%, panel), border = border-default 72%, min-width 12rem, radius-surface, shadow_md (contract §8 Overlay/Item/Separator/Meta).
- [x] FIXED (pass 41) — **No magic px / raw weights**: `pad_y` literal gone (height via min-height); trigger + item weight use a `LABEL_WEIGHT=600` constant (mirrors GPUI `FontWeight::SEMIBOLD` / contract `typography.label.weight`, per the `form_shell.rs` convention). Idle trigger text now stays text-primary (dropped the non-contract muted/400 idle treatment).
- [x] FIXED (pass 41) — **`radius.control` on triggers** applied. Focus-ring treatment is NOT applied — JsEl exposes only `.focusable()`, no focus-ring channel (accepted runtime limit; below).
- [x] FIXED (pass 41) — **open menu shown** via the rendered Overlay (open trigger fill = accent 14%; the dropdown items render below).
- [x] FIXED (pass 41) — **size-table dimensions**: trigger + item `min-height` = `size.control.height`; item padding = `space.control.y` / control-x; font steps via `size_font_rem` (trigger) / `typography.body.size` (item).
- accepted: focus-ring treatment — JsEl has `.focusable()` but no focus-ring draw channel; the visible ring lives in the preview event loop / is a runtime limit.
- accepted: box-shadow uses the `shadow_md()` preset, not a token-driven `elevation.overlay` (JsEl has no token box-shadow channel).
- accepted: interaction (trigger click, item nav, outside-click close) lives in preview event loop.

## Specimen parity

- Svelte covers: Application menu bar with File/Edit/View, each with shortcut items + separators; interactive open/switch (`MenubarSpecimen.svelte`).
- GPUI covers: **GPUI specimen done.** File/Edit/View triggers with shortcut items + separators, open overlay via real `Menu`, **checkbox + radio rows** in the View menu (live-toggled checked state), **disabled item** (Paste Special in Edit), **disabled top-level trigger** (Window), tracked open value + selected action, plus full **sizes + densities** matrices (`menubar.rs`). Covers every contract §4 state and §13 specimen; checkbox/radio/disabled added beyond §13.
- Jetstream covers: Default (File open), Edit open, With disabled entry (`menubar.rs:46-57`). — triggers render with open/disabled states, but **no dropdown items render** (impl has no overlay), so the submenu specimen content is invisible. `specimen=gap`.

## Notes

- `consv=fixed`: contract §8 size table reconciled to Svelte (font-size-only stepping; min-height/padding pinned to control tokens); base trigger/item font/weight/dimension rows now cite tokens instead of resolved literals.
- Jetstream is the weakest target: trigger strip only, no list chrome, no dropdown — roughly half the contract unimplemented. GPUI is close to parity (chrome + delegated Menu dropdown); its main debt is the inherited shadow literal and overlay-anchoring delta.
