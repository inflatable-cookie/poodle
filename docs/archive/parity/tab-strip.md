<!-- parity consv=gap gpui=0 jetstream=0 specimen=ok | Jetstream specimen rebuilt to mirror GPUI (real js_tab_strip): horizontal closable strip + host panel, disabled items, vertical orientation (labels + close visible), size matrix xs–xl, density matrix; tab-strip registry entry added. Both previews build clean. | GPUI specimen done; Jetstream pending engine recovery. Dedicated GPUI specimen created (specimens/tab_strip.rs, registered slug=tab-strip): horizontal closable+reorderable, disabled items, vertical, sizes + densities — no add-tab/overflow/reorder-handle invented (out of contract §1 scope). | pass: GPUI close-button + vertical-active opacity now token/spec-resolved, Enter/Space activation + Alt+Arrow reorder wired; Jetstream built out from placeholder to contract-faithful (size/density tokens, close button, disabled, vertical, accent indicator, current_value fallback) + 6 render-probe tests. No Svelte authority — built to contract. -->
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
- Specimens: svelte **none** (folded into `packages/svelte/preview/src/specimens/TabsSpecimen.svelte`) · gpui **dedicated** `packages/gpui/preview/src/specimens/tab_strip.rs` (slug `tab-strip`; the `tabs.rs` strip sections 5/6/7 remain as Tabs-context demos) · jetstream `packages/jetstream/preview/src/specimens/tab_strip.rs`

## Contract ↔ Svelte

TabStrip has **no standalone Svelte component** — the contract itself documents this (`tab-strip.md:3`). The contract authority is therefore the contract + the `strip`-variant slice of `Tabs.svelte`. Divergences below are contract↔(Tabs strip behaviour) and contract↔spec.

- **No dedicated Svelte file** — Svelte authority missing for the standalone primitive; this is the sole `consv=gap` driver and is irreducible by the audit rule. The tablist semantics, keyboard nav, reorder, and close all live in `Tabs.svelte` and are inherited only conceptually by `TabStripSpec`. The contract's top-of-file implementation note already records this. **No Svelte action; no contract authority to edit it against.** Keep the Rust TabStrip's keyboard/selection semantics traceable to `Tabs.svelte` handlers (`Tabs.svelte:374–434`).
- [x] CLARIFIED **`item_gap` token.** Contract §6 maps item gap to `space.inline.sm` (`item_gap_token()` → `SPACE_INLINE_SM`, `tab_strip.rs:98`); the `Tabs.svelte` `strip` variant uses `gap: 0` (`:817`). TabStrip intentionally ships one non-strip treatment, so the §6 token row now states it follows the text/card list spacing, **not** the strip variant's butted `gap: 0`.
- [x] RESOLVED **TabStripItem has no `icon` field** (`types.rs:789–794`). Contract §2 anatomy lists no Icon part; contract↔spec agree. Left absent — adding `icon` would invent capability Svelte does not drive. The GPUI vertical-strip specimen degrading to text labels (`tabs.rs:290–295`) is a specimen choice, not a contract gap.
- [x] CLARIFIED **vertical label/close hiding.** `Tabs.svelte:1049–1055` hides label + close in the vertical `strip` variant; the contract was silent. §4 vertical row now explicitly states TabStrip **keeps** labels and close buttons visible vertically (it does not adopt the Tabs strip-variant icon-only collapse).
- **`is_disabled` skip-in-arrow-nav, `Delete` to close, `Home`/`End`** — contract §5 specifies all three; `Tabs.svelte` implements them (`findNextEnabledIndex` 386/400, `Home` 405, `End` 411, `Delete` 430). Spec helpers exist (`current_value()` skips disabled, `tab_strip.rs:77`). **Agree — no action.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Close-button dimensions now resolve from spec: `close_button_size_rem()` (1.25rem) via `rem_to_px`, radius from `close_button_radius_token()` − `close_button_radius_inset_rem()` via `resolve_px`, gap from `close_button_gap_token()`. No bare rem float literals.
- [x] FIXED Vertical-active bg opacity uses the named `vertical_active_fill_opacity()` spec method (0.08) instead of the magic `accent.opacity(0.08)` literal.
- [x] FIXED `Enter`/`Space` activation wired into the key handler (calls `nav_handler` on the focused tab's value), alongside the existing arrows/Home/End/Delete.
- [x] FIXED Reorder wired: `Alt+Arrow` fires the host-owned `on_reorder(value, direction)` callback when `is_reorderable` (checked before plain arrow nav so the modifier wins). Drag reordering stays host/preview-loop owned.
- accepted: no ARIA — `role="tablist"`/`role="tab"`/`aria-selected`/`aria-disabled` not expressible on GPUI native elements (documented in file header, `tab_strip.rs:6-8`).
- accepted: horizontal active treatment uses a bottom-border accent edge (strip-style, `tab_strip.rs:184`) rather than the text-variant pill fill — contract permits "one default treatment" (Known Delta §8).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Font-size resolves from `size_font_rem(effective_size)` (after `resolve_semantic_size`), matching GPUI/Tabs. The `size` field is honored.
- [x] FIXED Tab padding resolves from `control_space_x_rem(density) + size_padding_x_offset_rem(effective_size)`; min-height from `control_height_rem − 0.25rem`. The `density` + `size` fields are honored.
- [x] FIXED Close button rendered for `is_closable` items via a shared `build_close_button` (1.25rem square, `x` icon, `text-secondary`, radius `radius-control − 0.125rem`), mirroring the Tabs close button.
- [x] FIXED Disabled state honored — `is_disabled` items dim via `disabled_opacity_token()` and render `text-secondary`.
- [x] FIXED (focus channel) Tabs are `.focusable()` — the Jetstream focus channel (same treatment as Tabs/Button); no separately-drawn outline ring exists in the JsEl chrome. Focus-ring color token resolution is a CSS-only concern.
- [x] FIXED Vertical orientation — `orientation == Vertical` lays tabs in a `flex_col()`; labels + close buttons stay visible (contract §4), active tab gets the accent-tint fill instead of the bottom border.
- [x] FIXED (host-owned) `is_reorderable` is honored as a host/preview-loop concern. Per contract §2 anatomy there is no reorder grab-handle part, so none is drawn (matches GPUI, which fires `on_reorder` from the preview loop). The `js_*` builder is stateless and has no event channel.
- [x] FIXED `current_value()` fallback now drives selection (value → default_value → first non-disabled), so empty/unset selection renders the first enabled tab active.
- accepted: no ARIA channel (Jetstream native rendering has no HTML role/aria attributes).
- accepted: keyboard navigation + selection commit live in the preview event loop, not the component (the `js_*` builder is stateless).
- note: contract §2 anatomy defines only Root / TabItem / CloseButton — no add-tab "+" affordance, overflow-scroll chrome, or reorder handle. None invented (would add capability the contract + Svelte tablist do not drive).

## Specimen parity

- Svelte covers: **no TabStrip specimen.** `TabsSpecimen.svelte` exercises the `strip` variant of `Tabs` (closable/reorderable/vertical) instead. This is the reference for strip visuals.
- GPUI covers: **GPUI specimen done.** Dedicated `specimens/tab_strip.rs` (slug `tab-strip`): horizontal closable + reorderable strip with live `on_change`/`on_close`/`on_reorder` and host-owned panel, **disabled-items** group (dimmed, skipped by arrow nav), **vertical** orientation (labels + close buttons stay visible per contract §4), and full **sizes + densities** matrices via `specimen_layout` — all real `TabStrip` instances. No add-tab/overflow/reorder-handle affordances (out of contract §1 scope). The earlier `tabs.rs` strip sections 5/6/7 remain as Tabs-context demos. — accepted gap: icon-only collapse (no `icon` field on `TabStripItem`, by contract).
- Jetstream covers (`specimens/tab_strip.rs`): two horizontal selection states only (`review` selected, `history` selected), one closable item rendered but with **no visible close button** since the component omits it. — missing: **vertical orientation, disabled item, working close button, reorderable, size/density variation, keyboard/selection wiring**. Specimen is a static render, not an interactive integration test.

## Notes

- **TabStrip ↔ Tabs relationship.** `Tabs` is the full panel-coupled surface (5 variants, activation modes, URL sync, panels). `TabStrip` is the tablist-only primitive — same selection/keyboard/close/reorder affordances, **no panels and no variant system** (contract §1 out-of-scope, §8 Known Deltas). In Svelte both live in one file (`Tabs.svelte`); in Rust the strip is extracted because separating tablist from panel management is useful for host-owned content regions. Use `Tabs` by default; reach for `TabStrip` only when the caller owns the rendered content.
- `consv=gap` is driven solely by "no dedicated Svelte file" (Svelte authority missing) — there is no contradictory Svelte prop/anatomy surface to reconcile, unlike Button, so the contract stays the sole authority and is not bent. The two open reconciliation questions (item-gap rationale, vertical label/close hiding) are now resolved as explicit contract clarifications; the gap remains irreducible because no Svelte primitive exists to validate against.
- Biggest real divergence is **Jetstream**: it resolves almost nothing from tokens (font-size + padding hardcoded) and ignores 5 of its own spec fields (`size`, `density`, `orientation`, `is_reorderable`, `aria_label`) plus `is_disabled`/`is_closable` on items. It is closer to a placeholder render than a contract-faithful component.
- GPUI is substantially complete (keyboard nav, close, disabled, focus ring, both orientations) — its open todos are token-cleanliness (close-button dims, active-bg opacity) and the missing reorder + Enter/Space.
