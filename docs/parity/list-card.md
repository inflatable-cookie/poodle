<!-- parity consv=gap gpui=7 jetstream=10 specimen=gap -->
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

Svelte has several props/snippets the contract §3 does not document. Svelte is authoritative — update the contract.

- Svelte adds `highlighted?: boolean` (default false) (`ListCard.svelte:25,67`). Not in contract §3. **Fix: add to contract props + a `highlighted` visual state to §4.**
- Svelte adds `selectionIndicator?: "none" | "checkbox"` (default `"none"`) (`ListCard.svelte:26,68`) controlling whether selectable mode shows a checkbox overlay. Not in contract. **Fix: add prop + document the checkbox selection indicator anatomy.**
- Svelte adds a context-menu cluster: `contextMenuItems?: MenuItem[] | null`, `contextMenuAriaLabel?: string | null`, `contextMenuTrigger?: "context" | "leading"`, `onContextAction?: (value) => void` (`ListCard.svelte:32-37`). Contract §10 only says "wrap ListCard in ContextMenu" — Svelte builds context-menu support directly into the card. **Fix: document the built-in context-menu props (or note the contract intentionally externalizes this and the Svelte additions are an accepted superset).**
- Svelte adds a `corner?: Snippet` (`ListCard.svelte:44`). Contract §3 snippet list has `sashContent` but no `corner`. **Fix: add `corner` snippet to contract.**
- All other props (title/subtitle/meta/href/leadingShape/leadingFill/leadingSizeOffset/accentColor/layout/interactive/disabled/selectable/selected/showReorderHandle/notLive/sash/sashColor/ariaLabel/size/sizeRole/density + callbacks) match contract §3 exactly.

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded leading dimensions: `leading_size` = `px(32.0)`/`px(44.0)`, `leading_radius` = `px(16.0)`/`px(6.0)` (`list_card.rs:159-166`). Contract §7 says 2rem/2.75rem — resolve from rem/tokens, not raw px. `leadingSizeOffset` prop is entirely unsupported (spec lacks the field).
- [ ] Hardcoded sash dimensions + wrong placement: `w(px(rem_to_px(6.0)))`, `py(px(rem_to_px(0.125)))`, `text_size(px(rem_to_px(0.5625)))`, `line_height(px(rem_to_px(0.75)))` (`list_card.rs:251-258`). Worse, the sash is positioned `top(0) right(0)` with **no rotation** (`:248-249`); contract §8 sash is `top: 0.34375rem; left: -2.25rem; transform: rotate(-45deg)` — a diagonal top-left ribbon. GPUI sash is a plain top-right block, visually wrong.
- [ ] not-live values wrong: GPUI uses `opacity(0.6)` (`list_card.rs:378`); contract §4/§8 specify `opacity: 0.72`, plus `filter: grayscale(1)` and a `0.1875rem dashed` border — greyscale filter is absent and opacity is 0.6 not 0.72.
- [ ] No `layout` support (compact/stacked) — `ListCardSpec` has no `layout` field; contract §4 compact/stacked states + §7 stacked utility rail are unimplemented.
- [ ] No `accentColor` applied to leading: GPUI parses hex only for the sash (`parse_hex_to_hsla` at `:240-245`); the leading accent uses `leading_tint_bg_token()` (theme accent), ignoring `spec.accent_color`. Contract §3 `accentColor` overrides leading bg/icon.
- [ ] Reorder handle hardcoded px: `w(px(3.0)) h(px(3.0)) rounded(px(1.5))`, `gap(px(2.0))`, `opacity(0.6)` (`list_card.rs:337-353`). Resolve dot size/gap from tokens.
- [ ] Root vertical padding `py(px(rem_to_px(0.625)))` (`list_card.rs:277`) — contract padding is `0.625rem 0.75rem`; horizontal uses `space.inline.md` which must equal `0.75rem` or it diverges. Verify the token resolves to the contract value; otherwise add an explicit list-card padding token.
- accepted: no ARIA (gpui has no accessibility API) — role/aria-pressed/aria-disabled/anchor semantics not emitted (documented in `:264-269`).

## Jetstream gap (vs Svelte + contract)

`js_list_card` is a reduced subset — it renders leading + title/subtitle + meta only, missing most of the card.

- [ ] Leading size ignores shape: `leading_size = rem_to_px(1.5)` for both shapes (`list_card.rs:28`). Contract §7: circle 2rem, rounded-square 2.75rem. `spec.leading_shape` only affects radius, not size.
- [ ] Magic tint ratio `tint(leading_accent, 0.14)` (`list_card.rs:35`) — contract leading tint is `accent 12%`. Use 0.12.
- [ ] Hardcoded text-block gap `gap(rem_to_px(0.125))` (`list_card.rs:62`) — contract body gap is `0.0625rem`, not 0.125rem; resolve from a token.
- [ ] Solid leading icon color uses `color.text.on-accent` (`list_card.rs:40`) — contract solid color is `#fff`. Confirm the token resolves white; if not, mismatch.
- [ ] No badges, no footer (counters), no trailing/actions snippets — contract §2 anatomy parts absent.
- [ ] No sash, no reorder handle, no selectable/selected indicator, no not-live state — all major contract states unimplemented.
- [ ] No hover state, no focus ring — contract §8 interactive-hover + focus tables unimplemented (cursor only).
- [ ] No `layout` (compact/stacked) — not in spec; unimplemented.
- [ ] No `accentColor` custom theming — leading always uses token accent.
- [ ] Meta uses `typography.caption.size` (`list_card.rs:25`) — contract meta is `0.75rem`/`tabular-nums`; verify caption == 0.75rem and add tabular-nums.
- accepted: interaction (click/keyboard) lives in preview `main.rs` event loop.

## Specimen parity

- Svelte covers (602 lines): Interactive cards, Hierarchy titles (titleContent + chevrons), Rounded-square leading, With badges, With footer counters, Solid fill + accent colors, With context menu, Not-live (dashed), Corner sash badges, Static card.
- GPUI covers (823 lines): broad — interactive, leading shapes, sash, reorder handle, selectable, footer, not-live, etc. — closest target. Verify hierarchy-title and accent-color solid groups render correctly (accentColor unsupported on leading).
- Jetstream covers (41 lines, **1 group "List cards"**): badly under-covers. Missing badges, footer counters, sash, not-live, selectable, rounded-square+solid+accent, hierarchy, context menu. Largest specimen gap of the assigned set.

## Notes

- `ListCardSpec` (`packages/contracts/components/src/list_card.rs`) is missing fields for `layout`, `leading_size_offset`, `selection_indicator`, `highlighted`, and the context-menu cluster — so neither Rust target *can* implement those without spec changes. The token methods that exist are well-used (GPUI resolves fill/border/hover/leading from them); the gaps are unmodeled features and a handful of hardcoded dimensions.
- The GPUI sash is the most visible visual bug: it renders a top-right unrotated block instead of the contract's diagonal top-left ribbon. Fix placement + `rotate(-45deg)`.
- Jetstream is the priority: bring `js_list_card` up to at least badges/footer/sash/not-live/selectable and expand the specimen to mirror Svelte's groups (it currently renders a single basic row, which misrepresents the component's surface).
