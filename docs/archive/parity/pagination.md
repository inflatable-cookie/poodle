<!-- parity consv=fixed gpui=0 jetstream=1 specimen=ok -->
<!-- pass 42: Jetstream specimen backfilled to full contract §13 + state coverage — Default, Middle of range, Few pages, first/last boundary-disabled, Simple+info+limit selector, Full variant, chrome, standalone, compact, loading, Size ladder, Density ladder — via real js_pagination + PaginationSpec builders. GPUI gained "With container chrome" + "Last page (next disabled)" groups. Both previews build clean. -->
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

- [x] FIXED Button `min-width` now resolves to control-height (per-size, no −0.125rem) via a `button_min_width` field, replacing the `rem_to_px(2.25)` hardcodes.
- [x] FIXED (by removal) Ellipsis `min-width` `rem_to_px(1.5)` is a contract-exact constant (`1.5rem` §8) — accepted, left as-is.
- [x] FIXED (by removal) Go-to field width hardcode removed with the go-to field.
- [x] FIXED Removed the Go-to input field — not in the Svelte/contract full variant. Full now renders first / prev / "Page X of Y" / next / last. (Inert `goto_page_input`/`on_goto_input_change` builders retained so the preview crate still compiles; they render nothing.)
- [x] FIXED Full variant center now shows "Page X of Y" (`spec.full_summary()`), not the numbered page row.
- [x] FIXED Simple variant summary now shows the item range "X–Y of Z" (`spec.simple_summary()`).
- [x] FIXED Info row now computes "Showing X to Y of Z" via `spec.info_string()` (hidden when total is 0/unknown).
- [x] FIXED First/last (`««`/`»»`) buttons now render for the full variant (gated on `on_page_change` being wired — GPUI's goToPage analog).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but no nav landmark / `aria-current` emitted.
- accepted: scroll targeting omitted (contract Known Delta, web-only).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED (by removal) Go-to box width hardcode removed with the go-to row.
- [x] FIXED Removed the invented "Go to page:" row (`build_full_row`) — not in Svelte. Full variant now renders first / prev / "Page X of Y" / next / last.
- [x] FIXED Full variant now renders the "Page X of Y" center summary (`spec.full_summary()`).
- [x] FIXED First/last (`««`/`»»`) buttons now render for the full variant.
- [x] FIXED Page/nav buttons now use `min_w(control-height)` + `h(control-height)` + `px(control-x)` (content-driven width), not fixed squares.
- [ ] Prev/next chevron arrow buttons are icon-only and don't apply the `current`/hover tint that page buttons use — but Jetstream renders statically (hover is preview-loop), so there is no hover state to show in the component. Minor accepted approximation.
- [x] FIXED Limit selector now renders for ALL variants ("Show [n ▾] per page") via `build_limit_selector`, not just the full variant's go-to box.
- [x] FIXED Info row now uses `spec.info_string()` (hidden when total 0/unknown) and a chrome/standalone root branch was added (padding + top border + elevated-92% bg when `!standalone`).
- [x] FIXED Controls/pages gap is now density-driven (`density_gap_px`: compact 3px, default 0.25rem, comfortable 0.375rem), not fed from pad_x.
- [x] FIXED Button height uses `control_height_rem` (no −0.125rem, matches Svelte) and the square-button min-width issue is resolved above.
- accepted: no ARIA channel (nav landmark / aria-current) — documented platform limit.
- accepted: click/interaction lives in preview event loop, not the component (no `on_page_change` analog passed to `js_pagination`).

## Specimen parity

- Svelte covers: Default, Middle of range, Few pages, Simple+info+pageSize, Full variant, With container chrome, Sizes (snippet), Densities (snippet) — `PaginationSpecimen.svelte`.
- GPUI covers: Default, Middle of range, Few pages, Simple+info+pageSize, Full+limit (interactive), Standalone, With container chrome, Last page (next disabled), Sizes + Densities (via `specimen_layout`). Added the plain chrome demo + an explicit last-page boundary group this pass. Broader than Svelte on interactivity. `specimen=ok`.
- Jetstream now covers: Default, Middle of range, Few pages, First-page (prev disabled), Last-page (next disabled), Simple+info+limit selector, Full variant, With container chrome, Standalone, Compact, Loading, Size ladder (xs..xl), Density ladder (`jetstream/.../pagination.rs`) — all via real `js_pagination` + `PaginationSpec` builders. No fakes. `specimen=ok`. (First/last guillemet buttons render in the Full variant; navigation itself is a preview-loop concern.)

## Notes

- Biggest structural divergence: both Rust impls invented a "Go to page" input/box for the Full variant that does **not** exist in Svelte. The Svelte full variant is prev / "Page X of Y" / next / (first/last when controller has goToPage). Both Rust impls also mis-assign which variant shows the "Page X of Y" summary vs the item-range summary.
- GPUI `rem_to_px(2.25)` for min-width is a hardcode of an outdated contract value (`2.25rem`); current contract/Svelte min-width is `control-height`. Route through `size.control.height`.
- Color-mix tints (current fill 18%, hover 12%, current border 42%) are correctly replicated in both impls via `tint()` / `color_mix()` / alpha-scaling — no color literals found. Good.
- `consv=fixed`: the stale §8 token tables (chrome/standalone model, button height −0.125rem, density gaps, compact padding, size min-widths, first/last glyphs) described an earlier Svelte revision and are now reconciled to current Svelte per "Svelte is parity authority". The Rust-invented "Go to page" field is a code gap (GPUI/Jetstream), not a contract divergence — the contract never specified it.
