<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- corner-slot pass: both targets gained the header `corner` slot. GPUI `ListCard::with_corner(impl IntoElement)` renders top-right in a header row beside the title (tertiary color, gap inline-xs, shrink-proof). Jetstream `js_list_card_with_slots(..., corner: Option<JsEl>)` renders in the header-accessories cluster alongside badges (tertiary color); `js_list_card(spec, theme)` still delegates with empty slots. 2 new Jetstream render_probe tests (corner alone; corner + badges together). Both specimens gain a "With corner (header-corner slot)" group composing a real icon + label. Both previews build 0 errors. The only remaining unmodeled surface is the built-in context-menu cluster (needs a menu/interaction channel both Rust targets lack). -->
<!-- jetstream-slots pass: `js_list_card_with_slots(spec, theme, leading, badges, footer, actions, trailing)` added — host-snippet slots now match the contract §2 anatomy + GPUI's slot API. leading slot overrides the derived first-letter avatar (rendered inside the styled box); badges render in the header-accessories cluster beside the title; footer renders below the body with margin-top 0.125rem; actions sit in the right-edge lane after meta; trailing is the exclusive lane (replaces meta + actions). `js_list_card(spec, theme)` delegates with empty slots (back-compat, callers unchanged). 6 new render_probe tests (default no-slots, leading override, footer, badges-in-header, trailing, trailing-exclusive). Specimen gains With badges / With footer counters / With trailing / With actions composing real js_pill / js_list_card_counter / js_button / js_icon. Both previews build 0 errors. -->
<!-- pass 41: leadingSizeOffset modeled — additive `leading_size_offset: i32` on
     ListCardSpec, consumed by `leading_size_rem()` (±2 steps × 0.25rem, clamped,
     box ≥ 1rem); both targets inherit it through the spec helper. Jetstream gains
     a render_probe test (offset ±1 shifts the leading box 2→2.25/1.75rem).
     Remaining per target = host-snippet/runtime-limit gaps only (badges/footer/
     trailing need a JsEl/GPUI snippet data channel; sash rotate(-45deg), not-live
     grayscale + thick dashed stroke = JsEl/GPUI runtime limits, noted in code).
     contracts cargo test + gpui build + jetstream list_card tests all pass. -->
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
- [x] DONE `leadingSizeOffset` — additive `leading_size_offset: i32` on `ListCardSpec`; `leading_size_rem()` applies `offset.clamp(-2,2) * 0.25rem` over the shape base (box kept ≥ 1rem). GPUI's `leading_el` already reads `spec.leading_size_rem()`, so the offset flows through with no further GPUI code change.
- [x] DONE `corner` header slot — `ListCard::with_corner(impl IntoElement)` renders the corner content top-right in a header row beside the title (tertiary text color, gap inline-xs, shrink-proof; contract §2 Corner / §8 Header + Badges-and-Corner). No spec field — host element pass-through, mirroring `with_leading`/`with_footer`/`with_trailing`.
- accepted (runtime limit): badges are host snippets — GPUI takes `with_footer`/`with_trailing`/`with_corner` host elements but has no badges data channel; the contract badges cluster is host-composed, not spec-modeled.
- accepted (runtime limit): sash `rotate(-45deg)` — GPUI 0.2.2 `div` has no rotation transform; rendered as a top-left block. not-live `grayscale(1)` filter + the thicker `0.1875rem` dashed stroke have no GPUI div API.
- accepted: no ARIA (gpui has no accessibility API) — role/aria-pressed/aria-disabled/anchor semantics not emitted.

## Jetstream gap (vs Svelte + contract)

`js_list_card` rebuilt — leading (shape/fill/accent) + body + meta + selection + sash + reorder handle + hover/highlighted/not-live states.

- [x] DONE Leading size follows shape — `leading_size_rem()` (circle 2rem / square 2.75rem, compact shrinks). Not a fixed 1.5rem.
- [x] DONE Tint ratio — `leading_tint_ratio()` = 0.12 (was 0.14).
- [x] DONE Body gap — `body_gap_rem()` = 0.0625 (was 0.125).
- [x] DONE Solid leading icon color — now `on_accent_color_token()` (`color.text.inverse`, the closest on-accent token; no pure-white token exists — NOTE in spec).
- [x] DONE `leadingSizeOffset` — flows through `spec.leading_size_rem()` (additive `leading_size_offset` field; ±2 steps × 0.25rem, clamped). render_probe test asserts offset ±1 shifts the leading box 2→2.25/1.75rem.
- [x] DONE host-snippet slots — `js_list_card_with_slots(spec, theme, leading, badges, footer, actions, trailing, corner)` exposes the contract §2 optional slots as `JsEl` content (not spec fields). leading slot overrides the derived first-letter avatar (rendered inside the styled leading box); `badges: Vec<JsEl>` render in the header-accessories cluster beside the title (gap `space.inline.xs`, `flex_wrap`, shrink-proof); `corner` renders in the same header-accessories cluster alongside badges at the tertiary text color (contract §8 Badges-and-Corner); footer renders below the body column at `margin-top 0.125rem`, gap `footer_gap_rem()` (0.5rem); actions sit in the right-edge lane after meta; trailing is the exclusive lane and replaces meta + actions (contract §7). `js_list_card(spec, theme)` delegates with empty slots (back-compat). 8 render_probe tests cover override/footer/badges/trailing/trailing-exclusive/default/corner/corner+badges. Matches GPUI's `with_leading`/`with_footer`/`with_trailing`/`with_corner` and extends it with badges + actions per the contract.
- [x] DONE Sash + reorder handle + selectable/selected indicator + not-live — all now rendered. Sash is a top-left block (JsEl has no rotate — NOTE); not-live = opacity 0.72 (no dashed/grayscale in JsEl — NOTE).
- [x] DONE Hover state — interactive cards set hover bg + border (`color-mix` 82% / border-default 52%). Focus ring color surfaced; painted by preview focus layer (JsEl focusable is single-affordance — NOTE).
- [x] DONE `layout` (compact/stacked) — `ListCardLayout` field; stacked = column, compact = tighter gap + smaller leading.
- [x] DONE `accentColor` — `hex_to_rgb255(spec.accent_color)` overrides leading bg/icon.
- [x] DONE Meta font — now `small_font_size_rem()` = 0.75rem (was caption 0.6875). NOTE: `tabular-nums` has no JsEl API.
- accepted: interaction (click/keyboard) lives in preview event loop.

## Specimen parity

- Svelte covers (602 lines): Interactive cards, Hierarchy titles (titleContent + chevrons), Rounded-square leading, With badges, With footer counters, Solid fill + accent colors, With context menu, Not-live (dashed), Corner sash badges, Static card.
- GPUI specimen DONE: full contract-state coverage — interactive, rounded-square leading, badges (via trailing Pill), **With corner** (icon + label header-corner slot), footer counters (+ inherited typography), solid fill + accent colors, context menu, not-live, sash, selectable, **link roots (href)**, **highlighted**, **selection-indicator checkbox**, **layout default/compact/stacked**, **leading-size-offset**, reorder handle, static. Every group is a real `ListCard::from_spec` resolving from tokens — no hand-rolled boxes. `gpui/preview` builds 0 errors.
- Jetstream specimen DONE: interactive/disabled, rounded-square leading, solid-fill accent colors, not-live, sash, selectable, reorder handle, link roots (href), highlighted, selection-indicator checkbox, layout default/compact/stacked, leading-size-offset, static — plus the now-renderable slot groups: **With badges** (header-accessories `js_pill`s), **With corner** (header-corner `js_icon` + label, plus a corner+badges sharing-the-cluster card), **With footer counters** (`js_list_card_counter` row below the body), **With trailing** (exclusive `js_pill` lane), **With actions** (`js_button` / `js_icon` lane composed with meta). Every group is a real `js_list_card` / `js_list_card_with_slots` resolving from tokens — slot content is real components, no hand-rolled boxes. `jetstream/preview` builds 0 errors.

## Notes

- `ListCardSpec` (`packages/contracts/components/src/list_card.rs`) now models `layout`, `leading_size_offset`, `selection_indicator`, and `highlighted`. The host-snippet clusters (leading/badges/corner/footer/actions/trailing) are intentionally NOT spec fields — they are `JsEl`/element content the host composes and pass through `js_list_card_with_slots` (Jetstream) / `with_leading`/`with_corner`/`with_footer`/`with_trailing` (GPUI). The only remaining unmodeled surface is the context-menu cluster (Svelte-owned overlay; needs a menu/interaction channel both Rust targets lack). Token methods are well-used (both targets resolve fill/border/hover/leading/accent/sash from them).
- The GPUI sash is the most visible remaining visual delta: it renders a top-left unrotated block instead of the contract's diagonal `rotate(-45deg)` ribbon (GPUI 0.2.2 `div` has no rotation transform). Jetstream shares this limit.
- Both targets now expose every contract host slot (leading/badges/corner/footer/actions/trailing); the only remaining parity step is a menu/interaction channel for the built-in context menu.
