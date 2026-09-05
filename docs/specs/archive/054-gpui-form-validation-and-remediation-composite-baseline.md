# 054 GPUI Form, Validation, And Remediation Composite Baseline

Status: active
Updated: 2026-03-12
Depends on: `051-gpui-action-text-entry-and-field-primitives-baseline.md`, `052-gpui-selection-feedback-and-date-time-primitives-baseline.md`, `053-gpui-overlay-disclosure-navigation-and-menu-primitives-baseline.md`

## Purpose

Freeze the first GPUI composite layer above the widened primitive substrate.
This baseline adds reusable form-shell, validation-summary, and remediation
semantics so later GPUI browse, detail, picker, and workstation tranches do
not reinvent field grouping, validation exposure, or persistent inline
recovery patterns.

## Package Rule

The `g04.007` tranche introduces `poodle-gpui-composites` with:

- `FormShellSpec`
- `ValidationSummarySpec`
- `RemediationBannerSpec`
- `InlineRemediationSpec`

These exports become preview-channel public-intent GPUI composites, parallel
to the existing Svelte composite package posture.

## Contract Coverage Rule

The crate must stay aligned to the existing shared contracts and rules for:

- `field`
- `form-actions`
- `callout`
- `banner`
- `009-form-shell-validation-and-action-row-rules.md`
- `../015-loading-empty-error-notification-and-remediation-rules.md`

## Form Shell Rule

This baseline freezes the reusable form-shell posture that later GPUI
composites must inherit:

- grouped fields remain explicit through sections
- invalid and pending counts remain derivable from field-level state
- parent-owned busy and disabled posture stays explicit
- action-row layout intent stays explicit rather than implicit

The composite layer must reuse primitive validation semantics rather than
introducing a new form-state vocabulary.

## Validation Summary Rule

Validation summaries must keep review and recovery information coherent:

- invalid entries remain blocking
- pending entries may be included explicitly rather than silently replacing
  invalid messaging
- announcement urgency stays explicit through an announcement mode
- summary entries reference concrete field ids rather than generic text alone

## Remediation Rule

This baseline also freezes the persistent remediation posture above the field:

- strong inline remediation uses a banner-style summary
- lighter remediation uses an inline callout-style summary
- remediation actions stay adjacent to the affected surface
- remediation summaries remain textual, not icon-only

This keeps recoverable errors, warnings, and pending coordination visible in
the same UI area instead of scattering them between form actions and unrelated
status regions.

## Runtime Honesty Rule

This tranche remains explicit about current depth:

- validation counts, announcement posture, field references, and remediation
  action structure are explicit
- mounted native banner widgets, real accessibility event wiring, and full
  downstream task-flow proof still belong to later `g04` milestones

The repo may expose these composites as contract-backed GPUI specs before
every one of them is rendered as a mounted native shell.

## Token Rule

Form, validation, and remediation composites must continue resolving from the
existing GPUI token and primitive baselines for at least:

- stack and inline spacing roles
- panel and status surface roles
- action-row alignment and button variant roles
- accent and danger or warning emphasis roles
- announcement and validation state posture inherited from the primitive layer

## Seed Evidence

- `packages/gpui/form-validation-remediation-composites-baseline.json`
- `packages/gpui/composites/Cargo.toml`
- `packages/gpui/composites/README.md`
- `packages/gpui/composites/src/lib.rs`
- `packages/gpui/composites/src/form_shell.rs`
- `packages/gpui/composites/src/validation_summary.rs`
- `packages/gpui/composites/src/remediation_banner.rs`
- `packages/gpui/composites/src/inline_remediation.rs`
- `packages/gpui/composites/src/types.rs`
