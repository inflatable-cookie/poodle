# g05.007 GPUI Form, Validation, And Remediation Composite Parity

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g05.004, g05.005, g05.006
Primary repos: `flint`

## Goals

- [x] implement the GPUI composite layer for form, validation, and remediation flows
- [x] prove the shared contracts still hold once GPUI moves above primitive depth

## Execution Checklist

- [x] implement GPUI form wrappers, validation shells, action rows, banners,
  and remediation patterns where contracts already exist
- [x] align error, pending, disabled, and remediation posture to the shared contracts
- [x] document native-runtime differences in announcement and validation exposure
- [x] verify the composite layer is usable by later data and workstation tranches

## Acceptance Criteria

- [x] GPUI form composite posture is explicit
- [x] GPUI remediation and validation parity posture is explicit

## Completed Work

- added the normative baseline `docs/specs/054-gpui-form-validation-and-remediation-composite-baseline.md`
- added the machine-readable artifact `packages/gpui/form-validation-remediation-composites-baseline.json`
- introduced `packages/gpui/composites` as the first public-intent GPUI composite crate with:
  - `FormShellSpec`
  - `ValidationSummarySpec`
  - `RemediationBannerSpec`
  - `InlineRemediationSpec`
- added shared GPUI composite support types for announcement mode, field-state inventories, remediation actions, and validation-summary entries
- froze grouped field, blocking or pending validation, action-row coordination, and persistent remediation posture inside the GPUI composite layer so later shells inherit the same contract semantics as Svelte
- added crate tests for invalid or pending form posture, validation-summary announcement behavior, remediation-banner action structure, and inline-remediation field references
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI composite baseline artifact is machine-checked
- updated release metadata, package surfaces, and roadmap status so the repo now points at `g05.008`

## Next Task

Open `g05.008` and implement the GPUI data, browse, detail, picker, and media
composite tranche.
