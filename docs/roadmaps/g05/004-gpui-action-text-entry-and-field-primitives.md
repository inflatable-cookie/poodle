# g05.004 GPUI Action, Text-Entry, And Field Primitives

Status: completed
Owner: Poodle Core
Updated: 2026-03-12
Depends on: g05.001, g05.002, g05.003
Primary repos: `poodle`

## Goals

- [x] implement the core GPUI action and text-entry primitives
- [x] keep field semantics aligned with the existing Poodle contracts rather than
  ad hoc native widget assumptions

## Execution Checklist

- [x] implement button, icon-button, field, text-input, text-area, search, and
  form-action primitives in GPUI
- [x] align disabled, loading, validation, and sizing behavior to the current contracts
- [x] document text-input and field deltas where GPUI behaves differently from
  browser inputs
- [x] verify these primitives are sufficient to unblock form and composite work

## Acceptance Criteria

- [x] GPUI action and text-entry primitive posture is explicit
- [x] field and form-foundation parity posture is explicit

## Completed Work

- added the normative baseline `docs/specs/051-gpui-action-text-entry-and-field-primitives-baseline.md`
- added the machine-readable artifact `packages/gpui/action-field-primitives-baseline.json`
- expanded `packages/gpui/primitives` with:
  - `ButtonSpec`
  - `IconButtonSpec`
  - `FieldSpec`
  - `FieldRelationships`
  - `TextInputSpec`
  - `TextAreaSpec`
  - `SearchFieldSpec`
  - `FormActionsSpec`
- added shared GPUI primitive enums for control size, button variant, validation state, and form-action alignment
- froze token-backed button, input, field, validation, and action-row semantics inside the Rust crate rather than leaving them as future placeholder names
- added crate tests for disabled/loading suppression, field message precedence, controlled or uncontrolled input meaning, search clear-action posture, and form-action alignment
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI action/field baseline artifact is machine-checked
- updated package and roadmap surfaces so the repo now points at `g05.005`

## Next Task

Open `g05.005` and implement the GPUI selection, value, feedback, and
date-time primitive tranche.
