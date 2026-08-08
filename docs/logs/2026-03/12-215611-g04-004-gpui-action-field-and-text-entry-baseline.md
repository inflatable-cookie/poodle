---
title: g04.004 gpui action field and text-entry baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, forms]
---

## Summary

Completed `g04.004` by widening `pug-gpui-primitives` beyond structural layout
into the first real GPUI action, field-wrapper, and text-entry primitive
baseline.

## What changed

- added the normative baseline `docs/specs/051-gpui-action-text-entry-and-field-primitives-baseline.md`
- completed `docs/roadmaps/g04/004-gpui-action-text-entry-and-field-primitives.md`
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
- added shared enums for button variant, control size, validation state, and
  form-action alignment
- pinned token-backed button, validation, field-message, and text-entry
  semantics inside the Rust crate
- added crate tests for loading or disabled suppression, field message
  precedence, controlled or uncontrolled value posture, search clear-action
  behavior, and form-action alignment
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  action or field baseline artifact is machine-checked
- rolled the package and roadmap surfaces forward to `g04.005`

## Validation

- `cargo fmt --manifest-path packages/gpui/primitives/Cargo.toml`
- `cargo check --manifest-path packages/gpui/primitives/Cargo.toml`
- `cargo test --manifest-path packages/gpui/primitives/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.004` is now explicit. Pug has a GPUI primitive baseline for buttons,
fields, inputs, search, and form-action layout that later form and composite
work can reuse without restating value, validation, or command semantics.

## Next

Open `g04.005` and implement the GPUI selection, value, feedback, and
date-time primitive tranche on top of the widened GPUI primitive crate.
