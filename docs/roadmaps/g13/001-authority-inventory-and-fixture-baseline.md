# g13.001 Authority Inventory And Fixture Baseline

Status: ready
Owner: Poodle core
Depends on: `docs/specs/063-rust-authored-component-and-scene-ir.md`

## Objective

Freeze what is authoritative before creating an IR crate. Measure the current
component, specimen, registry, contract, and backend duplication and capture
the pilot fixtures that later cards must preserve.

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

- No implementation package is created in this card.
- Every pilot fixture has a stable identifier and owning contract.
- Unknown authority or runtime behavior becomes a named decision, not an
  inferred compiler feature.
- `effigy docs:check` is green before schema implementation begins.

## Next

`g13.002` implements only the schema proven necessary by this inventory.
