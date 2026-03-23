# 051 GPUI Action, Text-Entry, And Field Primitives Baseline

Status: active
Updated: 2026-03-12
Depends on: `050-gpui-structural-primitives-baseline.md`

## Purpose

Freeze the GPUI primitive tranche that unblocks form, remediation, and
shell-level input work. This baseline defines the first action, field-wrapper,
and text-entry primitive semantics in `flint-gpui-primitives` so later GPUI
composites build on the same button, field, and text-entry meaning as Svelte.

## Package Rule

The `g04.004` tranche extends `flint-gpui-primitives` with:

- `ButtonSpec`
- `IconButtonSpec`
- `FieldSpec`
- `FieldRelationships`
- `TextInputSpec`
- `TextAreaSpec`
- `SearchFieldSpec`
- `FormActionsSpec`

These exports belong to the same preview-channel public-intent Rust crate as
the structural baseline from `g04.003`.

## Contract Coverage Rule

The crate must stay aligned to the existing foundation contracts for:

- `button`
- `icon-button`
- `field`
- `text-input`
- `text-area`
- `search-field`
- `form-actions`

This tranche should preserve contract-first semantics before later GPUI
selection, feedback, and date-time work widens the primitive surface further.

## Form-Foundation Rule

This baseline freezes the reusable form-foundation slice needed by later GPUI
composite work:

- button and icon-button activation suppression while disabled or loading
- field label, description, invalid-message, and pending-message relationships
- text-input and text-area controlled or uncontrolled value posture
- search-specific query and clear-action semantics
- form-action row grouping and alignment posture

Later GPUI form composites should build on these primitives instead of
redefining button or field behavior locally.

## Runtime Honesty Rule

This baseline remains honest about current depth:

- spec-level value, validation, and accessibility relationships are explicit
- token-backed visual semantics are explicit
- mounted GPUI widget behavior, native event plumbing, and full accessibility
  proof still belong to later `g04` milestones

The repo may expose these primitives as contract-backed specs before every one
of them is rendered by a fully mounted GPUI control implementation.

## Token Rule

Action, field, and text-entry primitives must resolve from `flint-gpui-tokens`
for at least:

- control size, min width, and icon size
- focus ring width and color
- surface or field fill and border roles
- validation-state border emphasis
- label and supporting-text typography roles
- disabled opacity treatment

## Controlled-Value Rule

`TextInputSpec`, `TextAreaSpec`, and `SearchFieldSpec` must preserve the same
controlled or uncontrolled meaning as the shared contracts:

- explicit controlled values take precedence over defaults
- uncontrolled defaults remain stable initial state
- invalid and busy semantics remain derived from explicit validation posture,
  not from visual styling alone

## Seed Evidence

- `packages/gpui/action-field-primitives-baseline.json`
- `packages/gpui/primitives/README.md`
- `packages/gpui/primitives/src/lib.rs`
- `packages/gpui/primitives/src/button.rs`
- `packages/gpui/primitives/src/icon_button.rs`
- `packages/gpui/primitives/src/field.rs`
- `packages/gpui/primitives/src/text_input.rs`
- `packages/gpui/primitives/src/text_area.rs`
- `packages/gpui/primitives/src/search_field.rs`
- `packages/gpui/primitives/src/form_actions.rs`
- `docs/contracts/foundation/button.md`
- `docs/contracts/foundation/icon-button.md`
- `docs/contracts/foundation/field.md`
- `docs/contracts/foundation/text-input.md`
- `docs/contracts/foundation/text-area.md`
- `docs/contracts/foundation/search-field.md`
- `docs/contracts/foundation/form-actions.md`

## Next Task

Carry this GPUI form-foundation baseline into `g04.005`, adding selection,
value, feedback, and date-time primitives on top of the same crate and token
posture.
