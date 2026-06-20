<!-- parity consv=fixed gpui=3 jetstream=5 specimen=gap -->
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

- [ ] No `layout()` builder — `DetailItemLayout` is used internally but render is fixed to spec default (Inline); cannot set stacked via builder. Add `.layout()`.
- [ ] No numeric `span` support (1/2/3/4) — only the spec's Full/Half; Svelte supports numerics.
- [ ] No `children` fallback slot for value rendering (Svelte uses `children` as the value when no `value`/`valueContent`); `.with_value_content()`/`.with_action()` exist but the bare-children path is absent.
- accepted: no ARIA / no description Popover (gpui has no accessibility API). Inline label width `px(rem_to_px(11.25))` at `detail_item.rs:121` is token-derived (matches contract `minmax(8rem, 11.25rem)`) — not a violation.

## Jetstream gap (vs Svelte + contract)

Drastically simplified: single flex row, always surface-like.

- [ ] No `layout` support — inline/stacked toggle absent; always a row.
- [ ] No `presentation` support — always applies padding/background ("surface"); no "simple" variant.
- [ ] Description rendered inline (right-aligned in the row), not as the contract's info-icon Popover.
- [ ] Hardcoded body font `rem_to_px(0.8125)` at `detail_item.rs:20` — resolve from `--poodle-typography-body-size` token.
- [ ] Hardcoded description font `rem_to_px(0.75)` at `detail_item.rs:21` — resolve from typography token, not raw 0.75.
- [ ] No `span` and no `action`/`valueContent` slots built.
- accepted: no ARIA (Jetstream emits no roles).

## Specimen parity

- Svelte covers: inline layout (default), with-description (Popover info icon), with-action slot, with-value slot (Pill), stacked layout (truncated UUID), surface presentation, density variants (compact/default/comfortable).
- GPUI covers: basic pairs, with-description (+truncate), with-action, with-value, stacked, surface. — missing: **density variants**, **responsive collapse demo** (45rem → single column).
- Jetstream covers: key-value rows, one inline description, basic rows. — missing: **action slot**, **value slot**, **stacked layout**, **simple presentation**, **density variants**, **info Popover description**.

## Notes

- The single most important fix is the Svelte layout inversion — it silently disables the default inline layout for every consumer.
- After fixing Svelte, the contract `presentation`/`emptyText` defaults and `span`/`valueContent` additions bring `consv` back to `ok`.
