<!-- parity consv=fixed gpui=2 jetstream=4 specimen=gap -->
# Parity: ListCard

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/list-card.md`
- Svelte (authoritative): `packages/svelte/components/src/ListCard.svelte`
- GPUI: `packages/gpui/components/src/primitives/list_card.rs`
- Jetstream: `packages/jetstream/components/src/list_card.rs`
- Spec: `packages/contracts/components/src/list_card.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ListCardSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/list_card.rs` · jetstream `packages/jetstream/preview/src/specimens/list_card.rs`

## Contract ↔ Svelte

Svelte had several props/snippets the contract §3 did not document. Svelte is authoritative — contract updated.

- [x] FIXED `highlighted?: boolean` (default false) added to contract §3 props, §4 states, §8 "Root highlighted" token block (border tint + accent gradient + inset accent ring), and §10 `data-highlighted` note (Svelte `:25,67,849-864`).
- [x] FIXED `selectionIndicator?: "none" | "checkbox"` (default `"none"`) added to §3; selection-indicator anatomy (overlay-over-leading vs inline) + size token documented in §2/§8 (Svelte `:26,68,1082-1097`).
- [x] FIXED Built-in context-menu cluster (`contextMenuItems`, `contextMenuAriaLabel`, `contextMenuTrigger`, `onContextAction`) added to §3 props/callbacks; §7 + §10 now describe the card-owned context menu (right-click / leading-trigger / keyboard) alongside the external-wrapper alternative (Svelte `:32-37,702-755`).
- [x] FIXED `corner?: Snippet` added to §3 snippet list, §2 anatomy (header-accessories cluster), and §8 Badges-and-Corner token block (Svelte `:44,417-419,1266-1275`).
- All other props match contract §3 exactly.

## GPUI gap (vs Svelte + contract)

- [x] DONE Leading dimensions — `leading_size_rem()` (circle 2rem / square 2.75rem, compact −0.25rem); radius = pill (circle) / `radius.control` (square). No raw px. (`leadingSizeOffset` prop still unmodeled — see remaining.)
- [x] DONE Sash placement + tokens — moved to top-left (`top 0.34375rem; left 0`), px/py/font from contract-exact rem, color from `sash_bg_token()` + hex override. NOTE: GPUI 0.2.2 `div` has no rotation, so the diagonal `rotate(-45deg)` is a platform gap (now a corner block, not a ribbon).
- [x] DONE not-live opacity — now `not_live_opacity()` = 0.72 (was 0.6). Dashed border kept. NOTE: `filter: grayscale(1)` and the thicker `0.1875rem` dashed stroke have no GPUI div API — platform gaps.
- [x] DONE `layout` support — `ListCardLayout` field; `stacked` switches root to `flex_col`/top-leading, `compact` shrinks leading.
- [x] DONE `accentColor` on leading — `parse_hex_color(spec.accent_color)` overrides leading bg + icon color; falls back to theme accent.
- [x] DONE Reorder handle — dot size `0.1875rem`, gap from `space.inline.xs` token. No raw px.
- [x] DONE Root padding — `0.625rem` y kept; x from `space.inline.md` (= `0.75rem`, contract-exact). Verified token resolves to 0.75rem.
- [ ] `leadingSizeOffset` prop still unmodeled (spec lacks the field) — leading ladder offset unimplemented. (Remaining.)
- accepted: no ARIA (gpui has no accessibility API) — role/aria-pressed/aria-disabled/anchor semantics not emitted.

## Jetstream gap (vs Svelte + contract)

`js_list_card` rebuilt — leading (shape/fill/accent) + body + meta + selection + sash + reorder handle + hover/highlighted/not-live states.

- [x] DONE Leading size follows shape — `leading_size_rem()` (circle 2rem / square 2.75rem, compact shrinks). Not a fixed 1.5rem.
- [x] DONE Tint ratio — `leading_tint_ratio()` = 0.12 (was 0.14).
- [x] DONE Body gap — `body_gap_rem()` = 0.0625 (was 0.125).
- [x] DONE Solid leading icon color — now `on_accent_color_token()` (`color.text.inverse`, the closest on-accent token; no pure-white token exists — NOTE in spec).
- [~] PARTIAL badges/footer/trailing — these are host snippets (no JsEl snippet API); meta is rendered. badges/footer counters still need a data channel. (Remaining — not spec-modeled.)
- [x] DONE Sash + reorder handle + selectable/selected indicator + not-live — all now rendered. Sash is a top-left block (JsEl has no rotate — NOTE); not-live = opacity 0.72 (no dashed/grayscale in JsEl — NOTE).
- [x] DONE Hover state — interactive cards set hover bg + border (`color-mix` 82% / border-default 52%). Focus ring color surfaced; painted by preview focus layer (JsEl focusable is single-affordance — NOTE).
- [x] DONE `layout` (compact/stacked) — `ListCardLayout` field; stacked = column, compact = tighter gap + smaller leading.
- [x] DONE `accentColor` — `hex_to_rgb255(spec.accent_color)` overrides leading bg/icon.
- [x] DONE Meta font — now `small_font_size_rem()` = 0.75rem (was caption 0.6875). NOTE: `tabular-nums` has no JsEl API.
- accepted: interaction (click/keyboard) lives in preview event loop.

## Specimen parity

- Svelte covers (602 lines): Interactive cards, Hierarchy titles (titleContent + chevrons), Rounded-square leading, With badges, With footer counters, Solid fill + accent colors, With context menu, Not-live (dashed), Corner sash badges, Static card.
- GPUI covers (823 lines): broad — interactive, leading shapes, sash, reorder handle, selectable, footer, not-live, etc. — closest target. Verify hierarchy-title and accent-color solid groups render correctly (accentColor unsupported on leading).
- Jetstream covers (41 lines, **1 group "List cards"**): badly under-covers. Missing badges, footer counters, sash, not-live, selectable, rounded-square+solid+accent, hierarchy, context menu. Largest specimen gap of the assigned set.

## Notes

- `ListCardSpec` (`packages/contracts/components/src/list_card.rs`) is missing fields for `layout`, `leading_size_offset`, `selection_indicator`, `highlighted`, and the context-menu cluster — so neither Rust target *can* implement those without spec changes. The token methods that exist are well-used (GPUI resolves fill/border/hover/leading from them); the gaps are unmodeled features and a handful of hardcoded dimensions.
- The GPUI sash is the most visible visual bug: it renders a top-right unrotated block instead of the contract's diagonal top-left ribbon. Fix placement + `rotate(-45deg)`.
- Jetstream is the priority: bring `js_list_card` up to at least badges/footer/sash/not-live/selectable and expand the specimen to mirror Svelte's groups (it currently renders a single basic row, which misrepresents the component's surface).
