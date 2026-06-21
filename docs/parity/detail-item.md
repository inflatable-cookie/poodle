<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- specimen pass: both Rust targets now cover inline + stacked, simple vs surface, empty/em-dash value, with-action (real js_button/Button), with-value slot (real js_pill/Pill), inline description, truncation, and the three density variants. Both preview crates build clean. -->

<!-- pass: both Rust targets rebuilt — layout(inline/stacked), simple/surface presentation, density-aware spacing, token-resolved fonts, em-dash placeholder, value emphasis, action slot. Spec gained `density` field + em-dash default + density/typography token helpers. Jetstream probe tests added. -->
# Parity: DetailItem

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/detail-item.md`
- Svelte (authoritative): `packages/svelte/components/src/DetailItem.svelte`
- GPUI: `packages/gpui/components/src/primitives/detail_item.rs`
- Jetstream: `packages/jetstream/components/src/detail_item.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DetailItemSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/detail_item_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/detail_item.rs`

## Contract ↔ Svelte

`consv=fixed`. Svelte layout bug already fixed in code; contract defaults/types reconciled to Svelte.

- [x] **Svelte layout-inversion bug — FIXED.** `DetailItem.svelte:43` now `layout === "stacked" ? "stacked" : "inline"`. Inline (the default) is reachable again; verified in preview (9 inline / 1 stacked render with `data-layout="inline"`).
- [x] FIXED `presentation` default → contract §3 now `"surface"` (matches `DetailItem.svelte:33`).
- [x] FIXED `emptyText` default → contract now em-dash `"—"` (matches `DetailItem.svelte:29`).
- [x] FIXED `span` type → contract now `"full" | "half" | 1 | 2 | 3 | 4 | null` with span-count notes (matches `DetailItem.svelte:18`).
- [x] FIXED `valueContent`/`action`/`children` snippets now listed in contract §3 props.
- [x] FIXED (extra) responsive collapse rewritten from `@media (max-width: 45rem)` to Svelte's `@container (max-width: 26rem)` + `21rem` steps; surface+stacked label treatment (text-tertiary, 0.75rem, lh 1.35) added to §7/§8.

## GPUI gap (vs Svelte + contract)

- [x] FIXED `.layout()` builder added — inline/stacked selectable; surface+stacked applies tertiary label (0.75rem) + value emphasis (1rem / SEMIBOLD).
- [x] FIXED `.density()` builder added — row-gap / inline-gap / surface padding resolve from the new density-aware spec helpers.
- [x] FIXED `.span()` builder added — `Full` stretches via `w_full()`. (Half / numeric 1-4 are grid-only; GPUI is flexbox, so they are inert without a grid parent — approximated, noted.)
- [x] FIXED value typography now resolves from `typography.label.size` / `typography.body.size` tokens (was implicit default).
- accepted: `with_value_content()` is the GPUI equivalent of Svelte's `children` value fallback. No ARIA / no description Popover (gpui has no accessibility API); description renders inline under the label. Inline label width `px(rem_to_px(11.25))` is token-derived (matches contract `minmax(8rem, 11.25rem)`).

## Jetstream gap (vs Svelte + contract)

Rebuilt: layout-aware row/column root, presentation-gated chrome, density-driven spacing, token fonts.

- [x] FIXED `layout` support — inline (row) vs stacked (column); surface+stacked applies tertiary label + value emphasis.
- [x] FIXED `presentation` support — `Surface` paints bg/padding/radius; `Simple` is plain (no chrome).
- [x] FIXED body font now `resolve_px(typography.label.size)` for the label and `typography.body.size` for the value (no raw `rem_to_px(0.8125)`).
- [x] FIXED `span` (`Full` → `self_stretch`) + `action`/`valueContent` slots built via `js_detail_item_with_slots`.
- [x] FIXED density-driven row/inline gap + surface padding from the new spec helpers.
- accepted: description rendered inline (JsEl has no popover channel — approximated, noted). `Full` span approximated via self-stretch (no flex grid). No ARIA (Jetstream emits no roles).

## Specimen parity

- Svelte covers: inline layout (default), with-description (Popover info icon), with-action slot, with-value slot (Pill), stacked layout (truncated UUID), surface presentation, density variants (compact/default/comfortable).
- GPUI covers: basic pairs, with-description (+truncate), with-action, with-value, stacked, surface, **simple vs surface**, **empty/em-dash value**, **density variants** (compact/default/comfortable). — missing: responsive collapse demo (container-query, host-driven Tier-3).
- Jetstream covers: inline (default), with-description (inline), **with-action slot** (real `js_button`), **with-value slot** (real `js_pill`), **empty/em-dash value**, **stacked layout** (truncated), **simple vs surface presentation**, **density variants** (compact/default/comfortable). — info Popover is inline (JsEl has no popover channel — approximated, noted in component parity).

## Notes

- The single most important fix is the Svelte layout inversion — it silently disables the default inline layout for every consumer.
- After fixing Svelte, the contract `presentation`/`emptyText` defaults and `span`/`valueContent` additions bring `consv` back to `ok`.
