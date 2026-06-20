<!-- parity consv=gap gpui=6 jetstream=6 specimen=gap -->
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

Svelte carries props + anatomy the contract and spec do not document. Svelte is authoritative — update contract + spec.

- `description?: string | null` (default `null`) — Svelte renders a `<p class="poodle-fieldset__description">` between legend and fields (`FieldSet.svelte:8,18,38-40,66-71`). Contract §2 anatomy + §3 props omit it entirely; `FieldSetSpec` has no `description` field (`field_set.rs:12-17`). **Fix: add `description` to contract §2 anatomy + §3 props + §6 tokens, and to `FieldSetSpec`.**
- `span?: number | "full" | null` (default `null`) — present in contract §3 props but **not in `FieldSetSpec`** (`field_set.rs:12-17`); Svelte composes it into `grid-column` on the root (`FieldSet.svelte:11,21,33`). **Fix: add `span` to `FieldSetSpec` so Rust targets can honor parent-grid spanning.**
- Legend font-size: Svelte hardcodes `font-size: 0.6875rem` (`FieldSet.svelte:57`), but the spec exposes `legend_size_token()` → `TYPOGRAPHY_LABEL_SIZE` (`field_set.rs:65-67`), which is a different value. The two disagree on what drives legend size. **Fix: define a dedicated eyebrow/legend size token at `0.6875rem` and point both Svelte and spec at it (Svelte wins on value).**
- Legend typography detail: Svelte legend also sets `font-weight: 600`, `letter-spacing: 0.12em`, `line-height: 1.5`, `text-transform: uppercase` (`FieldSet.svelte:58-61`). Contract §6 token table only lists color + family. `FieldSetSpec` exposes `legend_family_token()` but no weight / letter-spacing / transform tokens. **Fix: document legend weight/letter-spacing/transform in contract §6 and add token methods to the spec.**
- Row-gap vs column-gap: Svelte uses `row-gap = scaleToSpace(gap) + 0.5rem` and `column-gap = scaleToSpace(gap)` (`FieldSet.svelte:24-28`) — asymmetric. Contract §6 describes a single "grid gap"; spec exposes one `gap_token()` (`field_set.rs:49-55`). **Fix: document the `+0.5rem` row-gap offset in contract and expose a row-gap token/derivation in the spec.**
- `gap` type: contract §3 says `gap: SpaceScale` default `"md"` with values implied `sm|md|lg`; Svelte's `SpaceScale` type may carry more steps, but the Rust `SpaceScale` enum is `Sm|Md|Lg` only (`field_set.rs:5-10`). Confirm Svelte `SpaceScale` union matches; if Svelte allows `xs|xl`, contract+spec under-cover. **Fix: reconcile `SpaceScale` range across types.**

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
