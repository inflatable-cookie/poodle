<!-- parity consv=fixed gpui=2 jetstream=2 specimen=gap -->
<!-- pass 52: fullWidth + vertical orientation built on BOTH targets. Added
     TabsSpec.is_full_width (+ with_full_width, uses_full_width(), is_vertical()).
     fullWidth (horizontal): list flex w_full, tabs flex_grow + w_full + justify_center
     (underline/card; block already flexes under fullWidth — content-sized otherwise per
     Svelte flex:0 0 auto). Vertical: flex_col, border shifts to inline-end (right) edge,
     icon-only label (contract §8 label display:none), block separator → top border.
     build_tab_label gained icon_only param. Jetstream caption-size badge now from
     typography.caption.size token. 4 new probe tests (jet: fullWidth equal-width,
     vertical icon-only, vertical block column); gpui clean. REMAINING (accepted/deferred):
     strip variant = separate tab_strip.rs (architecture delta); badge geometry rems
     (radius 0.5625 / min-w 1.125 / px 0.3125) = token gap, no dedicated token;
     separator/actions/collapse/historyKey/tooltips = host-snippet/preview-loop;
     close/reorder/keyboard = preview-loop. -->
# Parity: Tabs

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/tabs.md`
- Svelte (authoritative): `packages/svelte/components/src/Tabs.svelte`
- GPUI: `packages/gpui/components/src/primitives/tabs.rs` (+ separate `primitives/tab_strip.rs` for the `strip` variant)
- Jetstream: `packages/jetstream/components/src/tabs.rs` (+ separate `tab_strip.rs` for the `strip` variant)
- Specimens: svelte `packages/svelte/preview/src/specimens/TabsSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/tabs.rs` · jetstream `packages/jetstream/preview/src/specimens/tabs.rs`

## Contract ↔ Svelte

Svelte carries props the contract omits. Svelte authoritative — contract fixed.

- [x] FIXED Svelte adds `collapseWhenOverflow?: boolean` (`Tabs.svelte:46`) → collapses the tablist into a `Menu` when it overflows. Added to contract §3 props, §2 anatomy (Collapsed Menu part), §1 in-scope (removed "overflow menus" from out-of-scope), and Svelte Notes.
- [x] FIXED Svelte adds `fullWidth?: boolean` (`Tabs.svelte:47`) → tabs flex to fill the row (`data-full-width`). Added to contract §3 + §8 full-width token table + Svelte Notes.
- [x] FIXED Svelte adds `collapseLabel?: string | null` (`Tabs.svelte:48`). Added to contract §3 (falls back to active tab label).
- [x] FIXED Variant naming: Svelte normalizes `variant="underline"` → `"text"` (`resolvedVariant`, `Tabs.svelte:146`); rendered `data-variant="text"`; default `"text"`. Contract §3 already matches Svelte exactly. Added a GPUI-notes clarification that the Rust `TabVariant::Underline` enum member is the same variant (implementation-side naming only). No §3 change needed — Svelte side already authoritative.
- Contract §3 omits `block` from prop default examples but documents it in token tables; Svelte/Rust both implement `block`. Consistent — no fix.
- [x] FIXED Tooltip behavior (`showTooltips`) documented in §3 but missing from §2 anatomy. Added Tooltip anatomy part + Svelte Notes describing tooltip wrapping and vertical/icon-only label surfacing.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded badge geometry literals `min_w(px(rem_to_px(1.125)))`, `px(rem_to_px(0.3125))`, `rounded(px(rem_to_px(0.5625)))` at `tabs.rs:65-67` — resolve from Pill spec tokens, not raw rems.
- [x] FIXED Card-row gap now resolves from `list_gap_token()` (`space.inline.sm`) instead of inline `0.125rem`. Pill gap `0.125`, pill padding `0.1875`, pill tab-height delta `0.5` are contract-exact rems (accepted — `rem_to_px(contract-exact)` is not a hardcode violation).
- [x] FIXED Removed the `color.background.elevated` hover background from the underline and card variants — Svelte text/card variants have NO hover bg (only block/strip do, which keep their token-resolved hover).
- [x] FIXED Density→padding match arms (underline/block/pill) now resolve via `control_space_x_rem(density)` instead of inline `0.5/0.75/1.0` floats.
- [x] FIXED Removed the `border_b_2()` accent underline from the active text/underline tab — the indicator is now the pill-shaped bg tint only, matching Svelte.
- [x] FIXED `fullWidth` — `uses_full_width()` (horizontal only) makes the underline/card list `w_full` and each tab `flex_grow().w_full().justify_center()`; block tabs flex to equal shares under fullWidth (content-sized `flex:0 0 auto` otherwise, matching Svelte). Contract §8 Full-width table.
- [x] FIXED vertical orientation — `is_vertical()` drives `flex_col` lists with the rule on the inline-end (right) edge, icon-only tabs (label/close hidden per contract §8), and block separators on the top border. `build_tab_label` icon_only param.
- [x] FIXED count badge size now from `typography.caption.size` (already resolved in GPUI's `build_tab_label`). Badge background still `color_mix(text,surface,0.14)` — see badge-geometry note below.
- [ ] No close button interaction — card close icon rendered but no `onClose` callback wired; Delete key unhandled. **preview-loop** (interaction lives in the host event loop, not the component).
- [ ] No reorder (drag-and-drop or Alt+Arrow). **preview-loop**.
- accepted: `strip` variant handled by separate `tab_strip.rs`/`TabStripItem` (architecture delta — contract treats strip as a Tabs variant; audit under a `tab-strip` pass).
- accepted: badge geometry rems (radius `0.5625rem`, `min-w 1.125rem`, `px 0.3125rem`) — **token gap**: no dedicated badge token exists; `radius.pill` is a full 999px pill, not the 9px badge radius. Contract-exact rems until a badge token lands.
- accepted: `separator`/`actions`/`collapseWhenOverflow`/`historyKey`/tooltips — host-snippet / overflow-measurement / URL-sync features, not component visual structure.
- [ ] Arrow-key nav wraps unconditionally and ignores disabled tabs — Svelte skips disabled via `findNextEnabledIndex`; no Home/End handling. **preview-loop** (keyboard handling).
- accepted: no ARIA (gpui has no accessibility API) — `role=tablist/tab/tabpanel`, `aria-selected`, `aria-controls` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded badge literals `rem_to_px(0.6875)`, `rem_to_px(0.5625)`, `rem_to_px(0.3125)`, `min_w(rem_to_px(1.125))` at `tabs.rs:70-80` — resolve from Pill/caption tokens.
- [x] FIXED Label icon↔label gap now resolves from `space.inline.sm` (was inline `0.25`). Remaining inline rems (pill gap `0.125`, pill padding `0.1875`, pill height delta `0.5`) are contract-exact rems (accepted). Badge geometry rems remain a token gap — see note.
- [x] FIXED Replaced the `control_height_rem * 0.25` `pad_y` heuristic with the Svelte model: zero vertical padding + `min_h(control-height - 0.25rem)` on underline/card tabs.
- [x] FIXED All tabs now render at `font-weight: 600` (underline/card/block); weight no longer changes on selection.
- [x] FIXED Removed the accent `border_b_2().border_color_bottom()` from the active underline tab — pill-shaped bg tint only, matching Svelte.
- [x] FIXED Close `x` button now rendered for closable card tabs (1.25rem square, `text-secondary` icon, `radius-control − 0.125rem`, `margin-right 0.25rem`). `onClose`/Delete interaction is preview-loop.
- [x] FIXED `fullWidth` — `uses_full_width()` (horizontal only) sets the underline/card list `w_full` and each tab `flex_grow().w_full().justify_center()`. Probe-tested: 3 tabs each ≈ container/3. Block flexes equally already.
- [x] FIXED vertical orientation — `is_vertical()` renders `flex_col` lists with the rule on the right edge (`border_r_1`), icon-only tabs (label hidden via `build_tab_label` icon_only), block separators on the top border. Probe-tested: vertical underline is icon-only; vertical block stacks into a column.
- [x] FIXED count-badge font now from `typography.caption.size` token (was `rem_to_px(0.6875)` literal).
- [ ] No reorder; interaction (click/keyboard) lives in the preview `main.rs` event loop. **preview-loop**.
- accepted: `strip` variant handled by separate `tab_strip.rs` (architecture delta).
- accepted: `separator`/`actions`/`collapseWhenOverflow`/`historyKey`/tooltips — host-snippet/overflow/URL features, not component visual structure.
- accepted: badge geometry rems (radius `0.5625`, `min-w 1.125`, `px 0.3125`) — **token gap**, no dedicated badge token (`radius.pill` is a 999px full pill).
- [ ] No panel rendering — `js_tabs` returns the tab bar only; content is the caller's responsibility, so `role=tabpanel`/`aria-labelledby` linkage is absent. accepted (panel is host-owned).
- [x] FIXED Removed the fragile card active-tab bottom-border reset workaround — card items now use a uniform border + `radius-control` on all sides, with selected recoloring border/bg only (matches Svelte, which keeps a uniform card border).
- [ ] Disabled tabs not skipped in nav; Home/End/Delete unhandled. **preview-loop** (lives in main.rs).
- accepted: no ARIA channel; interaction in preview event loop, not the component.

## Specimen parity

- Svelte covers: Text+panel, Card (closable+reorderable), Block (full-width+separators), Pill (icons), Underline (icons, no panel), Strip (horizontal full-width closable reorderable), Strip vertical (icon-only), Strip collapse-toggle, Card with counts+separators+URL sync (`TabsSpecimen.svelte`).
- GPUI covers: Underline, Card, Card+counts(icons), Pill, Block, Underline+icons, Strip (via TabStrip), Strip vertical, collapse-toggle. — missing: reorder demo parity in Tabs proper, separator demo, fullWidth, historyKey.
- Jetstream covers: Underline, Card, Pill, Block, icons (Underline/Pill), count badges, disabled. — missing: **closable/close button**, **reorder**, **strip variant** (in this specimen), **vertical orientation**, **panel content**, **separators**, **counts+URL sync**.

## Notes

- Strip variant is split into a dedicated `tab_strip.rs` component in both Rust targets, diverging from Svelte where strip is a `Tabs` variant. This is an architecture delta worth a contract note; treat Strip parity under a separate `tab-strip` audit if one exists.
- The former `consv=gap` driver (undocumented Svelte surface `collapseWhenOverflow`/`fullWidth`/`collapseLabel` plus tooltip anatomy missing from §2) is now resolved in the contract — `consv=fixed`. Remaining items are all Rust-impl gaps, not contract↔Svelte.
- Biggest single gap: neither Rust target wires close/reorder/vertical, and both contradict the contract `font-weight: 600` (Jetstream) / no-tab-bottom-border (both) rules.
