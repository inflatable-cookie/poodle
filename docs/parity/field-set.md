<!-- parity consv=fixed gpui=6 jetstream=6 specimen=gap -->
# Parity: FieldSet

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/field-set.md`
- Svelte (authoritative): `packages/svelte/components/src/FieldSet.svelte`
- GPUI: `packages/gpui/components/src/primitives/field_set.rs`
- Jetstream: `packages/jetstream/components/src/field_set.rs`
- Spec: `packages/contracts/components/src/field_set.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FieldSetSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/field_set_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/field_set.rs`

## Contract ↔ Svelte

Svelte carries props + anatomy the contract and spec do not document. Svelte is authoritative — contract reconciled; remaining `FieldSetSpec` code changes tracked below.

- [x] FIXED `description?: string | null` (default `null`) — Svelte renders a `<p class="poodle-fieldset__description">` between legend and fields (`FieldSet.svelte:8,18,38-40,66-71`). Added to contract §2 anatomy + §3 props + §4 states + §6 tokens. Spec-side `description` field on `FieldSetSpec` remains (code track).
- [x] FIXED `span?: number | "full" | null` (default `null`) — already in contract §3; added §4 `spanned` state documenting `grid-column: span <n>` / `1 / -1`. Spec-side `span` field on `FieldSetSpec` remains (code track).
- [x] FIXED Legend font-size — contract §6 now documents legend font-size as fixed `0.6875rem` (eyebrow scale, not `typography-label-size`), Svelte wins on value. Spec-side `legend_size_token()` repoint remains (code track).
- [x] FIXED Legend typography detail — contract §6 now documents legend `font-weight: 600`, `letter-spacing: 0.12em`, `line-height: 1.5`, `text-transform: uppercase`. Spec-side token methods remain (code track).
- [x] FIXED Row-gap vs column-gap — contract §6 now documents asymmetric gap: `column-gap = scaleToSpace(gap)`, `row-gap = scaleToSpace(gap) + 0.5rem`. Spec-side row-gap derivation remains (code track).
- [x] FIXED `gap` type — Svelte `SpaceScale = "none" | "sm" | "md" | "lg"` (`types.ts:29`). Contract §3 references `SpaceScale` abstractly (consistent); the Rust `SpaceScale` enum missing `none` is a spec-side under-coverage (code track), not a contract divergence.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded legend size literal `px(rem_to_px(0.6875))` at `field_set.rs:74` — resolve from `spec.legend_size_token()` (or the new legend-size token), not a raw `0.6875`.
- [ ] Hardcoded column widths `rem_to_px(15.0)` / `rem_to_px(10.0)` / `rem_to_px(7.5)` at `field_set.rs:94-96` — these guessed per-column min-widths are not token-derived and do not implement Svelte's `repeat(columns, minmax(0, 1fr))` grid; replace with a real equal-fraction layout.
- [ ] Legend ignores `legend_family_token()`, `font-weight 600`, `letter-spacing 0.12em`, `line-height 1.5` — only `text_size` + `SEMIBOLD` + uppercase-in-Rust applied (`field_set.rs:73-79`). Apply family + letter-spacing tokens; `to_uppercase()` (`field_set.rs:78`) should be a `text-transform` token, not a Rust string op.
- [ ] Row-gap/column-gap collapsed to a single `gap` (`field_set.rs:83,87`); Svelte uses `row-gap = gap + 0.5rem`. Implement asymmetric gap once spec exposes it.
- [ ] No `description` part — `<p>` between legend and fields is absent (Svelte `FieldSet.svelte:38-40`). Add once spec gains `description`.
- [ ] No `span` support — builder has no `span` method and root never sets a parent-grid span (Svelte `FieldSet.svelte:33`). Add once spec gains `span`.
- accepted: no ARIA / native `<fieldset>`+`<legend>` grouping semantics (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [ ] Legend size uses `spec.legend_size_token()` (`TYPOGRAPHY_LABEL_SIZE`, `field_set.rs:23,33`) but Svelte legend is `0.6875rem` — wrong token until the dedicated legend-size token exists; then repoint.
- [ ] Legend missing `letter-spacing 0.12em`, `line-height 1.5`, and `text-transform: uppercase` — `js_field_set` sets only color/size/weight 600 and never uppercases the text (`field_set.rs:28-35`). Apply transform + letter-spacing tokens.
- [ ] Row-gap/column-gap collapsed to one `gap` on both root and grid (`field_set.rs:25,38`); Svelte uses `row-gap = gap + 0.5rem`. Implement asymmetric gap once spec exposes it.
- [ ] Multi-column is faked: `col_basis` percentage is computed then discarded (`_pct` unused, `field_set.rs:43-48,55-57`) and every child just calls `.grow()` regardless of `columns`, so `columns > 1` produces a single flow-wrap row, not an N-column grid matching `repeat(columns, minmax(0,1fr))`. Implement real column basis.
- [ ] No `description` part — `<p>` between legend and fields is absent (Svelte `FieldSet.svelte:38-40`). Add once spec gains `description`.
- [ ] No `span` support — `js_field_set` ignores parent-grid spanning (Svelte `FieldSet.svelte:33`). Add once spec gains `span`.
- accepted: no native `<fieldset>` grouping / ARIA semantics; no interaction needed (FieldSet is layout-only, so no preview event-loop handler is expected).

## Specimen parity

- Svelte covers: single-column with legend (incl. required/description/optionalLabel fields), two-column with `span="full"`, no-legend small-gap, multiple stacked sections (`FieldSetSpecimen.svelte:7-103`). Exercises `legend`, `columns`, `gap`, `span`, `description`-on-children.
- GPUI covers: single column, two columns, without legend (`gap=Sm`), multiple groups (`field_set_specimen.rs:13-215`). — missing: **`span="full"`** demonstration; no FieldSet-level `description` (spec lacks it).
- Jetstream covers: with legend, without legend, two columns (`field_set.rs:13-50`). — missing: **multiple-sections** group, **small-gap** variant, **`span`** demo; uses raw `js_text_input` instead of wrapping in `js_field` so field labels/required/description are absent.

## Notes

- Root cause of `consv=gap`: undocumented Svelte surface (`description` part) plus spec drift (`span` in contract but not spec; legend-size token disagrees with Svelte's `0.6875rem`; legend weight/letter-spacing/transform untokenized; asymmetric row-gap undocumented). All belong in contract + spec per "Svelte is parity authority".
- No color literals (`hsla/rgba/rgb`) in either Rust target — clean on that axis. The hardcoded-literal violations are all dimensional (legend size, column widths) in GPUI.
- Both Rust targets approximate the CSS grid with flex-wrap; neither reproduces `grid-template-columns: repeat(columns, minmax(0, 1fr))`. Acceptable as a rendering-engine delta only if equal-fraction columns are achieved — currently GPUI uses guessed rem widths and Jetstream collapses to one row, so both are functional gaps, not accepted deltas.
