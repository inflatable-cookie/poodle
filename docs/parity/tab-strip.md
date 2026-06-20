<!-- parity consv=gap gpui=4 jetstream=8 specimen=gap -->
# Parity: TabStrip

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/tab-strip.md` (authoritative for this audit)
- Related contract: `docs/contracts/components/tabs.md` (panel-coupled superset; strip is the tablist-only primitive underneath it)
- Svelte (authoritative): **no dedicated `TabStrip.svelte`.** The tablist is built directly into `packages/svelte/components/src/Tabs.svelte` (the `role="tablist"` list at lines 518–617). The strip-only primitive is a Rust-only extraction. Closest visual reference is the `strip` variant of `Tabs.svelte` (CSS lines 815–821, 972–1046).
- GPUI: `packages/gpui/components/src/primitives/tab_strip.rs`
- Jetstream: `packages/jetstream/components/src/tab_strip.rs`
- Spec: `packages/contracts/components/src/tab_strip.rs` (`TabStripSpec`), item type `packages/contracts/components/src/types.rs:789` (`TabStripItem`)
- Specimens: svelte **none** (folded into `packages/svelte/preview/src/specimens/TabsSpecimen.svelte`) · gpui **none dedicated** — driven via `packages/gpui/preview/src/specimens/tabs.rs` sections 5/6/7 · jetstream `packages/jetstream/preview/src/specimens/tab_strip.rs`

## Contract ↔ Svelte

TabStrip has **no standalone Svelte component** — the contract itself documents this (`tab-strip.md:3`). The contract authority is therefore the contract + the `strip`-variant slice of `Tabs.svelte`. Divergences below are contract↔(Tabs strip behaviour) and contract↔spec.

- **No dedicated Svelte file** — drives `consv=gap` by the audit rule. The tablist semantics, keyboard nav, reorder, and close all live in `Tabs.svelte` and are inherited only conceptually by `TabStripSpec`. **Action: none required on Svelte; the contract's implementation note already records this. Keep the Rust TabStrip's keyboard/selection semantics traceable to `Tabs.svelte` handlers (`Tabs.svelte:374–434`).**
- **`item_gap` token mismatch.** Contract §6 maps item gap to `space.inline.sm` (`item_gap_token()` returns `SPACE_INLINE_SM`, `tab_strip.rs:98`). But the `strip` variant in `Tabs.svelte` uses `gap: 0` on the list (`Tabs.svelte:817`) — strip tabs butt together, gap lives only between non-strip tabs. **The strip reference is right; the standalone TabStrip intentionally ships one non-strip treatment, so `space.inline.sm` gap is acceptable. Document that TabStrip's gap follows the text/card list (0.25rem-class), not the strip variant.**
- **TabStripItem has no `icon` field** (`types.rs:789–794`: only `value/label/is_disabled/is_closable`). The contract anatomy §2 lists no Icon part either, so contract↔spec agree. **But** the GPUI/Tabs specimen for the vertical strip is icon-only (`tabs.rs:290–295` uses bare labels as a stand-in). **Action: if icon-only vertical strips are a real requirement, add `icon` to `TabStripItem` + contract §2 anatomy; today it is correctly absent and the vertical specimen degrades to text.**
- **Contract §4 says vertical hides labels (icon-only).** `Tabs.svelte:1049–1055` hides `.poodle-tabs__label` and `.poodle-tabs__close` in vertical orientation. The contract's TabStrip §4 vertical row only says "column layout with up/down arrow navigation" — it does **not** specify label hiding. **Minor: contract is silent on vertical label/close hiding; reconcile with Tabs behaviour (hide both in vertical) or explicitly state TabStrip keeps labels visible vertically.**
- **`is_disabled` skip-in-arrow-nav, `Delete` to close, `Home`/`End`** — contract §5 specifies all three; `Tabs.svelte` implements them (`findNextEnabledIndex` 386/400, `Home` 405, `End` 411, `Delete` 430). Spec helpers exist (`current_value()` skips disabled, `tab_strip.rs:77`). **Agree — no action.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded close-button dimensions: `.w(px(rem_to_px(1.25)))`, `.h(px(rem_to_px(1.25)))`, `.rounded(px(rem_to_px(0.25)))` at `tab_strip.rs:286-288` — raw rem literals, not token-resolved. Contract close-button is `1.25rem`/`radius-control − 0.125rem`; resolve via a close-button size token + `resolve_radius`, not float constants.
- [ ] Hardcoded vertical-active bg opacity `accent.opacity(0.08)` at `tab_strip.rs:182` — magic multiplier, no token/spec method. Centralize like Tabs' opacity multipliers (`tabs.md:546-554`).
- [ ] No `Enter`/`Space` activation in `manual` mode and no roving `tabindex` model — key handler covers arrows/Home/End/Delete only (`tab_strip.rs:216-268`); contract §5 lists Enter/Space activation. (TabStrip has no activationMode prop, so automatic-only is defensible — but Enter/Space confirm is still contract-listed.)
- [ ] No reorder implementation — `is_reorderable` is a spec field and a builder (`reorderable()`, `tab_strip.rs:77`) but nothing reads it; no drag, no `Alt+Arrow`. Contract §4 reorderable state + §3 `isReorderable` unhonored.
- accepted: no ARIA — `role="tablist"`/`role="tab"`/`aria-selected`/`aria-disabled` not expressible on GPUI native elements (documented in file header, `tab_strip.rs:6-8`).
- accepted: horizontal active treatment uses a bottom-border accent edge (strip-style, `tab_strip.rs:184`) rather than the text-variant pill fill — contract permits "one default treatment" (Known Delta §8).

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded font-size `rem_to_px(0.8125)` at `tab_strip.rs:19` — must resolve from size token (`size_font_rem(effective_size)` like GPUI `tab_strip.rs:121`); the `size` spec field is ignored entirely.
- [ ] Hardcoded tab padding `rem_to_px(0.25)` / `rem_to_px(0.5)` at `tab_strip.rs:20-21` — must resolve from control-y + density-aware control-x tokens; the `density` spec field is ignored.
- [ ] No close button — `is_closable` items render no close affordance; contract §2 CloseButton + §4 closable state unimplemented (`closable_item_count()` unused).
- [ ] No disabled state — `is_disabled` never read; no opacity dim, no `not-allowed` cursor, not skipped by nav. Contract §4 disabled-item + `disabled_opacity_token()` unused.
- [ ] No focus ring — `focus_ring_color_token()` unused; tabs are `.focusable()` (`tab_strip.rs:38`) but draw no focus outline. Contract §6 focus ring.
- [ ] No vertical orientation — always `flex_row()` (`tab_strip.rs:23`); `orientation` spec field ignored. Contract §4 vertical state + up/down nav.
- [ ] No reorder — `is_reorderable` spec field ignored. Contract §4 reorderable.
- [ ] `current_value()` fallback not used — selection is `value.or(default_value)` only (`tab_strip.rs:16`), missing the "first non-disabled item" fallback the spec helper provides (`tab_strip.rs:77`). Empty/unset selection renders nothing active.
- accepted: no ARIA channel (Jetstream native rendering has no HTML role/aria attributes).
- accepted: keyboard navigation + selection commit live in the preview event loop, not the component — but this component's specimen never wires them (see Specimen parity).

## Specimen parity

- Svelte covers: **no TabStrip specimen.** `TabsSpecimen.svelte` exercises the `strip` variant of `Tabs` (closable/reorderable/vertical) instead. This is the reference for strip visuals.
- GPUI covers (`specimens/tabs.rs`): horizontal strip closable+reorderable (§5, `tabs.rs:238-287`), vertical strip (§6, `tabs.rs:289-318`), collapse-toggle horizontal↔vertical (§7, `tabs.rs:320-357`) — all real `TabStrip` instances with live `on_change`/`on_close`. — missing: disabled-item demo; icon-only vertical (degrades to text labels since `TabStripItem` has no icon).
- Jetstream covers (`specimens/tab_strip.rs`): two horizontal selection states only (`review` selected, `history` selected), one closable item rendered but with **no visible close button** since the component omits it. — missing: **vertical orientation, disabled item, working close button, reorderable, size/density variation, keyboard/selection wiring**. Specimen is a static render, not an interactive integration test.

## Notes

- **TabStrip ↔ Tabs relationship.** `Tabs` is the full panel-coupled surface (5 variants, activation modes, URL sync, panels). `TabStrip` is the tablist-only primitive — same selection/keyboard/close/reorder affordances, **no panels and no variant system** (contract §1 out-of-scope, §8 Known Deltas). In Svelte both live in one file (`Tabs.svelte`); in Rust the strip is extracted because separating tablist from panel management is useful for host-owned content regions. Use `Tabs` by default; reach for `TabStrip` only when the caller owns the rendered content.
- `consv=gap` is driven solely by "no dedicated Svelte file" — there is no contradictory Svelte prop/anatomy surface to reconcile, unlike Button. The contract already documents the absence.
- Biggest real divergence is **Jetstream**: it resolves almost nothing from tokens (font-size + padding hardcoded) and ignores 5 of its own spec fields (`size`, `density`, `orientation`, `is_reorderable`, `aria_label`) plus `is_disabled`/`is_closable` on items. It is closer to a placeholder render than a contract-faithful component.
- GPUI is substantially complete (keyboard nav, close, disabled, focus ring, both orientations) — its open todos are token-cleanliness (close-button dims, active-bg opacity) and the missing reorder + Enter/Space.
