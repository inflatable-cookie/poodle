<!-- parity consv=fixed gpui=10 jetstream=11 specimen=gap -->
# Parity: OrderBy

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/order-by.md`
- Svelte (authoritative): `packages/svelte/components/src/OrderBy.svelte`
- GPUI: `packages/gpui/components/src/primitives/order_by.rs`
- Jetstream: `packages/jetstream/components/src/order_by.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/OrderBySpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/order_by_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/order_by.rs`

## Contract ↔ Svelte

Contract and Svelte diverge heavily. Svelte is authoritative — the contract is stale in many places. Both Rust impls follow the *contract*, so contract errors propagate downstream.

- [x] FIXED — **Item actions: contract had 4 IconButtons (toggle, move-up, move-down, remove); Svelte has 2 (toggle + remove).** Reordering is via the drag handle + Alt+ArrowUp/ArrowDown (`OrderBy.svelte:358-367`). §2 anatomy, §6 ARIA, §10, §11 sub-components, §13 checklist all rewritten to drop move-up/move-down and document the drag-handle + Alt-arrow reorder.
- [x] FIXED — **Drag handle is a focusable `<button>`, not a `<span aria-hidden>`.** §2 anatomy + §6 (drag handle = focusable button, `aria-label="Reorder {field}. Drag or use Alt plus arrow keys."`) + §11.
- [x] FIXED — **Reset is an `IconButton icon="x" variant="ghost"`, not a `1.75rem` square button.** §2/§7/§8 rewritten: reset is an IconButton (own hover/focus via the primitive); the bespoke reset token tables removed.
- [x] FIXED — **No footer / no "Clear all" Button.** Footer + Clear-all removed from §2 anatomy, §4 states (dropped "single field active" row), §11 sub-components, §13 checklist. Clearing is the reset `×` only.
- [x] FIXED — **Item is single-row, label single-line; no stacked direction text.** §2/§7/§8 rewritten: item is a single flex row (handle + label + 2 IconButtons); Item Main / Item Direction parts removed; item label is `flex:1` ellipsis at `0.8125rem`.
- [x] FIXED — **Empty text "No sort fields".** §2/§4 updated from "No sort fields selected" to "No sort fields".
- [x] FIXED — **Size table.** §8 size table reset to Svelte: xs `1.5`, sm `1.75`, md control-height, lg `2.75`, xl `3.25`; label fonts xs `0.5625`/sm `0.625`/md `0.75`/lg `0.8125`/xl `0.875`; summary fonts added; xs/lg/xl padding overrides.
- [x] FIXED — **`showClearButton?: boolean` (default true)** added to §3 props (and to §2 anatomy reset condition + §13 checklist).
- [x] FIXED — **Direction-toggle/remove IconButtons are `size="xs"`, tooltip `"Asc"`/`"Desc"`, remove has no danger tone.** §6/§7/§11 updated (was `size="sm"`, tooltip "Ascending"/"Descending", remove `tone="danger"`).
- [x] FIXED — **Item visual tokens.** §8 item table now border-subtle, radius `calc(radius-control − 0.0625rem)`, bg `surface 90%/elevated`; drop-target now accent 8% fill + left accent bar (`inset 0.125rem 0 0`); item hover added.
- [x] FIXED — **Popover wrapper + dialog surface.** §2 adds the `.order-by-popover` positioning layer; §8 adds the surface table (`role="dialog"`, `tabindex=-1`, `min-width:14rem`, `top: calc(100% + 0.5rem)`); §12 documents auto-focus-first + outside-click/Escape close.
- [x] FIXED — **Trigger `aria-expanded`/`aria-controls`** added to §6 ARIA. `onChange` signature already matched (no change).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. GPUI mirrors the (stale) contract, so it also carries contract↔Svelte drift. Mark accepted runtime limits.

- [ ] Item background hardcoded via raw `Hsla{ a: surface.a*0.88 + elevated.a*0.12, ..surface }` alpha-math at `order_by.rs:107-110` — resolve a real fill token, not a hand-rolled HSLA mix.
- [ ] Renders move-up + move-down IconButtons (`order_by.rs:209-224`) that Svelte does not have; missing the draggable drag-handle button + Alt-arrow keyboard reorder. Align to Svelte: 2 action buttons + interactive handle.
- [ ] Item laid out as stacked `flex_col` with a separate "Ascending"/"Descending" direction text line (`order_by.rs:246-269`) — Svelte item is a single inline row with no direction text. Restructure.
- [ ] Empty text `"No sort fields selected"` (`order_by.rs:185`) — Svelte says `"No sort fields"`.
- [ ] Renders a footer "Clear all" `Button` at `active_count >= 2` (`order_by.rs:300-310`) — Svelte has no footer/Clear-all.
- [ ] Reset is a bespoke `1.75rem` square `div` with literal `×` (`order_by.rs:156-166`) — Svelte uses an `IconButton icon="x"` ghost. Drag-handle `⠿` is a plain `div`, not interactive (`order_by.rs:256`).
- [ ] Direction-toggle/remove IconButtons use `ControlSize::Sm` (`order_by.rs:206,229`) — Svelte uses `xs`; aria/tooltip strings are generic ("Toggle direction", "Remove field") not the per-field Svelte strings.
- [ ] Sizes hardcoded xs `1.625`/sm `1.75`/lg `2.25`/xl `2.5` (`order_by.rs:80-86`) — Svelte is xs `1.5`/lg `2.75`/xl `3.25`. (Driven by contract; fix once contract corrected.)
- [ ] Spec exposes `gap_token()`, `active_fill/border/text_token()`, `reset_color_token()` — all unused; item radius uses literal `"radius.surface"` string (`order_by.rs:242,314`) and gaps/paddings are raw `rem_to_px` literals where spec token methods exist. Resolve via spec methods.
- [ ] No interactivity beyond `on_reset`: add/remove/toggle/move/drag/clear callbacks absent — render-only.
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded`/roles not emitted.
- accepted: anchored-dropdown positioning is platform-owned; panel rendered inline below trigger.

## Jetstream gap (vs Svelte + contract)

- [ ] Item background via `tint(elevated, 0.88)` heuristic (`order_by.rs:172`) — resolve a real fill token, not an ad-hoc tint factor.
- [ ] Renders move-up + move-down IconButtons (`order_by.rs:134-149`) absent from Svelte; missing the draggable handle + Alt-arrow keyboard reorder.
- [ ] Item is stacked `flex_col` with a separate "Ascending"/"Descending" direction line (`order_by.rs:174-199`) — Svelte item is a single inline row, no direction text. Restructure.
- [ ] Empty text `"No sort fields selected"` (`order_by.rs:108`) — Svelte says `"No sort fields"`.
- [ ] Footer "Clear all" `js_button` at `active_count >= 2` (`order_by.rs:232-240`) — Svelte has no footer/Clear-all.
- [ ] Reset is a plain `button("×")` square (`order_by.rs:91-98`) — Svelte uses `IconButton icon="x"` ghost. Drag handle `⠿` is a non-interactive `label` (`order_by.rs:183-186`).
- [ ] Direction-toggle/remove IconButtons use `ControlSize::Sm` (`order_by.rs:131,154`) — Svelte uses `xs`; generic aria/tooltip strings, not per-field Svelte strings; remove sets `tone=Danger` which Svelte does not.
- [ ] Sizes hardcoded xs `1.625`/sm `1.75`/lg `2.25`/xl `2.5` (`order_by.rs:23-29`) — Svelte is xs `1.5`/lg `2.75`/xl `3.25`. (Contract-driven.)
- [ ] Spec `gap_token()`, `active_*_token()`, `reset_color_token()` unused; chevron sized with literal `0.875` rem (`order_by.rs:80-81`); item radius via literal `"radius.surface"` (`order_by.rs:169,244`). Resolve via spec token methods.
- [ ] Trigger gap/pad and panel/list/item gaps are raw `rem_to_px` literals where `gap_token()` and size/density methods should drive them (`order_by.rs:30-43`).
- [ ] No interactivity in component (no callbacks at all) — render-only.
- accepted: no ARIA channel (documented pattern); interaction would live in preview event loop, but no add/remove/toggle/drag wiring exists there either.

## Specimen parity

- Svelte covers: Multi-field sort builder (compact, with live JSON), Disabled, Sizes snippet (xs–xl), Densities snippet (compact/default/comfortable). (`OrderBySpecimen.svelte`)
- GPUI covers: Sort controls (open, 2 active fields), Disabled (open), Sizes, Densities — broad. — missing: a `compact` example; uses different field set ("Name/Date/Size/Type") than Svelte's ("Title/Kind/Updated/Created/Visibility"), so compact-summary `+N` truncation is never shown.
- Jetstream covers: "No active sort" (empty/open), "Multi-field sort" (2 active/open). — missing: **Disabled**, **Sizes**, **Densities**, **compact** — only two static groups, no size/density coverage at all.

## Notes

- `consv=fixed`: the contract was badly out of date vs authoritative Svelte and is now fully reconciled — move-up/move-down buttons, footer Clear-all, stacked item with direction text, bespoke square reset, and wrong size heights all removed/corrected to match `OrderBy.svelte`. The popover wrapper, dialog surface, drag-handle button, Alt-arrow keyboard reorder, `showClearButton` prop, xs-size IconButtons, and corrected item/drop-target tokens are now documented. Both Rust impls still replicate the OLD contract (move buttons, footer, stacked item, bespoke reset, wrong sizes) — they now need to re-converge against the corrected contract; that is code work, tracked in the GPUI/Jetstream sections, out of scope for this contract reconciliation.
- `rem_to_px(<contract-rem-value>)` is the sanctioned resolution path and is **not** flagged as a token violation. Flags above are genuine: raw `Hsla{}` alpha-math (GPUI), `tint()` factor (Jetstream), unused spec token methods, and literal `"radius.surface"` strings bypassing `spec.radius_token()`-style accessors.
- Neither Rust impl is interactive; this is the expected build-verified-only posture for the Rust previews, but the drag handle / add-field Select being non-functional `div`/`label` placeholders (no Select component instance — just a `"+ Add field"` text box) is a real anatomy gap vs Svelte's real `Select`.
