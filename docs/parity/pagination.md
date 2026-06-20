<!-- parity consv=fixed gpui=8 jetstream=10 specimen=gap -->
# Parity: Pagination

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/pagination.md`
- Svelte (authoritative): `packages/svelte/components/src/Pagination.svelte`
- GPUI: `packages/gpui/components/src/primitives/pagination.rs`
- Jetstream: `packages/jetstream/components/src/pagination_comp.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PaginationSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/pagination.rs` · jetstream `packages/jetstream/preview/src/specimens/pagination.rs`

## Contract ↔ Svelte

Divergences in both directions. Svelte is authoritative except where it drops contract-specified functionality.

- FIXED — chrome/standalone model. Contract §8 root tables described the OLD standalone model (base `.pagination` carrying border/background + a `.pagination--standalone` strip-chrome variant). Svelte base `.poodle-pagination` is chrome-free (`padding:0`) and chrome is opt-in via `.poodle-pagination--chrome`. §8 rewritten: base root = chrome-free (`padding:0`, no border/bg/margin), new `Root chrome .pagination--chrome` table carries the padding/border-top/elevated-bg. The stale `standalone` strip-chrome table removed.
- FIXED — Button `height`: dropped the `−0.125rem`; §8 button height now `var(--poodle-size-control-height)` to match Svelte (line 427).
- Button `min-width` (`var(--poodle-size-control-height)`) already matched Svelte (line 426). The GPUI/Jetstream `2.25rem` hardcodes are code gaps, not contract↔Svelte.
- FIXED — Density gaps: §8 density table updated to Svelte's `compact 3px` / `default 0.25rem` / `comfortable 0.375rem` (lines 483/488/499); noted the base `space-inline-sm` and the compact-mode `0.25rem` collapse.
- FIXED — Size table: rewritten from calc-offsets to Svelte's flat rem (`xs 1.5rem`, `sm 1.75rem`, `md` no-override/inherits-base, `lg 2.75rem`, `xl 3.25rem`; fonts 0.6875/0.75/0.75/0.875/0.9375).
- FIXED — First/last glyph: §8 variant table + §9 notes now `««`/`»»` (double guillemets, lines 275/336), was `<<`/`>>`.
- First/last buttons gate on `variant === "full" && supportsGoToPage` (controller required) — contract §2/§3 already agree. No contract change; the specimen demo note is a code/specimen concern.
- FIXED — `compact` padding: §8 compact table now `padding: 0` (was `0.5rem 0.75rem`) to match Svelte `--compact` (lines 362–365); gap `0.75rem` unchanged.
- `limitOptions` default `[30, 50, 100]` already matched Svelte (line 62). OK.
- Also fixed in passing: `standalone` prop default corrected to `undefined` (was `false`) with the `chrome = !standalone` resolution note; §9/§6 limit-selector id corrected from the static `pagination-limit` to the per-instance `poodle-pagination-limit-{n}`; stale `margin-top` removed from the base root. (No contract change needed for the Rust-invented "Go to page" field — it never existed in the contract.)

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Button `min-width` hardcoded `rem_to_px(2.25)` at `pagination.rs:232`, `:348` — contract/Svelte min-width is `var(--poodle-size-control-height)`; resolve from `size.control.height` token, not a raw `2.25`.
- [ ] Ellipsis `min-width` hardcoded `rem_to_px(1.5)` at `pagination.rs:386` — Svelte ellipsis min-width is `1.5rem` (contract §8); acceptable as a contract constant but should route through a token/named const, not an inline float.
- [ ] Go-to field width hardcoded `rem_to_px(3.0)` at `pagination.rs:642`, `:688` — no contract basis (the whole go-to field is a GPUI invention, see below); derive from a token.
- [ ] **Go-to input field is not in the Svelte full variant.** Svelte full variant renders prev / "Page X of Y" summary / next / first / last only (lines 308–338). GPUI renders a "Go to" text-input row (`pagination.rs:619-701`) that has no Svelte or contract counterpart. **Remove or move behind an explicit contract opt-in.**
- [ ] Full variant center content wrong: Svelte shows `Page X of Y` summary (line 310); GPUI numbered+full both render the numbered page row and only the **Simple** branch prints "Page X of Y" (`pagination.rs:594-604`). Full variant shows numbered pages instead of the page-of-total summary. **Swap: Full must show "Page X of Y", Simple must show "X–Y of Z" item range.**
- [ ] Simple variant summary wrong: Svelte simple shows item range `X–Y of Z` (lines 312–315); GPUI simple prints `Page {current} of {total}` (`pagination.rs:599`). **Fix to item-range string.**
- [ ] Info row format: Svelte renders "Showing X to Y of Z" (line 240); GPUI renders raw `info_text` + a separate `"{page_size} per page"` chip (`pagination.rs:464-486`) and never computes the "Showing X to Y of Z" string from total/page_size. **Compute info string to match Svelte.**
- [ ] First/last (`««`/`»»`) buttons absent entirely — GPUI never renders first/last even for full variant (no branch in `into_element`). Contract §2 + Svelte require them when goToPage is wired.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but no nav landmark / `aria-current` emitted.
- accepted: scroll targeting omitted (contract Known Delta, web-only).

## Jetstream gap (vs Svelte + contract)

- [ ] Go-to box width hardcoded `height * 2.0` at `pagination_comp.rs:293` — ad-hoc multiplier, no token/contract basis; the entire go-to row is a Jetstream invention (see below).
- [ ] **"Go to page:" row is not in Svelte.** `build_full_row` (`pagination_comp.rs:264-349`) renders a "Go to page:" label, a static bordered box, and a static page-size selector — none exist in the Svelte full variant. **Remove or gate behind contract.**
- [ ] Full variant missing "Page X of Y" summary — Jetstream full renders numbered pages (line 129 shares the Numbered arm) then the go-to row, never the contract's "Page X of Y" center text.
- [ ] First/last (`««`/`»»`) buttons absent — no branch renders them in any variant.
- [ ] Page buttons use `.w(height).h(height)` (`pagination_comp.rs:86-87`) — fixed square; Svelte/contract button `min-width = control-height` but `width` is content-driven with `padding 0 control-x`. Square buttons clip multi-digit page numbers and ignore `min-width` vs `width` distinction.
- [ ] Prev/next arrow buttons omit the `text_size`/`font` treatment and use raw icon sizing `.w(font_size).h(font_size)` for the chevron (lines 148-151, 207-211); fine, but they also skip `current`/hover tint logic the `make_button` helper applies — arrows can't show hover fill.
- [ ] No limit selector control: Svelte renders `<label>Show</label><select>…<span>per page</span>` (lines 248–262). Jetstream only shows a **static** page-size box inside `build_full_row` and only for the Full variant (lines 310–346) — Simple+numbered with `show_limit_selector` render nothing. **Add limit selector for all variants per contract §2.**
- [ ] No info-row "X to Y of Z" gating on `show_info` && total>0 only — `build_info_text` returns None when total is 0 (line 242) which is correct, but the info row is always stacked above controls regardless of `standalone`/chrome; no chrome/standalone handling at all (no border/background/padding branch). **Add chrome/standalone root treatment.**
- [ ] Density ignored: `control_space_x_rem(spec.density)` feeds padding (line 21) but controls/pages gap is fixed `space.inline.xs` (lines 48, 70, 164) regardless of `spec.density`. Contract density table varies the **gap**, not padding. **Drive gap from density, not pad_x.**
- [ ] Size font wired but button height uses `control_height_rem(effective_size)` directly (line 19) with no `−0.125rem` — matches Svelte (good) but min-width square-button issue above still applies.
- accepted: no ARIA channel (nav landmark / aria-current) — documented platform limit.
- accepted: click/interaction lives in preview event loop, not the component (no `on_page_change` analog passed to `js_pagination`).

## Specimen parity

- Svelte covers: Default, Middle of range, Few pages, Simple+info+pageSize, Full variant, With container chrome, Sizes (snippet), Densities (snippet) — `PaginationSpecimen.svelte`.
- GPUI covers: Default, Middle of range, Few pages, Simple+info+pageSize, Full+limit (interactive), Standalone, Sizes + Densities (via `specimen_layout`). — missing: a plain **chrome** demo (only standalone shown); first/last buttons never visible (no controller). Broader than Svelte on interactivity.
- Jetstream covers: First page, Middle page, Last page only (`jetstream/.../pagination.rs`). — missing: **Simple variant**, **Full variant**, **limit selector**, **info row**, **sizes**, **densities**, **compact**, **loading**, **chrome/standalone**. Heavily under-covers.

## Notes

- Biggest structural divergence: both Rust impls invented a "Go to page" input/box for the Full variant that does **not** exist in Svelte. The Svelte full variant is prev / "Page X of Y" / next / (first/last when controller has goToPage). Both Rust impls also mis-assign which variant shows the "Page X of Y" summary vs the item-range summary.
- GPUI `rem_to_px(2.25)` for min-width is a hardcode of an outdated contract value (`2.25rem`); current contract/Svelte min-width is `control-height`. Route through `size.control.height`.
- Color-mix tints (current fill 18%, hover 12%, current border 42%) are correctly replicated in both impls via `tint()` / `color_mix()` / alpha-scaling — no color literals found. Good.
- `consv=fixed`: the stale §8 token tables (chrome/standalone model, button height −0.125rem, density gaps, compact padding, size min-widths, first/last glyphs) described an earlier Svelte revision and are now reconciled to current Svelte per "Svelte is parity authority". The Rust-invented "Go to page" field is a code gap (GPUI/Jetstream), not a contract divergence — the contract never specified it.
