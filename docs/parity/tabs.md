<!-- parity consv=fixed gpui=4 jetstream=4 specimen=gap -->
<!-- pass 51: variant/state cleanup both targets. GPUI: removed non-Svelte hover bg from
     text/card + accent bottom-border on active (pill tint only); density padding via
     control_space_x_rem; card-row gap from list_gap_token. Jetstream: all tabs weight 600
     (was per-selection); zero vertical pad + min_h(control-height − 0.25rem) (was *0.25 heuristic);
     close-x for closable card tabs; uniform card border + radius (dropped fragile reset); icon↔label
     gap from space.inline.sm. 6 probe tests; jet 231, gpui clean. REMAINING (real, deferred):
     strip variant (separate tab_strip.rs), vertical orientation, separator/actions/collapse/
     fullWidth/historyKey/tooltips host features. Activation/keyboard/reorder = preview-loop. -->
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
- [ ] No close button interaction — card close icon rendered (`tabs.rs:441-452`) but no `onClose` callback wired; Delete key unhandled.
- [ ] No reorder (drag-and-drop or Alt+Arrow) — `reorderable` absent from builder + spec usage.
- [ ] No `strip` variant in `Tabs` — handled by separate `tab_strip.rs`/`TabStripItem`; contract treats strip as a Tabs variant. Note split.
- [ ] No vertical orientation rendering — `orientation` builder exists (`tabs.rs:132`) but render fns ignore it; no icon-only collapse, no label/close hiding.
- [ ] No `separator`, `actions`, `count` token resolution (count badge bg is `color_mix(text,surface,0.14)` ad-hoc, not Pill tokens), `collapseWhenOverflow`, `fullWidth`, `historyKey`, tooltips.
- [ ] Arrow-key nav wraps unconditionally and ignores disabled tabs (`tabs.rs:276-292`) — Svelte skips disabled via `findNextEnabledIndex`; no Home/End handling.
- accepted: no ARIA (gpui has no accessibility API) — `role=tablist/tab/tabpanel`, `aria-selected`, `aria-controls` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded badge literals `rem_to_px(0.6875)`, `rem_to_px(0.5625)`, `rem_to_px(0.3125)`, `min_w(rem_to_px(1.125))` at `tabs.rs:70-80` — resolve from Pill/caption tokens.
- [x] FIXED Label icon↔label gap now resolves from `space.inline.sm` (was inline `0.25`). Remaining inline rems (pill gap `0.125`, pill padding `0.1875`, pill height delta `0.5`) are contract-exact rems (accepted). Badge geometry rems remain a token gap — see note.
- [x] FIXED Replaced the `control_height_rem * 0.25` `pad_y` heuristic with the Svelte model: zero vertical padding + `min_h(control-height - 0.25rem)` on underline/card tabs.
- [x] FIXED All tabs now render at `font-weight: 600` (underline/card/block); weight no longer changes on selection.
- [x] FIXED Removed the accent `border_b_2().border_color_bottom()` from the active underline tab — pill-shaped bg tint only, matching Svelte.
- [x] FIXED Close `x` button now rendered for closable card tabs (1.25rem square, `text-secondary` icon, `radius-control − 0.125rem`, `margin-right 0.25rem`). `onClose`/Delete interaction is preview-loop.
- [ ] No reorder; interaction (click/keyboard) lives in preview `main.rs` event loop — note if absent there.
- [ ] No `strip` variant in `js_tabs` — handled by separate `tab_strip.rs`. Contract treats strip as a Tabs variant.
- [ ] No vertical orientation, no `separator`, no `actions`, no `count` Pill tokens, no `collapseWhenOverflow`/`fullWidth`/`historyKey`/tooltips.
- [ ] No panel rendering — `js_tabs` returns tab bar only (`tabs.rs:394-407`); content is caller's responsibility, so `role=tabpanel`/`aria-labelledby` linkage absent.
- [x] FIXED Removed the fragile card active-tab bottom-border reset workaround — card items now use a uniform border + `radius-control` on all sides, with selected recoloring border/bg only (matches Svelte, which keeps a uniform card border).
- [ ] Disabled tabs not skipped in nav; Home/End/Delete unhandled (lives in main.rs).
- accepted: no ARIA channel; interaction in preview event loop, not the component.

## Specimen parity

- Svelte covers: Text+panel, Card (closable+reorderable), Block (full-width+separators), Pill (icons), Underline (icons, no panel), Strip (horizontal full-width closable reorderable), Strip vertical (icon-only), Strip collapse-toggle, Card with counts+separators+URL sync (`TabsSpecimen.svelte`).
- GPUI covers: Underline, Card, Card+counts(icons), Pill, Block, Underline+icons, Strip (via TabStrip), Strip vertical, collapse-toggle. — missing: reorder demo parity in Tabs proper, separator demo, fullWidth, historyKey.
- Jetstream covers: Underline, Card, Pill, Block, icons (Underline/Pill), count badges, disabled. — missing: **closable/close button**, **reorder**, **strip variant** (in this specimen), **vertical orientation**, **panel content**, **separators**, **counts+URL sync**.

## Notes

- Strip variant is split into a dedicated `tab_strip.rs` component in both Rust targets, diverging from Svelte where strip is a `Tabs` variant. This is an architecture delta worth a contract note; treat Strip parity under a separate `tab-strip` audit if one exists.
- The former `consv=gap` driver (undocumented Svelte surface `collapseWhenOverflow`/`fullWidth`/`collapseLabel` plus tooltip anatomy missing from §2) is now resolved in the contract — `consv=fixed`. Remaining items are all Rust-impl gaps, not contract↔Svelte.
- Biggest single gap: neither Rust target wires close/reorder/vertical, and both contradict the contract `font-weight: 600` (Jetstream) / no-tab-bottom-border (both) rules.
