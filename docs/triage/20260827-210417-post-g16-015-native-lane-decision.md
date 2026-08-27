# Post-g16.015 Native Lane Decision

Status: resolved — selected as g16.016; merged in PR #91
Captured: 2026-08-27
Resolved: 2026-08-27
Source: merged `g16.015` / PR #90 and the 44 mounted / 130 missing ledger

## Decision

Select Pagination as the next bounded native behavior lane.

Pagination already has aligned Svelte and React behavior, a detailed contract,
one shared Rust renderer, a live GPUI specimen, and the node/backend channels
needed for pointer, keyboard, controlled rebuild, disabled, and composed Select
interaction. It therefore admits one honest mounted-behavior card without a
public API break or a new generic rendering capability.

The concrete repair is also measured: `is_loading` disables page buttons but
the wired page-size Select is built without the same disabled state. A loading
Pagination can still open that selector and report a limit change. The next
card must close that leak and prove the navigation and page-size paths through
the mounted production tree.

## Alternatives Kept Separate

- `NumberInput` remains blocked on the raw-draft/value-model decision in
  `20260826-213343-number-input-native-value-model.md`.
- `EditableLabel` has mounted commit/cancel evidence, but full closure still
  needs a decision about default double-click activation, select-on-focus, and
  focus restoration. The current node activation channel cannot distinguish a
  single activation from a double click.
- `Rating` needs a clean Rust API migration before mounted proof: the web
  authority is nullable and fractional with a default `step=0.5`, while the
  Rust spec stores concrete `f64`, defaults to whole steps, retains legacy
  precision, and reports `u32` changes. Do not disguise that as a test-only
  gap.
- `Select` itself is an overlay-sized behavior lane. Pagination may exercise
  its existing composed limit-selector path, but this does not close Select's
  own ledger row or authorize a generic Select redesign.
- SplitButton, visual comparison, broad native accessibility, and Jetstream
  admission remain separate programmes.

## Fixed Boundary

`g16.016` owns Pagination page-request semantics, loading suppression,
host-owned rebuilds, the existing wired page-size Select path, one named
mounted GPUI regression, and exactly one ledger-cell move. It does not change
the public web or Rust APIs, remove legacy Pagination aliases, extend generic
node vocabulary, claim native accessibility or visual parity, or move Select's
evidence cell.
