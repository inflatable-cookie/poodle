# g13.001 Authority Inventory And Fixture Baseline

Status: in progress
Owner: Poodle core
Depends on: `docs/specs/063-rust-authored-component-and-scene-ir.md`
Governing rules: `IR-01`–`IR-12`

## Objective

Freeze what is authoritative before creating an IR crate. Measure the current
component, specimen, registry, contract, and backend duplication and capture
the pilot fixtures that later cards must preserve.

## Execution Plan

- [x] `g13-b001` — authority inventory and inherited docs-baseline repair
  (merged `251cc858`; [review log](../../logs/2026-08/11-g13-b001-b005-review-and-merge.md))
- [ ] `g13-b002` — pilot fixture and quantitative-metrics freeze
  (ready; dispatchable)
- [ ] `g13-b003` — maintainer crate-placement ruling and executable g13.002
  handoff

Parallel research batch `g13-b005` is merged (`2f8dc5db`); its 129-requirement
corpus is the acceptance input for the `g13.002` schema card. Prior-art batch
`g13-b004` is dead and must be recompiled onto a different model before any new
dispatch; it does not block `g13-b002`.

The merged inventory now answers "map every current definition surface" and
"record direct Jetstream render paths and preview compatibility layers".
Crate/package placement for `poodle-ir` and `poodle-codegen` remains **unruled**
— `g13-b001` supplied evidence only, by design, and `g13-b003` owns the ruling.

## Deliverables

- Map every current definition surface and generated/manual boundary across
  Svelte, React, `poodle-render`, GPUI, and Jetstream.
- Resolve crate/package placement for `poodle-ir` and `poodle-codegen` against
  publication and workspace constraints.
- Record existing direct Jetstream render paths and preview compatibility
  layers as explicit migration debt.
- Freeze Button, RangeSlider, TextInput, preview-header, and specimen-axis
  semantic/interaction/accessibility fixtures.
- Define quantitative pilot measures: authored LOC, generated LOC, extension
  count, build time, diagnostic quality, and four-runtime drift count.
- Repair the inherited docs baseline: AgentSubagent usage coverage; Keyboard,
  ModMatrixGrid, and WaveformDisplay contract/preview inventory; and stale
  shared-demo export/preview counts.

## Acceptance

- [x] No implementation package is created in this card.
- [ ] Every pilot fixture has a stable identifier and owning contract.
      (`g13-b002`)
- [ ] Unknown authority or runtime behavior becomes a named decision, not an
  inferred compiler feature. (named below; ruled by `g13-b003`)
- [x] `effigy docs:check` is green before schema implementation begins.
      (exit 0 on merged `main`, 2026-08-11)

## Open Decisions Carried Into b002/b003

Named by the merged batches, deliberately unresolved. None may be settled
implicitly by a fixture or an IR representation.

- `UNKNOWN-01` — does `range-slider.md` §6's "`aria-orientation` NOT set"
  extend to the embedded variant's `role="slider"` stops? Svelte and React emit
  it there; the contract is silent. Maintainer decision, `g13-b003`.
- `UNKNOWN-02` — are `ButtonVariant::Danger` and `ButtonTone::Success`
  in-scope vocabulary, or do they need a `button.md` §3 contract change under
  `IR-09`? Maintainer decision, `g13-b003`.
- Jetstream `component_registry.rs` declares generator derivation from the
  Svelte registry but no generator exists. Candidate codegen target,
  `g13-b003`.
- Evidence gaps `GAP-01`–`GAP-07` (missing range/text conformance vectors, no
  executed native AT traces, native vertical RangeSlider, GPUI per-thumb focus,
  Jetstream TextInput typing events, contract-silent Button density values,
  `truncate`/`fit`/`maxWidth` absent from `ButtonSpec`) are `g13-b002` baseline
  inputs. A gap may be recorded as a measured zero; it may not be closed by
  inventing evidence.

## Next

`g13.002` implements only the schema proven necessary by this inventory.
