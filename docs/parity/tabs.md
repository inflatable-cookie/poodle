<!-- parity consv=gap gpui=11 jetstream=12 specimen=gap -->
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

Svelte carries props the contract omits, and the contract `variant` union deviates from Svelte's enum naming. Svelte authoritative.

- Svelte adds `collapseWhenOverflow?: boolean` (`Tabs.svelte:46`) → collapses the tablist into a `Menu` when it overflows. Not in contract §3. **Fix: add to contract props + anatomy (collapsed-menu affordance).**
- Svelte adds `fullWidth?: boolean` (`Tabs.svelte:47`) → tabs flex to fill the row (`data-full-width`). Not in contract. **Fix: add to contract.**
- Svelte adds `collapseLabel?: string | null` (`Tabs.svelte:48`) — label for the collapsed trigger. Not in contract. **Fix: add to contract.**
- Variant naming: contract §3 lists `"text"` as canonical with `"underline"` a deprecated alias. Svelte's `TabVariant` type / Rust `TabVariant` enum use `Underline` as the canonical name (no `Text`/`Block` mismatch in Rust — Rust enum is `Underline | Card | Pill | Block`, no `Strip`). Contract names the default `"text"`; Svelte default is `"text"` but renders identical to underline. **Fix: reconcile contract canonical name with the Rust `TabVariant::Underline` used by both Rust targets.**
- Contract §3 omits `block` from prop default examples but documents it in token tables; Svelte/Rust both implement `block`. Consistent — no fix.
- Tooltip behavior (`showTooltips`, vertical icon-only tooltips, `Tabs.svelte:189-206,600-604`) is documented in contract §3 (`showTooltips`) but the tooltip anatomy part is not in §2. **Fix: add tooltip to contract anatomy.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded badge geometry literals `min_w(px(rem_to_px(1.125)))`, `px(rem_to_px(0.3125))`, `rounded(px(rem_to_px(0.5625)))` at `tabs.rs:65-67` — resolve from Pill spec tokens, not raw rems.
- [ ] Hardcoded card-row gap `gap(px(rem_to_px(0.125)))` at `tabs.rs:330`; pill gap `0.125` at `tabs.rs:646`; pill padding `0.1875` at `tabs.rs:648`; pill tab-height delta `px(rem_to_px(0.5))` at `tabs.rs:653` — contract values, but resolve via tokens/spec multipliers not inline floats.
- [ ] Hover background uses `color.background.elevated` (`tabs.rs:196,398`) — contract specifies `color.surface-hover` for strip/block hover; underline/text hover is not contract-specified at all (Svelte text variant has no hover bg). Wrong token + extra hover.
- [ ] Inline density→padding match arms (`tabs.rs:181-185,469-473,619-623`) hardcode `0.5/0.75/1.0` rem instead of resolving `space.control.x` per density from tokens.
- [ ] Active-tab `border_b_2()` accent underline applied in underline variant (`tabs.rs:243-245`) — Svelte text variant has NO bottom border on the tab; the indicator is a pill-shaped bg only. Visual divergence.
- [ ] No close button interaction — card close icon rendered (`tabs.rs:441-452`) but no `onClose` callback wired; Delete key unhandled.
- [ ] No reorder (drag-and-drop or Alt+Arrow) — `reorderable` absent from builder + spec usage.
- [ ] No `strip` variant in `Tabs` — handled by separate `tab_strip.rs`/`TabStripItem`; contract treats strip as a Tabs variant. Note split.
- [ ] No vertical orientation rendering — `orientation` builder exists (`tabs.rs:132`) but render fns ignore it; no icon-only collapse, no label/close hiding.
- [ ] No `separator`, `actions`, `count` token resolution (count badge bg is `color_mix(text,surface,0.14)` ad-hoc, not Pill tokens), `collapseWhenOverflow`, `fullWidth`, `historyKey`, tooltips.
- [ ] Arrow-key nav wraps unconditionally and ignores disabled tabs (`tabs.rs:276-292`) — Svelte skips disabled via `findNextEnabledIndex`; no Home/End handling.
- accepted: no ARIA (gpui has no accessibility API) — `role=tablist/tab/tabpanel`, `aria-selected`, `aria-controls` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded badge literals `rem_to_px(0.6875)`, `rem_to_px(0.5625)`, `rem_to_px(0.3125)`, `min_w(rem_to_px(1.125))` at `tabs.rs:70-80` — resolve from Pill/caption tokens.
- [ ] Hardcoded gaps `rem_to_px(0.25)` (`tabs.rs:44`), `rem_to_px(0.125)` (`tabs.rs:178,261`), pill padding `rem_to_px(0.1875)` (`tabs.rs:264`), pill height delta `rem_to_px(0.5)` (`tabs.rs:239`) — use tokens.
- [ ] `pad_y` computed as `control_height_rem * 0.25` heuristic (`tabs.rs:93,155`) — not a token; Svelte uses `padding: 0 Xrem` (zero vertical) + `min-height: calc(control-height - 0.25rem)`. Wrong vertical model.
- [ ] Inactive tab font-weight `400` and active `600`/`700` (`tabs.rs:123,200,278,346`) — contract §8 says ALL tabs are `font-weight: 600`, weight does not change on selection. Wrong.
- [ ] Active underline uses `border_b_2().border_color_bottom(accent)` (`tabs.rs:133-137`) — Svelte text variant has no accent bottom border on the tab; pill-shaped bg only. Visual divergence.
- [ ] No close button rendered at all in `js_tabs` (card/strip closable tabs show no `x`) — Svelte/contract require it; no `onClose`.
- [ ] No reorder; interaction (click/keyboard) lives in preview `main.rs` event loop — note if absent there.
- [ ] No `strip` variant in `js_tabs` — handled by separate `tab_strip.rs`. Contract treats strip as a Tabs variant.
- [ ] No vertical orientation, no `separator`, no `actions`, no `count` Pill tokens, no `collapseWhenOverflow`/`fullWidth`/`historyKey`/tooltips.
- [ ] No panel rendering — `js_tabs` returns tab bar only (`tabs.rs:394-407`); content is caller's responsibility, so `role=tabpanel`/`aria-labelledby` linkage absent.
- [ ] Card active-tab bottom-border removal logic (`tabs.rs:211-221`) is a manual border reset workaround — fragile; no token basis.
- [ ] Disabled tabs not skipped in nav; Home/End/Delete unhandled (lives in main.rs).
- accepted: no ARIA channel; interaction in preview event loop, not the component.

## Specimen parity

- Svelte covers: Text+panel, Card (closable+reorderable), Block (full-width+separators), Pill (icons), Underline (icons, no panel), Strip (horizontal full-width closable reorderable), Strip vertical (icon-only), Strip collapse-toggle, Card with counts+separators+URL sync (`TabsSpecimen.svelte`).
- GPUI covers: Underline, Card, Card+counts(icons), Pill, Block, Underline+icons, Strip (via TabStrip), Strip vertical, collapse-toggle. — missing: reorder demo parity in Tabs proper, separator demo, fullWidth, historyKey.
- Jetstream covers: Underline, Card, Pill, Block, icons (Underline/Pill), count badges, disabled. — missing: **closable/close button**, **reorder**, **strip variant** (in this specimen), **vertical orientation**, **panel content**, **separators**, **counts+URL sync**.

## Notes

- Strip variant is split into a dedicated `tab_strip.rs` component in both Rust targets, diverging from Svelte where strip is a `Tabs` variant. This is an architecture delta worth a contract note; treat Strip parity under a separate `tab-strip` audit if one exists.
- The `consv=gap` driver is undocumented Svelte surface (`collapseWhenOverflow`/`fullWidth`/`collapseLabel`) plus tooltip anatomy missing from contract §2.
- Biggest single gap: neither Rust target wires close/reorder/vertical, and both contradict the contract `font-weight: 600` (Jetstream) / no-tab-bottom-border (both) rules.
