---
title: g04.007 gpui form validation and remediation composite baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, composites, forms]
---

## Summary

Completed `g04.007` by introducing `pug-gpui-composites` and freezing the
first GPUI form, validation, and remediation composite baseline.

## What changed

- added the normative baseline `docs/specs/054-gpui-form-validation-and-remediation-composite-baseline.md`
- completed `docs/roadmaps/g04/007-gpui-form-validation-and-remediation-composite-parity.md`
- added the machine-readable artifact `packages/gpui/form-validation-remediation-composites-baseline.json`
- introduced `packages/gpui/composites` with:
  - `FormShellSpec`
  - `ValidationSummarySpec`
  - `RemediationBannerSpec`
  - `InlineRemediationSpec`
- added shared GPUI composite support types for announcement mode,
  field-state inventories, remediation actions, and validation-summary entries
- pinned grouped field, blocking or pending validation, action-row
  coordination, and persistent remediation posture inside the GPUI composite
  layer so later shells inherit the same contract semantics as Svelte
- added crate tests for invalid or pending form posture, validation-summary
  announcement behavior, remediation-banner action structure, and
  inline-remediation field references
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  composite baseline artifact is machine-checked
- updated release metadata, package surfaces, and roadmap status so the repo
  now points at `g04.008`

## Validation

- `cargo fmt --manifest-path packages/gpui/composites/Cargo.toml`
- `cargo check --manifest-path packages/gpui/composites/Cargo.toml`
- `cargo test --manifest-path packages/gpui/composites/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.007` is now explicit. Pug has a first real GPUI composite crate and a
shared baseline for form-shell, validation-summary, and persistent inline
remediation posture, which clears the way for broader data and shell composite
parity.

## Next

Open `g04.008` and implement the GPUI data, browse, detail, picker, and media
composite parity tranche on top of the widened primitive and composite
surface.
