<!-- parity consv=gap gpui=8 jetstream=10 specimen=gap -->
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

- `chrome` vs `standalone` semantics inverted. Contract §3 lists `chrome` (default `false`, opt-in panel chrome) and `standalone` as a **deprecated inverse alias**. Svelte applies chrome class as `standalone !== undefined ? !standalone : chrome` (line 232) — chrome OFF by default, matching contract. But the contract's token table §8 names the modifier `.pagination--standalone` for the *strip-chrome* case and `.pagination` (base) as having border/background. The Svelte base `.poodle-pagination` has **no** chrome (lines 346–354, `padding:0`); chrome lives in `.poodle-pagination--chrome` (356–360). **Fix: contract §8 token tables describe the OLD standalone model; rewrite §8 root tables to match Svelte's `--chrome` opt-in.**
- Button `height`: contract §8 says `calc(control-height − 0.125rem)`; Svelte base button uses `height: var(--poodle-size-control-height)` (line 427) with **no** −0.125rem. **Fix: reconcile — either contract drops the −0.125rem or Svelte adds it. Svelte is authoritative → drop −0.125rem from contract §8.**
- Button `min-width`: contract §8 / §7 says `var(--poodle-size-control-height)`; Svelte uses the same (line 426). OK. But the GPUI/Jetstream impls hardcode `2.25rem` — noted below, not a contract↔Svelte issue.
- Density gap values: contract §8 density table says `compact=0.0625rem`, `comfortable=0.25rem`. Svelte uses `compact gap:3px` (line 483), `default gap:0.25rem` (488), `comfortable gap:0.375rem` (499). **Three mismatches.** Svelte authoritative → **fix contract density table to 3px / 0.25rem / 0.375rem.**
- Size table: contract §8 lists `md` min-width `control-height`, `sm` min-width `control-height − 0.375rem` etc. Svelte hardcodes `xs=1.5rem`, `sm=1.75rem`, `lg=2.75rem`, `xl=3.25rem` (lines 476–479) and omits an `md` override (inherits base). **Fix: contract size table is expressed as calc-offsets; Svelte uses flat rem. Reconcile to Svelte's literal values.**
- First/last button glyph: contract §8 variant table says `"<<"` / `">>"`; Svelte renders `««` / `»»` (double guillemets, lines 275/336). **Fix: contract glyph column to `««`/`»»`.**
- First/last buttons gate on `variant === "full" && supportsGoToPage` (lines 267, 328). `supportsGoToPage` requires `controller.goToPage` (line 108) — so first/last NEVER appear from plain props, only with a controller. Contract §2/§3 agree. OK, but note: the specimen "Full variant" passes no controller, so first/last are never demonstrated.
- `compact` padding: contract §8 compact table says `padding: 0.5rem 0.75rem`, `gap: 0.75rem`. Svelte `--compact` sets `padding:0; gap:0.75rem` (lines 362–365). **Padding mismatch (0 vs 0.5rem 0.75rem).** Svelte authoritative → **fix contract compact padding to `0`.**
- `limitOptions` default: contract §3 default `[30, 50, 100]`; Svelte default `[30, 50, 100]` (line 62). OK.

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
- `consv=gap` driver: stale §8 token tables (chrome/standalone model, button height −0.125rem, density gaps, compact padding, size min-widths, first/last glyphs) all describe an earlier Svelte revision. All belong updated to current Svelte per "Svelte is parity authority".
