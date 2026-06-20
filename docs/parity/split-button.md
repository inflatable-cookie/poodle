<!-- parity consv=fixed gpui=4 jetstream=7 specimen=gap -->
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

- [ ] **Hardcoded shadow color literals** — menu uses `hsla(0.0, 0.0, 0.0, 0.10)` and `hsla(0.0, 0.0, 0.0, 0.06)` with `px(4.0)`/`px(16.0)`/`px(1.0)` dims (`split_button.rs:340-350`); contract §8 menu shadow is `var(--poodle-elevation-overlay)` (spec exposes `shadow_token()`). Resolve from the elevation token, not raw HSLA.
- [ ] **No loading spinner** — `is_loading` only triggers disabled opacity (`split_button.rs:170,304`); contract §4/§8 require the shared `Spinner` (ring/sm/current) in the primary half. Not rendered.
- [ ] **`min_w` fudge factor** — primary `min_w(... * 0.75)` (`split_button.rs:191`); contract primary `min-width: 4rem` flat. Drop the `* 0.75`.
- [ ] **Toggle width hardcodes `rem_to_px(2.0)`** (`split_button.rs:255`) ignoring the per-size toggle-width scale Svelte applies (`toggle-width-base` 1.75–2.5rem across sizes). Resolve per effective size.
- accepted: no ARIA (gpui has no accessibility API) — menu/menuitem/separator roles, aria-haspopup/expanded not emitted.
- accepted: menu open/close + click-outside are platform-owned (contract §12); menu positioning/flip not replicated (render-only `is_open`).

## Jetstream gap (vs Svelte + contract)

- [ ] **No menu at all** — `js_split_button` never reads `spec.items` or `spec.is_open` (`split_button.rs`); the entire dropdown panel (contract §2 Menu/Item/Separator) is absent. Render the menu like GPUI.
- [ ] **No loading state** — `spec.is_loading` ignored; no spinner, and `is_unavailable()` (disabled||loading) not used (only `is_disabled` checked, `split_button.rs:56`). Loading neither disables nor shows a spinner.
- [ ] **No hover / active / focus visual states** — contract §4 hover (`split-fill-hover`), active (darkened), focus ring; none applied.
- [ ] **No tone/variant differentiation beyond token lookup** — relies solely on `fill_token()`/`border_token()`/`text_token()`; primary/ghost/danger color-mix nuances (e.g. ghost transparent fill, primary danger inverse text) not verified against Svelte. Cross-check resolved tokens cover all combos.
- [ ] **Separator height `* 0.56`** (`split_button.rs:19`) — contract divider is `60%` of control height; GPUI uses `relative(0.6)`. Use 0.6.
- [ ] **Toggle padding `rem_to_px(0.375)`** ad-hoc (`split_button.rs:20,52`) — contract toggle is `width: 2rem; padding: 0`. Use fixed 2rem width, zero padding.
- [ ] **Primary padding from density** — `control_space_x_rem(spec.density)` (`split_button.rs:17`) folds density into padding; verify against Svelte where density adjusts via `padding-inline-density-adjust` (compact −0.25rem, comfortable +0.25rem) on top of `space.control.x`.
- accepted: interaction (menu open, click-outside, keyboard) lives in preview `main.rs` event loop; no ARIA channel.

## Specimen parity

- Svelte covers: Primary, Secondary, Danger, **Loading**, Disabled, Last-action readout, plus size + density grids. Interactive onClick/onAction.
- GPUI covers: Primary, Secondary, Danger, Loading, Disabled, **Submit semantics**, **Constrained scroll container** (flip test), Last action, size + density grids. Broadest specimen — exceeds Svelte. Menu shown via `is_open` is render-only.
- Jetstream covers: **only Default + Disabled** (`specimens/split_button.rs`). — missing: **Primary/Secondary variants**, **Danger**, **Loading**, **menu/items**, **size grid**, **density grid**, **Last action**. Far under Svelte.

## Notes

- `consv=fixed`: all contract drift resolved — literal rem/px values now reference tokens, `aria-haspopup` is `"true"`, the z-index token name matches Svelte, and the size table covers all five sizes. All were contract-side fixes per "Svelte is parity authority."
- GPUI's menu is the closest to Svelte; its main real gap is the hardcoded shadow + missing spinner. Jetstream needs the menu and loading spinner built before it can claim parity.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/split_button.rs:31`) — specimen chrome.
