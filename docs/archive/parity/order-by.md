<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok -->
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

- [x] FIXED — Item background now resolves `color_mix(surface 90%, elevated)` via `theme_ext::color_mix`, not raw HSLA alpha-math.
- [x] FIXED — move-up/move-down IconButtons removed. Item now has the drag-handle glyph + 2 action buttons (direction toggle + remove); Alt-arrow keyboard reorder is preview-loop (note).
- [x] FIXED — Item restructured to a single inline flex row (drag handle + flex-1 ellipsis label + direction-toggle + remove); the stacked `flex_col` + "Ascending/Descending" direction line is gone.
- [x] FIXED — Empty text now `"No sort fields"`.
- [x] FIXED — footer "Clear all" Button removed (clearing is the reset `×` IconButton only).
- [x] FIXED — Reset is now an `IconButton icon="x" variant=Ghost` (aria "Clear sort") at the resolved size. Drag handle is a muted glyph; the focusable-`<button>` drag affordance is approximated (note: drag/keyboard reorder is preview-loop).
- [x] FIXED — Direction-toggle/remove IconButtons now `ControlSize::Xs`, ghost, with per-field aria (`"{field}: ascending. Click to toggle."`, `"Remove {field}"`) + tooltips `Asc`/`Desc`/`Remove`; remove no longer sets danger tone.
- [x] FIXED — Sizes corrected to the contract table (xs `1.5`/sm `1.75`/md control-height/lg `2.75`/xl `3.25`), with per-size label/summary fonts and xs/lg/xl trigger padding.
- [x] FIXED — Item radius now `radius.control` via `spec.radius_token()`; surface radius via `spec.surface_radius_token()`; item border via `spec.item_border_token()`; muted summary/handle via `spec.muted_color_token()`. The add-field box is now a real `Select` (placeholder "+ Add field", aria "Add sort field").
- [ ] Menu open + add-field selection, drag reorder, and Alt+Arrow keyboard reorder are not wired in the component — preview event-loop work, render-only build-verified posture.
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded`/roles not emitted.
- accepted: anchored-dropdown positioning is platform-owned; panel rendered inline below trigger.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED — Item background now `color_mix(surface 90%, elevated)`, not a `tint()` heuristic.
- [x] FIXED — move-up/move-down IconButtons removed; item now has the drag-handle glyph (a focusable `button`) + direction toggle + remove.
- [x] FIXED — Item restructured to a single inline flex row (handle + flex-1 ellipsis label + 2 IconButtons); the stacked `flex_col` + "Ascending/Descending" line is gone.
- [x] FIXED — Empty text now `"No sort fields"`.
- [x] FIXED — footer "Clear all" `js_button` removed.
- [x] FIXED — Reset is a ghost `js_icon_button` (icon "x", aria "Clear sort"). Drag handle is now a focusable `button("⠿")` (drag/Alt-arrow reorder is preview-loop).
- [x] FIXED — Direction-toggle/remove IconButtons now `ControlSize::Xs`, ghost, with per-field aria + `Asc`/`Desc`/`Remove` tooltips; remove no longer sets `tone=Danger`.
- [x] FIXED — Sizes corrected to the contract table (xs `1.5`/sm `1.75`/md control-height/lg `2.75`/xl `3.25`) with per-size fonts/padding.
- [x] FIXED — chevron sized from the summary font; item radius via `spec.radius_token()`, surface radius via `spec.surface_radius_token()`, item border via `spec.item_border_token()`, muted via `spec.muted_color_token()`. Add-field is now a real `js_select` (placeholder "+ Add field", aria via `SelectSpec.aria_label`).
- [ ] Menu open + add-field selection, drag reorder, and Alt+Arrow keyboard reorder are not wired in the component — preview event-loop work, render-only build/probe-verified.
- accepted: no ARIA channel (documented pattern); interaction would live in preview event loop, but no add/remove/toggle/drag wiring exists there either.

Probe tests (`order_by.rs` `#[cfg(test)] mod tests`): empty-open shows "No sort fields" + hidden reset; populated item shows label + `arrow-up`/`arrow-down` direction icon + `⠿` handle, no "Ascending"/"Descending" text, no "Clear all"; summary reads `"Title ↑"`.

## Specimen parity

- Svelte covers: Multi-field sort builder (compact, with live JSON), Disabled, Sizes snippet (xs–xl), Densities snippet (compact/default/comfortable). (`OrderBySpecimen.svelte`)
- GPUI covers: Sort controls (open, 2 active fields), Disabled (open), Sizes, Densities — broad. — missing: a `compact` example; uses different field set ("Name/Date/Size/Type") than Svelte's ("Title/Kind/Updated/Created/Visibility"), so compact-summary `+N` truncation is never shown.
- Jetstream covers: Multi-field sort builder (compact, open, Svelte field set Title/Kind/Updated/Created/Visibility), Empty (open), Disabled, Sizes (xs–xl), Densities (compact/default/comfortable) — now broad, parity with the Svelte/GPUI specimen set.

## Notes

- **Fix pass (both Rust targets reconverged to corrected contract):** both impls rewritten to the single-row item model — drag-handle glyph + flex-1 ellipsis label + xs ghost direction-toggle + xs ghost remove; ghost `×` reset IconButton; "No sort fields" empty text; `color_mix(surface 90%, elevated)` item bg; `radius.control` item radius + `radius.surface` surface; corrected size table; real `Select` (GPUI `Select`, Jetstream `js_select`) for add-field. Removed move-up/move-down buttons and the footer "Clear all". Additive `OrderBySpec` token methods: `muted_color_token()` (→ `color.text.placeholder`), `item_border_token()` (→ `border.subtle`), `surface_radius_token()` (→ `radius.surface`). Jetstream `#[cfg(test)] mod tests` added (render_probe). Token gap noted: there is no `color.text.muted` constant in the Rust token set, so the Svelte `--poodle-color-text-muted` (summary placeholder + drag handle) maps to `color.text.placeholder`. Preview-loop (not closed): menu open, add-field selection, drag reorder, Alt+Arrow keyboard reorder.

- `consv=fixed`: the contract was badly out of date vs authoritative Svelte and is now fully reconciled — move-up/move-down buttons, footer Clear-all, stacked item with direction text, bespoke square reset, and wrong size heights all removed/corrected to match `OrderBy.svelte`. The popover wrapper, dialog surface, drag-handle button, Alt-arrow keyboard reorder, `showClearButton` prop, xs-size IconButtons, and corrected item/drop-target tokens are now documented. Both Rust impls still replicate the OLD contract (move buttons, footer, stacked item, bespoke reset, wrong sizes) — they now need to re-converge against the corrected contract; that is code work, tracked in the GPUI/Jetstream sections, out of scope for this contract reconciliation.
- `rem_to_px(<contract-rem-value>)` is the sanctioned resolution path and is **not** flagged as a token violation. Flags above are genuine: raw `Hsla{}` alpha-math (GPUI), `tint()` factor (Jetstream), unused spec token methods, and literal `"radius.surface"` strings bypassing `spec.radius_token()`-style accessors.
- Neither Rust impl is interactive; this is the expected build-verified-only posture for the Rust previews, but the drag handle / add-field Select being non-functional `div`/`label` placeholders (no Select component instance — just a `"+ Add field"` text box) is a real anatomy gap vs Svelte's real `Select`.
