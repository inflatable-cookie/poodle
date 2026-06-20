<!-- parity consv=gap gpui=3 jetstream=3 specimen=gap -->
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

`consv=gap`. Contract/spec under-specify `columns` and omit Svelte's auto-grid props.

- `columns` type: contract spec (`packages/contracts/components/src/detail_section.rs`) is `u8` (default `1`); Svelte is `"auto" | 1 | 2 | 3 | 4` (default `"auto"`) with responsive auto-fit. Svelte authoritative. **Fix contract + spec to `"auto" | 1..4` with default `"auto"`.**
- `maxAutoColumns` (`2 | 3 | 4 | 5`, default `4`) present in Svelte, absent from contract/spec. **Fix: add to contract.**
- `itemMinColumnWidth` (`string | null`, default `null`) present in Svelte, absent from contract/spec. **Fix: add to contract.**
- `density` (`ControlDensity | null`, resolved from presentation context) present in Svelte, not in spec. **Fix: document inherited density in contract.**
- `separated`/`ariaLabel` map cleanly to spec `is_separated`/`aria_label` (naming only).

## GPUI gap (vs Svelte + contract)

- [ ] No `columns()` builder — `spec.columns > 1` is checked at `detail_section.rs:153` but only the spec default (1) is reachable; no way to set multi-column. Add builder; honor `"auto"` once contract lands.
- [ ] No `density` builder/awareness — section gaps fixed; Svelte varies gaps by density.
- [ ] No responsive collapse — Svelte uses container queries (44rem/32rem); GPUI flex-wrap is unconditional. (Acceptable as host-driven, but note the delta.)
- accepted: no ARIA (gpui has no accessibility API). Title-block gap `px(rem_to_px(0.375))` (`:120`), title size `px(rem_to_px(1.125))` (`:125`), separator `px(1.0)` (`:100`) are token/contract-derived rem conversions — not raw-px violations, though separator `1.0` and the rem literals would ideally come from spec methods.

## Jetstream gap (vs Svelte + contract)

- [ ] No `columns` rendering — `spec.columns` ignored; content always flex-column. Add multi-column layout when `columns > 1` / `"auto"`.
- [ ] No `density` support — no density-aware spacing.
- [ ] Hardcoded title-block gap `rem_to_px(0.375)` at `detail_section.rs:59` and separator `h(1.0)` at `:39` — resolve from spec/tokens (separator height especially should be a token, not raw 1.0).
- accepted: interaction n/a; Jetstream emits no ARIA.

## Specimen parity

- Svelte covers: title + rows, with-actions, item descriptions + truncate, two-column (`columns={2}`), density variants (compact/default/comfortable).
- GPUI covers: title + rows, with-actions, item descriptions, two-column (via hardcoded `div().w(px(192.0))` wrappers). — missing: **density variants**; two-column is faked with fixed-width wrappers (`detail_section_specimen.rs:154,162,170,178`) rather than a `columns` builder.
- Jetstream covers: title+description+body, title+actions, empty/no-separator. — missing: **two-column layout**, **density variants**, **description-only (no title)**.

## Notes

- The `consv=gap` driver is the columns/auto-grid surface (`columns` type, `maxAutoColumns`, `itemMinColumnWidth`, inherited `density`) — all undocumented in contract per "Svelte is parity authority".
- Both Rust targets ignore `columns` at render time, so multi-column sections only work in Svelte today.
