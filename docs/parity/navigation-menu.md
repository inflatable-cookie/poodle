<!-- parity consv=fixed gpui=5 jetstream=4 specimen=gap -->
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

- [ ] **Hardcoded viewport shadow literals** at `navigation_menu.rs:236-246` — two `BoxShadow` with raw `hsla(0,0,0,0.10/0.06)`, `px(4.0)`, `px(16.0)`, `px(1.0)`, `px(4.0)`. Contract §8 viewport `box-shadow` = `var(--poodle-elevation-overlay)`. Resolve from the elevation token, not raw HSLA/px.
- [ ] **`- px(2.0)` literal** for trigger height at `navigation_menu.rs:150` — the `0.125rem` inset must resolve via `rem_to_px(0.125)` (or a token), not a bare `2.0` px literal.
- [ ] **No icon support** — `NavigationMenuEntry` has no `icon` field (only `description`); GPUI trigger never renders an icon. Contract §3 `icon` absent from the Rust spec entirely (`contracts/components/src/types.rs:690-695`).
- [ ] **No roving tabindex / arrow-key focus movement** — triggers are `.focusable()` (`:169`) but there is no ArrowLeft/Right/Home/End handling and no roving `tabindex` (contract §6, §11 Tier-1). Keyboard nav across triggers is absent.
- [ ] **Click activates but does not toggle/close** — `on_click` calls the host `on_change` with the value (`:207-209`); there is no Escape/outside-click close and no toggle-off (Svelte `toggleValue` sets null when re-clicking). Active item can never be closed to show "all closed" state.
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded`/`aria_controls`/`aria-labelledby` (contract §6) not emitted.
- accepted: viewport uses `description` content instead of Svelte's `children` snippet (contract §12 Known Delta: viewport content rendering strategy may differ; slot props equivalent via `current_item()`).

## Jetstream gap (vs Svelte + contract)

- [ ] **No pill border on trigger** — Svelte trigger has `border 0.0625rem solid border-subtle 72%` + `radius-control` (`NavigationMenu.svelte:212-213`); `js_navigation_menu` (`navigation_menu.rs:31-37`) renders a bare `button` with no border, no radius. Contract §2 anatomy requires the pill border. Add border + `trigger_radius_token()`.
- [ ] **Wrong trigger background** — Svelte idle bg = `background-surface 88%` (`:214`); Jetstream applies no idle bg (only active gets `tint(accent, 0.12)` at `:20,:40`). Add idle surface-88% fill.
- [ ] **Wrong active treatment** — Svelte active = `accent 16%` bg + blended border (`accent 42%`/border-default) (`:223-226`); Jetstream active = `tint(accent, 0.12)` (`:20`) which matches *hover* 12%, not active 16%, and has no active border. Use accent-16% + active border token math.
- [ ] **No hover state** — contract §4 + Svelte `:228-232` give hover `accent 12%`; Jetstream component has no hover branch (interaction may live in preview loop — confirm, else add).
- [ ] **No viewport** — contract §2/§8 viewport part (border/radius/bg/shadow/panel padding) is entirely absent from `js_navigation_menu`; component renders only the trigger row (`:25-50`). GPUI renders it; Jetstream does not.
- [ ] **Hardcoded `pad_y = rem_to_px(0.25)`** at `navigation_menu.rs:15` — vertical padding baked to `0.25rem` literal; Svelte trigger has no vertical padding (height comes from `min-height`). Drop or derive from a size token; do not hardcode `0.25`.
- [ ] **Hardcoded gap `rem_to_px(0.25)`** at `navigation_menu.rs:25` — list gap literal; Svelte uses `--poodle-space-inline-sm`. Resolve from a space token (a `list_gap_token()` is not on the spec — add one).
- [ ] **No size/min-height handling** — `js_navigation_menu` resolves `size_font_rem` (`:12`) but never sets trigger min-height; Svelte ties height to `control-height`. No per-size dimension applied.
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
