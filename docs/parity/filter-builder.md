<!-- parity consv=ok gpui=1 jetstream=1 specimen=ok -->
# Parity: FilterBuilder

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/filter-builder.md`
- Svelte (authoritative): `packages/svelte/components/src/FilterBuilder.svelte` (+ pure model `packages/svelte/components/src/filter-builder-model.ts`)
- React: `packages/react/components/src/FilterBuilder.tsx` (+ `packages/react/components/src/filter-builder-model.ts`)
- GPUI: `packages/gpui/components/src/primitives/filter_builder.rs`
- Jetstream: `packages/jetstream/components/src/filter_builder.rs`
- Spec: `packages/contracts/components/src/filter_builder.rs` (`FilterBuilderSpec` + value model + `default_operators_for_kind`)
- Specimens: svelte `packages/svelte/preview/src/specimens/FilterBuilderSpecimen.svelte` · react `packages/react/preview/src/gallery/specimens/FilterBuilderSpecimen.tsx` · gpui `packages/gpui/preview/src/specimens/filter_builder_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/filter_builder.rs`

## Contract ↔ Svelte

`consv=ok`. The contract was authored fresh alongside the Svelte implementation (new component), so they match by construction. Svelte remains authoritative.

- [x] Six field kinds (boolean, enum, multi-enum, text, number, range) with standard operators per kind; `field.operators` may restrict/relabel.
- [x] Draft never emits an incomplete clause — `Add`/`Update` disabled until `isClauseComplete`. Verified headlessly: `Add` disabled while empty, enabled after a valid operand, then commits.
- [x] `Match all` / `Match any` combinator shown only with 2+ clauses.
- [x] Pills via the extended `SelectionSummary` (`onActivate` + `onRemove`) — separate edit/remove controls, no nested buttons.
- [x] `allowMultiple` governs duplicate-field clauses (default single); `maxClauses` hides the add row when reached.
- [x] Controlled `value` + `onChange`; external replacement re-renders.
- [x] Clause labels from the shared model, e.g. `Format is any of CLAP, VST3`, `Hidden is false`, `Tag count at least 3` (verified in-browser).

React is interface-invariant with Svelte (same props/types/behavior, own local type + model copy per repo convention). Both web targets verified headlessly (render, popover, combinator, draft-gating add, pill edit prefill, remove) with zero console errors.

## GPUI gap (vs Svelte + contract)

- [x] Trigger (FILTER label + summary + count badge + chevron), ghost `×` reset, clause pills (label + xs ghost remove IconButton), anchored surface (combinator two-option indicator + add-field `Select` + "No filters"), sizes/densities from the spec token methods and size table — all resolve from `FilterBuilderSpec` tokens (`count_fill_token`, `muted_color_token`, `surface_radius_token`, …). Build-verified (`cargo check`).
- [ ] Interactive draft editing (field → operator → operand editor → Add), pill edit-on-activate, and anchored-popover positioning are not wired in the component — preview event-loop work, render-only build-verified posture (same as OrderBy). The combinator renders as a static selected/unselected two-option indicator rather than an interactive SegmentedControl (no GPUI SegmentedControl primitive).
- accepted: no ARIA (GPUI has no accessibility API) — accessible-name intent documented only.
- accepted: anchored-dropdown positioning is platform-owned; surface renders inline below the trigger.

## Jetstream gap (vs Svelte + contract)

- [x] Same anatomy as GPUI built from `JsEl` (`js_filter_builder`): trigger + count badge + reset + pills + surface (combinator + `js_select` add-field + "No filters"), size/density tables, all colors/radii from spec token methods. Build-verified (`cargo check`).
- [ ] Interactive draft editing / pill activate / anchored positioning not wired — preview event-loop work, render-only build/probe-verified posture. Combinator is a static two-option indicator.
- accepted: no ARIA channel; interaction would live in the preview event loop.

`#[cfg(test)] mod tests` render_probe assertions are authored in `filter_builder.rs` (empty → "No filters" + "FILTER" + "Filter" placeholder; populated → "Format is any of CLAP, VST3" / "Hidden is false" pill labels + "2 filters" count; combinator visible only with 2+ clauses). **Note:** these probe tests cannot currently execute in this workspace — the `poodle-jetstream-components` *test* target fails to compile from pre-existing, unrelated breakage in `src/presentation/metrics_c.rs` (and the sibling `jetstream-runtime` test build), independent of FilterBuilder. The render code itself is `cargo check`-clean.

## Specimen parity

- Svelte covers: controlled builder with live JSON, Match any, Empty, overflowing pills + repeated field (allowMultiple), Max 2 clauses, Disabled, Sizes snippet (xs–xl), Densities snippet. (`FilterBuilderSpecimen.svelte`)
- React covers: the same set, one-to-one (`FilterBuilderSpecimen.tsx`), same `#components/filter-builder` route for side-by-side diffing.
- GPUI covers: filter builder (open, 3 clauses, Match all), Disabled (open), Sizes, Densities. — missing: a Match-any and an empty-open example (broad otherwise).
- Jetstream covers: filter builder (open, Match all), Empty (open), Disabled, Sizes (xs–xl), Densities (compact/default/comfortable).

## Notes

- **`showCombinator` opt-in + mode-reflecting label (2026-07-15).** The `Match all`
  / `Match any` root combinator is now off by default and shown only when a host
  opts in via `showCombinator` (and 2+ clauses exist). Most filter sets are AND-only,
  so the toggle was irrelevant noise. The expression still carries a `combinator`
  field (defaults `"and"`); the flag gates only the UI switch. Wired through all
  targets (`FilterBuilderSpec::show_combinator` gates `combinator_visible()`, which
  GPUI + Jetstream already consume). **To make the mode legible** (users, incl. the
  author, misread the popover-only toggle), the always-visible opener label now
  reads the live mode — "All" / "Any" — instead of "Filter" when the combinator is
  in effect (`FilterBuilderSpec::opener_label()`), rendered a touch more prominent
  (`data-combinator`, text-primary). Verified headless: opt-in specimen label reads
  "ALL", flips to "ANY" on toggle; default specimen reads "FILTER" and shows no
  toggle. Note: web uppercases the label via CSS; native renders the semantic
  title-case ("All"/"Any"/"Filter") — accepted build-verified delta.
- **Single-block layout refinement (2026-07-15).** Trigger + clause pills now live
  in one bordered field (pills flow inline, filling the row, wrapping only when
  needed) instead of a trigger row with pills stacked beneath. De-duplicated the
  count: the count badge is kept as the single indicator, right-aligned in a
  trailing group with the reset, and the opener "N filters" summary text is
  suppressed while pills show. De-duplicated the clear: dropped the SelectionSummary
  "Clear" link; one trailing reset × remains. Pills render inline reusing
  SelectionSummary's split-chip classes (single CSS source); because the classes
  are used outside their usual `.poodle-selection-summary` root, the field supplies
  the chip CSS variables (font/padding/min-height per size + density) so pill
  sizing scales correctly. Mirrored across Svelte + React (both re-verified
  headless: pills inline at correct chip font, count badge present, no summary dup,
  single clear, zero console errors) and GPUI + Jetstream (build-verified; Jetstream
  probe tests updated to assert inline pills + suppressed count text).
- **New component (2026-07-15).** Companion to OrderBy. The pure model (`default_operators_for_kind`, `isClauseComplete`, `clauseLabel`, `cloneOperand`, …) is authored once in TS (`filter-builder-model.ts`), mirrored verbatim in React, and re-implemented as `FilterBuilderSpec` methods in Rust; `poodle-specs` carries 6 unit tests for it.
- **SelectionSummary extension:** additive `onActivate` splits each chip into a separate activation button + remove IconButton (no nested buttons). Web-only for now; Rust SelectionSummary is unchanged (FilterBuilder's Rust targets render their own pills). See `docs/parity/selection-summary.md`.
- **Bug found + fixed during Svelte verification:** `editClause` used `structuredClone` on a reactive `$state` operand, which throws; replaced with a `cloneOperand` helper in the shared model.
- Preview-loop (not closed, both Rust targets): popover open, draft field/operator/operand editing, Add/Update/Cancel, pill activate/remove, combinator toggle. Accepted render-only posture.
- Contrast axis: CSS-only; Rust artifacts stay literal (no change here).
