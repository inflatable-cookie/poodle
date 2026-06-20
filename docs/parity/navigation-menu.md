<!-- parity consv=gap gpui=5 jetstream=8 specimen=gap -->
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

- **Trigger `min-height`.** Contract §8 base = `calc(var(--poodle-size-control-height) - 0.125rem)`; Svelte (`NavigationMenu.svelte:210`) uses flat `var(--poodle-size-control-height)` with no `- 0.125rem`. Per-size table (§8) prescribes distinct offsets (xs −0.625rem … xl +0.375rem); Svelte size blocks (`:249-270`) all keep `min-height: var(--poodle-size-control-height)` unchanged. **Fix: either Svelte applies the offsets, or contract drops the size min-height column. Svelte is authority for the *current* shipped look → update contract §8 min-height rows to match flat control-height unless the offset is intended; flag for design.**
- **Size padding + font.** Contract §8 size table sets per-size padding (`0 0.625rem` … `0 1.125rem`) and font (`0.625rem` … `0.875rem`). Svelte size blocks (`:249-270`) keep `padding: 0 var(--poodle-space-control-x)` for every size (no per-size padding) and only override `font-size` for xs (`0.6875rem`), lg (`0.8125rem`), xl (`0.875rem`); **sm gets no font override** (inherits `0.75rem`, contract wants `0.6875rem`), and xs is `0.6875rem` (contract wants `0.625rem`). **Fix: reconcile the size table — Svelte values are authoritative; update contract §8 font column (xs 0.6875, sm absent/0.75, md 0.75, lg 0.8125, xl 0.875) and drop the per-size padding column.**
- **Token vs literal for base metrics.** Contract §8 hardcodes resolved values: gap `0.375rem`, padding `0 0.875rem`, root gap `0.5rem`, list gap `0.25rem`. Svelte resolves these from tokens: trigger gap + list gap = `--poodle-space-inline-sm` (`:209,:203`), trigger padding = `--poodle-space-control-x` (`:211`), root gap = `--poodle-space-stack-md` (`:195`). **Fix: contract should name the tokens, not bake literals (matches Svelte-is-authority convention).**
- **`icon` field unused.** Contract §3 NavigationMenuItem includes `icon: string | null` and anatomy §2 implies icon gap on trigger. Svelte trigger renders only `<span class="…__label">` (`:176`) — no icon element, never reads `item.icon`. **Fix: either Svelte renders the icon (contract-specified functionality missing → Svelte bug) or contract drops `icon`. Given the trigger gap token exists for an icon, Svelte is under-implementing → add icon rendering to Svelte, keep contract.**
- **Density padding values undocumented.** Svelte density blocks (`:273-274`) set `padding-inline: 0.5rem` (compact) / `0.75rem` (comfortable); default inherits `--poodle-space-control-x`. Contract §8 documents no density padding. **Fix: add a density padding row to contract §8.** (Horizontal-only override — compliant with size/density orthogonality.)
- **Home/End keyboard.** Contract §6 says End → last *enabled* trigger. Svelte `End` (`:108-111`) calls `findNextEnabledIndex(items, 0, -1)` (search backward from 0 wraps to last enabled) — behaviourally correct, note only. No divergence.

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
- Contract §8's literal-value size/font/padding table is the main `consv=gap` driver — Svelte resolves these from tokens and ships flatter size variants than the table claims.
