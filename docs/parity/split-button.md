<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok | specimen pass: both Rust targets backfilled to full contract coverage with real js_split_button/SplitButton. Added variant×tone matrix (default/danger/success), dropdown-menu-open group (items + separator), loading, disabled, and xs–xl sizes; Jetstream rebuilt from Default+Disabled stub. Both previews build clean (0 err). -->
# Parity: SplitButton

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/split-button.md`
- Svelte (authoritative): `packages/svelte/components/src/SplitButton.svelte`
- GPUI: `packages/gpui/components/src/primitives/split_button.rs`
- Jetstream: `packages/jetstream/components/src/split_button.rs`
- Spec: `packages/contracts/components/src/split_button.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SplitButtonSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/split_button.rs` · jetstream `packages/jetstream/preview/src/specimens/split_button.rs`

## Contract ↔ Svelte

Svelte implements the full variant/tone/size/density system, menu with keyboard nav, click-outside, upward flipping, loading spinner, disabled. A few attribute/value mismatches make the contract wrong; Svelte is authoritative.

- [x] FIXED: contract §6 toggle `aria-haspopup` changed from `"menu"` to `"true"`, matching Svelte (`SplitButton.svelte:216`).
- [x] FIXED: contract §8 Item `padding` now references `var(--poodle-space-control-y) var(--poodle-space-control-x)` tokens (was literal `0.375rem 0.5rem`), matching Svelte (`:536`).
- [x] FIXED: contract §8 Item `min-height` now `var(--poodle-size-control-height)` (was `2rem`), matching Svelte (`:535`).
- [x] (already correct) Item border-radius `calc(var(--poodle-radius-control) - 0.125rem)` matches Svelte.
- [x] FIXED: contract §8 size table extended to all five sizes (`xs`/`sm`/`md`/`lg`/`xl`) with absolute height, font-size, toggle-width-base, and chevron-size columns per Svelte (`:398-436`). Replaced the old `calc()` height expressions with Svelte's absolute rem values.
- [x] FIXED: contract §8 menu `z-index` token reconciled to `var(--poodle-overlay-z-menu)` (was `var(--poodle-z-index-overlay-menu)`), matching Svelte (`:510`).

## GPUI gap (vs Svelte + contract)

GPUI renders both halves, divider (60% via `relative(0.6)`), variants incl. ghost, danger via tokens, hover/active/focus, disabled/loading opacity, menu overlay when `is_open`, items + separators, brand-raised treatment. Solid coverage.

- [x] **Hardcoded shadow color literals** — already resolved in code: the menu uses `crate::theme_ext::elevation_overlay_shadow()` (the `elevation.overlay` token), not raw HSLA. Doc was stale on this point.
- [x] FIXED **Loading spinner** — `is_loading` now renders the shared `Spinner` (ring/sm/current, `with_color(text_color)`) in the primary half before the label, alongside the disabled opacity (contract §4/§8).
- [x] FIXED **`min_w` fudge factor** — primary now `min_w(rem_to_px(4.0))` flat (contract §7), dropped the `* 0.75`.
- [x] FIXED **Toggle width** — now `split_button_toggle_width_rem(effective_size)` (new presentation helper, 1.75–2.5rem per size); chevron now sized via `split_button_chevron_size_rem` + `Icon::with_px_size`.
- accepted: no ARIA (gpui has no accessibility API) — menu/menuitem/separator roles, aria-haspopup/expanded not emitted.
- accepted: menu open/close + click-outside are platform-owned (contract §12); menu positioning/flip not replicated (render-only `is_open`).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED **Menu** — `js_split_button` now reads `spec.items` + `spec.is_open` and renders the dropdown panel (contract §2 Menu/Item/Separator): min-width 12rem, 0.25rem padding, surface-elevated fill/border/radius, per-item control padding + radius (control − 0.125rem) + accent-16% hover, and 0.0625rem separators. Wrapped row+menu in a column when open.
- [x] FIXED **Loading state** — uses `is_unavailable()` (disabled||loading) to dim + disable, and renders the ring spinner glyph in the primary half.
- [x] FIXED **Hover / active visual states** — per-segment `hover` (`color-mix(split-fill 84%, elevated)`) + `active` (72% mix) on both halves when available. (Focus ring: JsEl has no focus-style channel — accepted, same as other Jetstream controls.)
- [x] FIXED **Tone/variant differentiation** — new `resolve_split_colors` reproduces the full Svelte `--poodle-split-*` matrix: ghost = surface-42% fill / border-subtle-72% border (fully transparent for danger/success ghost, status text); primary = accent/status fill, mix(…84%, black) border, inverse text; secondary danger/success = mix(status 16%, surface) fill, mix(status 46%, border-default) border. Mirrors `js_button`.
- [x] FIXED **Divider height** — now `height * 0.6` (was `* 0.56`), centered by the row.
- [x] FIXED **Toggle padding/width** — fixed per-size width (`split_button_toggle_width_rem`), zero inline padding; the ad-hoc `rem_to_px(0.375)` trigger padding is gone.
- [x] (acceptable) **Primary padding from density** — `control_space_x_rem(spec.density)` returns compact 0.5 / default 0.75 / comfortable 1.0 rem, which equals Svelte's base `space.control.x` (0.75) plus the per-density ±0.25rem adjust. Already correct.
- accepted: interaction (menu open, click-outside, keyboard) lives in preview event loop; no ARIA channel.

## Specimen parity

- Svelte covers: Primary, Secondary, Danger, **Loading**, Disabled, Last-action readout, plus size + density grids. Interactive onClick/onAction.
- GPUI covers: Primary, Secondary, Danger, Loading, Disabled, **Submit semantics**, **Constrained scroll container** (flip test), Last action, size + density grids. Broadest specimen — exceeds Svelte. Menu shown via `is_open` is render-only.
- Jetstream covers: **only Default + Disabled** (`specimens/split_button.rs`). — missing: **Primary/Secondary variants**, **Danger**, **Loading**, **menu/items**, **size grid**, **density grid**, **Last action**. Far under Svelte.

## Notes

- `consv=fixed`: all contract drift resolved — literal rem/px values now reference tokens, `aria-haspopup` is `"true"`, the z-index token name matches Svelte, and the size table covers all five sizes. All were contract-side fixes per "Svelte is parity authority."
- GPUI's menu is the closest to Svelte; its main real gap is the hardcoded shadow + missing spinner. Jetstream needs the menu and loading spinner built before it can claim parity.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/split_button.rs:31`) — specimen chrome.
