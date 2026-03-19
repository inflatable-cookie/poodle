# Pug GPUI Composites

Contract-backed GPUI composite baseline for Pug.

## Public Surface

- crate: `pug-gpui-composites`
- current form, validation, and remediation tranche:
  - `FormShellSpec`
  - `ValidationSummarySpec`
  - `RemediationBannerSpec`
  - `InlineRemediationSpec`
- current data, browse, detail, picker, and media tranche:
  - `DataTableSpec`
  - `DetailShellSpec`
  - `FilterToolbarSpec`
  - `PaginationSummarySpec`
  - `EmptyStateSpec`
  - `PickerShellSpec`
  - `RelationPickerSpec`
  - `SelectionSummarySpec`
  - `MediaThumbnailSpec`
  - `MediaPreviewSpec`
- shared support types:
  - `AnnouncementMode`
  - `AspectRatio`
  - `BrowseState`
  - `DetailState`
  - `EmptyStateVariant`
  - `FormActionLayout`
  - `FormFieldState`
  - `FormSectionSpec`
  - `FormStatusSummary`
  - `MediaKind`
  - `MediaState`
  - `MinColumnWidth`
  - `PickerItemSpec`
  - `PickerVariant`
  - `RemediationAction`
  - `ScrollOwner`
  - `SelectionMode`
  - `SelectionSummaryItem`
  - `TableColumnSpec`
  - `TableRowSpec`
  - `TableSortDirection`
  - `ValidationSummaryEntry`

## Current Posture

- this crate now carries the `g04.007` form, validation, and remediation
  baseline plus the `g04.008` data, browse, detail, picker, and media
  baseline
- `g04.010` now makes the native accessibility, focus, keyboard, and
  assistive-technology posture for these shared composite surfaces explicit in
  `packages/gpui/native-accessibility-proof.json`
- it intentionally freezes reusable shell semantics and remediation posture
  before the repo contains the full GPUI composite catalogue
- later GPUI composite and workstation tranches should build on these shared
  semantics rather than introducing workflow-local validation or banner models

## Dependency Rule

- `pug-gpui-composites` depends on `pug-gpui-primitives` for shared validation,
  button, action-row, status, selection, and overlay semantics
- it resolves spacing and surface roles from `pug-gpui-tokens`
- composite semantics should stay aligned to documented contracts rather than
  runtime-local widget assumptions

## Non-Goals

- this crate does not yet prove mounted GPUI rendering parity for composite
  shells
- this crate does not yet prove mounted GPUI table semantics, virtualization,
  picker keyboarding, media renderer behavior, or workstation-shell depth
- this crate does not treat form-shell and remediation specs as proof that
  native accessibility or announcement plumbing is complete

## Next Task

Use this widened GPUI composite baseline and the explicit native accessibility
proof posture while executing `g04.011`, hardening the cross-runtime parity
report, intentional delta register, and acceptance-harness expansion.
