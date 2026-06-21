<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap -->
<!-- pass: both Rust targets rebuilt — columns() + density() builders, density-aware root/header/title/body gaps + separated top-padding, separator rule = 0.0625rem, title-block gap from spec. Spec gained `density` field + density gap helpers. Jetstream probe tests added. -->
# Parity: DetailSection

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/detail-section.md`
- Svelte (authoritative): `packages/svelte/components/src/DetailSection.svelte`
- GPUI: `packages/gpui/components/src/composites/detail_section.rs`
- Jetstream: `packages/jetstream/components/src/detail_section.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DetailSectionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/detail_section_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/detail_section.rs`

## Contract ↔ Svelte

`consv=fixed`. Contract reconciled to Svelte's columns/auto-grid surface (contract-only edits; spec is code, out of scope here).

- [x] FIXED `columns` type in contract → `"auto" | 1 | 2 | 3 | 4` default `"auto"`, with auto-fit/maxAutoColumns notes (`DetailSection.svelte:12,25`). (Spec `.rs` `u8` change is code — tracked under targets, not edited here.)
- [x] FIXED `maxAutoColumns` (`2|3|4|5`, default `4`) added to contract §3/§8 (`DetailSection.svelte:14,27`).
- [x] FIXED `itemMinColumnWidth` (`string|null`, default `null`) added to contract §3, mapped to `--poodle-detail-section-item-min` (`DetailSection.svelte:13,36`).
- [x] FIXED inherited `density` already documented in contract §3 ("resolves from inherited presentation"); density var table updated to Svelte's data-density values + separated-gap column.
- [x] FIXED (extra, contract was stale) `separated` rule corrected from `border-top: 0` to the `::before` border-subtle 72% rule; title `font-weight: 700` added; responsive collapse rewritten from `max-width: 60rem` media query to container queries (`44rem`/`32rem`/`28rem`); `data-max-auto-columns` attribute documented.
- `separated`/`ariaLabel` map cleanly to spec `is_separated`/`aria_label` (naming only).

## GPUI gap (vs Svelte + contract)

- [x] FIXED `.columns()` builder added — `columns > 1` renders the body as a flex-wrap multi-column grid approximation.
- [x] FIXED `.density()` builder added — root / header / title-block / body gaps + separated top-padding resolve from the new density-aware spec helpers.
- [x] FIXED separator rule now `px(rem_to_px(0.0625))` (contract §8 0.0625rem), no longer raw `px(1.0)`; title-block gap now `title_gap_rem()`.
- accepted: no responsive collapse — Svelte uses container queries (44/32/28rem); GPUI flex-wrap is host-driven (Tier-3 implementation freedom, noted). No ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED `columns` rendering — `columns > 1` lays the body out as a wrapping row (flex grid approximation); single-column otherwise.
- [x] FIXED `density` support — all section gaps + separated top-padding resolve from the new spec density helpers.
- [x] FIXED title-block gap now `title_gap_rem()` and separator height `rem_to_px(0.0625)` (no raw `0.375` / `1.0`).
- accepted: container-query responsive collapse is host-driven (Tier-3, noted). Jetstream emits no ARIA.

## Specimen parity

- Svelte covers: title + rows, with-actions, item descriptions + truncate, two-column (`columns={2}`), density variants (compact/default/comfortable).
- GPUI covers: title + rows, with-actions, item descriptions, two-column (via hardcoded `div().w(px(192.0))` wrappers). — missing: **density variants**; two-column is faked with fixed-width wrappers (`detail_section_specimen.rs:154,162,170,178`) rather than a `columns` builder.
- Jetstream covers: title+description+body, title+actions, empty/no-separator. — missing: **two-column layout**, **density variants**, **description-only (no title)**.

## Notes

- The `consv=gap` driver is the columns/auto-grid surface (`columns` type, `maxAutoColumns`, `itemMinColumnWidth`, inherited `density`) — all undocumented in contract per "Svelte is parity authority".
- Both Rust targets ignore `columns` at render time, so multi-column sections only work in Svelte today.
