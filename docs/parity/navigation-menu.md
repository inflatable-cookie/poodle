<!-- parity consv=fixed gpui=3 jetstream=0 specimen=gap -->
<!-- pass 41: trigger height + hover + min-height pass. GPUI: trigger min-height now
     flat size.control.height (dropped the bogus `- px(2.0)` inset + per-size offset,
     matching contract §8 / Svelte); viewport shadow already token-resolved
     (elevation_overlay_shadow). Jetstream: added trigger min-height (control-height) +
     hover state (accent 12%) + contract-faithful density padding (comfortable 0.75rem,
     not the generic 1.0rem ladder); pad_y/gap literals were already gone. Probe-tested.
     Remaining GPUI: icon field (spec divergence, code-side reconcile, out of scope) +
     roving tabindex/arrow-keys + click toggle-close (preview-loop, Tier-1). Remaining
     Jetstream: box-shadow (no JsEl channel, accepted) + roving/outside-close (preview-loop). -->
<!-- pass 30: viewport panel added — renders on active item with §8 tokens (padding
     space.panel.x/y, border 0.0625rem border-subtle@74%, radius.surface, bg panel@96%);
     content = active item description. Probe-tested. Remaining jetstream: box-shadow
     (elevation.overlay — JsEl has no shadow channel, accepted) + richer viewport content. -->
<!-- pass 29: Jetstream items now render contract-faithful pill triggers — pill radius +
     border, idle fill surface@88%/border-subtle@72%, active fill accent@16% + border
     accent/border-default@42% (was the wrong hover accent@12% with no border); list gap
     space.inline.sm. Probe-tested. Remaining jetstream: the viewport panel part
     (border/radius/bg/elevation/padding) is still absent. -->
# Parity: NavigationMenu

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/navigation-menu.md`
- Svelte (authoritative): `packages/svelte/components/src/NavigationMenu.svelte`
- GPUI: `packages/gpui/components/src/primitives/navigation_menu.rs`
- Jetstream: `packages/jetstream/components/src/navigation_menu.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/NavigationMenuSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/navigation_menu.rs` · jetstream `packages/jetstream/preview/src/specimens/navigation_menu.rs`

## Contract ↔ Svelte

Divergences between authoritative Svelte and contract §8/§3. Svelte wins unless it drops contract-specified functionality.

- [x] FIXED — **Trigger `min-height`.** Contract §8 base + size table dropped the `- 0.125rem` offset and per-size offsets; now flat `var(--poodle-size-control-height)` for every size, matching Svelte (`NavigationMenu.svelte:210,249-270`).
- [x] FIXED — **Size padding + font.** Per-size padding column removed (Svelte keeps `0 var(--poodle-space-control-x)` for all sizes); font column reconciled to Svelte values (xs `0.6875`, sm/md inherit base `0.75`, lg `0.8125`, xl `0.875`).
- [x] FIXED — **Token vs literal for base metrics.** Contract §8 + §7 now name the tokens: trigger gap + list gap = `--poodle-space-inline-sm`, trigger padding = `0 var(--poodle-space-control-x)`, root gap = `--poodle-space-stack-md`.
- [x] LEFT (Svelte gap noted) — **`icon` field unused.** Per "never weaken a contract feature just because Svelte hasn't shipped it", `icon` stays in contract §3. Added a §9 Svelte-gap note: the trigger renders only `__label` and never reads `item.icon`; Svelte should render the icon ahead of the label (the trigger gap token already exists for it).
- [x] FIXED — **Density padding values.** Added a Density adjustments table to contract §8: compact `0.5rem`, default inherits `--poodle-space-control-x`, comfortable `0.75rem` (horizontal-only, orthogonality-compliant).
- **Home/End keyboard** — no divergence (Svelte `End` → last enabled via backward search). Note only; no change.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED (pass 41, was already done) — **viewport shadow** now token-resolved via `elevation_overlay_shadow()` (`navigation_menu.rs` viewport branch), reading `poodle_tokens::typed::semantic::ELEVATION_OVERLAY`. No raw HSLA/px literals remain.
- [x] FIXED (pass 41) — **trigger height literal removed.** `trigger_height` is now flat `size.control.height` (dropped both the per-size `size_height_offset_rem` and the `- px(2.0)` inset), matching contract §8's flat min-height and Svelte `NavigationMenu.svelte:210`. No bare px literal.
- [ ] **No icon support** — `NavigationMenuEntry` has no `icon` field (only `description`); GPUI trigger never renders an icon. Contract §3 `icon` absent from the Rust spec entirely (`contracts/components/src/types.rs:690-695`).
- [ ] **No roving tabindex / arrow-key focus movement** — triggers are `.focusable()` (`:169`) but there is no ArrowLeft/Right/Home/End handling and no roving `tabindex` (contract §6, §11 Tier-1). Keyboard nav across triggers is absent.
- [ ] **Click activates but does not toggle/close** — `on_click` calls the host `on_change` with the value (`:207-209`); there is no Escape/outside-click close and no toggle-off (Svelte `toggleValue` sets null when re-clicking). Active item can never be closed to show "all closed" state.
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded`/`aria_controls`/`aria-labelledby` (contract §6) not emitted.
- accepted: viewport uses `description` content instead of Svelte's `children` snippet (contract §12 Known Delta: viewport content rendering strategy may differ; slot props equivalent via `current_item()`).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED (pass 29) — **pill border on trigger** present (border 0.0625rem + `trigger_radius_token()`).
- [x] FIXED (pass 29) — **idle trigger background** = surface 88%.
- [x] FIXED (pass 29) — **active treatment** = accent 16% bg + accent-42%/border-default blended border.
- [x] FIXED (pass 41) — **hover state** added: `.hover(|s| s.bg(tint(accent, 0.12)))` (accent 12%, contract §8 Hover/Focus). Active triggers keep their accent-16% open fill. Disabled triggers get no hover.
- [x] FIXED (pass 30) — **viewport** part renders on active item (border/radius/bg/panel padding); box-shadow omitted (no JsEl channel, accepted).
- [x] FIXED (pass 29, confirmed pass 41) — **no `pad_y` literal**: trigger has no baked vertical padding; height now comes from `min-height` (control-height).
- [x] FIXED (pass 29) — **list gap** resolves from `space.inline.sm` (`resolve_px(theme, "space.inline.sm")`), not a literal.
- [x] FIXED (pass 41) — **trigger min-height** now set to `size.control.height` (`resolve_px(theme, "size.control.height")`), matching Svelte's control-height tie. Trigger is `.items_center()` so the label vertically centers within the min-height.
- accepted: no ARIA channel (no accessibility API in jetstream runtime).
- accepted: roving-tabindex / arrow-key focus and outside-click close may live in preview `main.rs` event loop — verify there before adding to the component.

## Specimen parity

- **Svelte covers** (`NavigationMenuSpecimen.svelte`): Horizontal navigation with controlled `value`, `onValueChange`, disabled Changelog item, viewport `children` snippet ("Active section"), plus `sizes` and `densities` snippet matrices.
- **GPUI covers** (`navigation_menu.rs`): Horizontal navigation (active=Components, disabled Changelog), interactive `on_change` updating active value, "Active section" text, full sizes + densities matrices via `specimen_layout`. — missing: none material; closest parity of the three.
- **Jetstream covers** (`navigation_menu.rs`): three static groups — Default (auto-select first), Active=Contracts, With disabled entry. — **missing: sizes matrix, densities matrix, viewport panel, controlled/interactive selection, aria-labelled nav.** Uses a different item set (Docs/Contracts/Tokens/Changelog) than the contract §13 specimen (Home/Components/Tokens/Guides/Changelog).

## Notes

- The Rust `NavigationMenuEntry` (`contracts/components/src/types.rs:690-695`) drops the contract's `icon` field and adds a non-contract `description` field used to drive the GPUI viewport. This is a spec-level divergence from both contract §3 (which has `icon`, no `description`) and Svelte (which has `icon` in the TS type and a `children` snippet, no per-item description). Reconcile: add `icon`, and decide whether `description` is an accepted Rust-only viewport shortcut (note it in §12 if kept).
- Biggest single gap: **Jetstream is visually a plain text-button row** — missing the pill border, idle background, correct active/hover treatment, and the entire viewport. It does not yet implement the navigation-menu contract's anatomy.
- GPUI is the most faithful target but still lacks keyboard roving focus and close-on-Escape/outside-click; both are Tier-1 strict-parity items in contract §11.
- `consv=fixed`: contract §8 size/font/padding table reconciled to Svelte's token-resolved, flatter size variants (font-size-only stepping; min-height/padding pinned to control tokens); base metrics now name tokens; density padding row added. The `icon` field stays in the contract per the no-weakening rule, flagged as a Svelte under-implementation (§9). Remaining Rust spec note: `NavigationMenuEntry` (`types.rs:690-695`) drops `icon` and adds non-contract `description` for the GPUI viewport — code-side reconciliation, out of scope here.
