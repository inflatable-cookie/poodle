<!-- parity consv=fixed gpui=1 jetstream=1 specimen=ok -->
<!-- specimen=ok: both Rust specimens backfilled (legend+description, real Field children via js_field, multi-column, span=full, gap=none, multiple groups); Jetstream field_set children are now real Field components (was bare text inputs); both previews build clean. -->
<!-- pass: FieldSetSpec gained description, span, SpaceScale::None; legend size repointed to fixed 0.6875rem eyebrow scale (LEGEND_SIZE_REM), gap_token now mirrors Svelte scaleToSpace (sm→inline-sm, md→panel-y, lg→panel-x, none→0) and returns Option; description part + asymmetric row-gap (col-gap + 0.5rem) + equal-fraction columns implemented both targets; Jetstream legend uppercased. Remaining open: span = accepted layout delta; legend letter-spacing/line-height = engine delta (no JsEl/GPUI div API). Jetstream probe tests cover legend+description+children+multi-column+gap-none. -->
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

- [x] DONE Legend size now resolves from `FieldSetSpec::LEGEND_SIZE_REM` (the contract-exact `0.6875rem` eyebrow constant) via `rem_to_px`, not a raw literal — and is no longer the wrong `typography-label-size` token.
- [x] DONE Guessed column min-widths removed — multi-column uses `flex_1().min_w(0)` per child (≈ `repeat(columns, minmax(0, 1fr))`).
- [x] DONE Legend keeps `SEMIBOLD` + uppercase (`to_uppercase`); `letter-spacing 0.12em` and `line-height 1.5` have **no GPUI div API** — accepted rendering delta, noted inline. (`font-weight 600` ≈ SEMIBOLD.)
- [x] DONE Asymmetric gap implemented — column-gap = `column_gap_token` (Svelte `scaleToSpace`), row-gap = `column-gap + 0.5rem` (`ROW_GAP_EXTRA_REM`).
- [x] DONE `description` part renders as a styled block between legend and fields (`description_size_token` = body-size, secondary color, `space-stack-md` margin-bottom).
- [ ] `span` builder added on the spec but not emitted as grid placement — **accepted layout delta**: GPUI has no CSS-grid parent context (contract §6 `spanned` / §12). Documented inline.
- accepted: no ARIA / native `<fieldset>`+`<legend>` grouping semantics (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [x] DONE Legend size repointed to the fixed `0.6875rem` eyebrow scale via `rem_to_px(FieldSetSpec::LEGEND_SIZE_REM)` (no longer `TYPOGRAPHY_LABEL_SIZE`).
- [x] DONE Legend now uppercases via `to_uppercase()` (verified by probe test asserting `"CONTACT"`); `letter-spacing 0.12em` / `line-height 1.5` have **no JsEl API** — accepted rendering delta, noted inline.
- [x] DONE Asymmetric gap implemented — column-gap = `column_gap_token` (Svelte `scaleToSpace`), wrap row-gap = `column-gap + 0.5rem` when multi-column.
- [x] DONE Real equal-fraction columns — each child wrapper gets `flex_basis(100/cols).flex_grow().flex_shrink().min_w_0()`; probe test confirms two columns share a row and the second sits right of the first.
- [x] DONE `description` part renders between legend and fields (body-size, secondary color, `space-stack-md` margin-bottom).
- [ ] `span` builder added on the spec but not emitted as grid placement — **accepted layout delta**: no Jetstream flex equivalent (contract §6 `spanned` / §12). Documented inline.
- accepted: no native `<fieldset>` grouping / ARIA semantics; no interaction needed (FieldSet is layout-only, so no preview event-loop handler is expected).

## Specimen parity

- Svelte covers: single-column with legend (incl. required/description/optionalLabel fields), two-column with `span="full"`, no-legend small-gap, multiple stacked sections (`FieldSetSpecimen.svelte:7-103`). Exercises `legend`, `columns`, `gap`, `span`, `description`-on-children.
- GPUI covers: single column, two columns, without legend (`gap=Sm`), multiple groups (`field_set_specimen.rs:13-215`). — missing: **`span="full"`** demonstration; no FieldSet-level `description` (spec lacks it).
- Jetstream covers: with legend, without legend, two columns (`field_set.rs:13-50`). — missing: **multiple-sections** group, **small-gap** variant, **`span`** demo; uses raw `js_text_input` instead of wrapping in `js_field` so field labels/required/description are absent.

## Notes

- Root cause of `consv=gap`: undocumented Svelte surface (`description` part) plus spec drift (`span` in contract but not spec; legend-size token disagrees with Svelte's `0.6875rem`; legend weight/letter-spacing/transform untokenized; asymmetric row-gap undocumented). All belong in contract + spec per "Svelte is parity authority".
- No color literals (`hsla/rgba/rgb`) in either Rust target — clean on that axis. The hardcoded-literal violations are all dimensional (legend size, column widths) in GPUI.
- Both Rust targets approximate the CSS grid with flex-wrap; neither reproduces `grid-template-columns: repeat(columns, minmax(0, 1fr))`. Acceptable as a rendering-engine delta only if equal-fraction columns are achieved — currently GPUI uses guessed rem widths and Jetstream collapses to one row, so both are functional gaps, not accepted deltas.
