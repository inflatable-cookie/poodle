<!-- parity consv=gap gpui=10 jetstream=11 specimen=gap -->
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

- **Item actions: contract specifies 4 IconButtons (direction toggle, move-up, move-down, remove); Svelte has only 2 (direction toggle + remove).** Svelte `OrderBy.svelte:372-389` renders only `arrow-up/arrow-down` toggle and `x` remove. Reordering in Svelte is via the drag handle + **Alt+ArrowUp/ArrowDown** keyboard (`OrderBy.svelte:358-367`), not move buttons. **Fix: rewrite contract §2 anatomy, §6 ARIA rows, §11 sub-components to drop move-up/move-down and document the drag-handle button + Alt-arrow keyboard reorder.**
- **Drag handle is a focusable `<button>`, not a `<span aria-hidden>`.** Svelte `OrderBy.svelte:347-370` makes the `⠿` handle the draggable button carrying `aria-label="Reorder {label}. Drag or use Alt plus arrow keys."` and the keydown handler. Contract §2/§6 say `<span aria-hidden="true">`. **Fix: contract — drag handle is the interactive button.**
- **Reset is an `IconButton icon="x" variant="ghost"`, not a `1.75rem` square `<button>`.** Svelte `OrderBy.svelte:313-323`. Contract §2/§7/§8 describe a hand-built square reset button with its own hover/focus token table. **Fix: contract — reset is IconButton, drop the bespoke reset token tables (§8 Reset blocks).**
- **No footer / no "Clear all" Button in Svelte.** Contract §2/§11/§4 ("footer shown at 2+ fields") and §13 ("clear-all resets value") describe a footer Button that Svelte does not render. Clearing happens only via the reset `×`. **Fix: remove Footer + Clear-all from contract anatomy/states/checklist.**
- **Item is single-row, label is single-line; no stacked direction text.** Svelte `OrderBy.svelte:341-390`: item is `display:flex; align-items:center` with handle + label + 2 icon buttons inline. Contract §2/§8 describe `Item Main` as a `flex-direction:column` stack with an "Item Direction" text line ("Ascending"/"Descending"). Svelte has no direction text line. **Fix: contract — item is a single flex row, drop Item Main / Item Direction parts.**
- **Empty text is "No sort fields", not "No sort fields selected".** Svelte `OrderBy.svelte:394`. Contract §2/§4 say "No sort fields selected". **Fix: contract string.**
- **Size table wrong.** Svelte heights (`OrderBy.svelte:618-659`) are xs `1.5rem`, sm `1.75rem`, md `control-height`, lg `2.75rem`, xl `3.25rem`. Contract §8 table says xs `1.625`, sm `1.75`, md `2`, lg `2.25`, xl `2.5`. Label font per size also differs (xs `0.5625rem` not `0.625rem`). **Fix: contract size table to match Svelte.**
- **`showClearButton?: boolean` (default true) prop exists in Svelte (`OrderBy.svelte:35,50,313`); absent from contract §3.** **Fix: add to contract props.**
- **Direction-toggle IconButton is `size="xs"` with tooltip `"Asc"`/`"Desc"`; remove is `size="xs"`.** Svelte `OrderBy.svelte:376,386`. Contract §6/§11/§7 say `size="sm"` and tooltip `"Ascending"`/`"Descending"`, and remove `tone="danger"` — Svelte remove has **no** `tone="danger"` (`OrderBy.svelte:381-389`). **Fix: contract — icon buttons size `xs`, tooltip Asc/Desc, remove has no danger tone.**
- **Item visual tokens differ.** Svelte item border is `border-subtle`, radius `calc(radius-control − 0.0625rem)`, drop-target is a left `inset box-shadow` accent bar + 8% accent fill (`OrderBy.svelte:553-573`). Contract §8 says border `border-default`, radius `radius-surface`, drop-target glow `0 0 0 …`. **Fix: contract item/drop-target token tables.**
- **Root wraps in an extra `.poodle-order-by-popover` positioning layer; panel is `role="dialog"` surface with `min-width:14rem`, `top: calc(100% + 0.5rem)`.** Contract §2 omits the popover wrapper and labels the surface only `aria-label`; Svelte adds `role="dialog"`, `tabindex=-1`, auto-focus-first, outside-click + Escape close (`OrderBy.svelte:250-275,327-335`). **Fix: document the dialog surface + dismissal behavior in contract.**
- **`onChange` signature: contract §3 says `(value) => void \| null`; matches Svelte. OK.** Trigger carries `aria-expanded`/`aria-controls` (`OrderBy.svelte:301-302`) not noted in contract §6. **Fix: add to contract ARIA.**

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

- `consv=gap` is dominated by the contract being badly out of date vs authoritative Svelte: the contract still describes move-up/move-down buttons, a footer Clear-all, a stacked item with direction text, a bespoke square reset, and wrong size heights — none of which match `OrderBy.svelte`. Both Rust impls were built to the contract, so they replicate every one of these deviations. Correct the contract first, then re-converge GPUI + Jetstream against it.
- `rem_to_px(<contract-rem-value>)` is the sanctioned resolution path and is **not** flagged as a token violation. Flags above are genuine: raw `Hsla{}` alpha-math (GPUI), `tint()` factor (Jetstream), unused spec token methods, and literal `"radius.surface"` strings bypassing `spec.radius_token()`-style accessors.
- Neither Rust impl is interactive; this is the expected build-verified-only posture for the Rust previews, but the drag handle / add-field Select being non-functional `div`/`label` placeholders (no Select component instance — just a `"+ Add field"` text box) is a real anatomy gap vs Svelte's real `Select`.
