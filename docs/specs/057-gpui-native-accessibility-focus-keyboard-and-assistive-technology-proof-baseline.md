# 057 GPUI Native Accessibility, Focus, Keyboard, And Assistive-Technology Proof Baseline

Status: active
Updated: 2026-03-12
Depends on: `043-accessibility-audit-and-cross-runtime-delta-handling-baseline.md`, `048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`, `049-gpui-theme-runtime-and-native-preview-app-baseline.md`, `050-gpui-structural-primitives-baseline.md`, `051-gpui-action-text-entry-and-field-primitives-baseline.md`, `052-gpui-selection-feedback-and-date-time-primitives-baseline.md`, `053-gpui-overlay-disclosure-navigation-and-menu-primitives-baseline.md`, `054-gpui-form-validation-and-remediation-composite-baseline.md`, `055-gpui-data-browse-detail-picker-and-media-composite-baseline.md`, `056-gpui-workstation-shell-command-and-layout-baseline.md`

## Purpose

Freeze the first honest GPUI accessibility-proof posture now that Flint has real
primitive, composite, and workstation crates instead of only a token surface
and browser-side accessibility audit.

This milestone does not certify complete native conformance. It makes the
current GPUI focus, keyboard, state-exposure, announcement, and
assistive-technology posture explicit so later parity and downstream-adoption
claims stop leaning on browser inference.

## Core Rule

GPUI accessibility claims must be machine-readable, section aligned, and honest
about the difference between explicit API posture, crate-test evidence, and
mounted native assistive-technology proof.

The repo may say that the current GPUI surface has explicit accessibility
posture. It may not say that every implemented GPUI section already has full
native conformance proof.

## Required Proof Artifact

The current generation must maintain a machine-readable proof artifact at:

- `packages/gpui/native-accessibility-proof.json`

The artifact must record:

- the GPUI review sections that now have explicit native accessibility posture
- per-layer focus-entry, focus-recovery, keyboard, state-exposure,
  announcement, and assistive-technology status
- evidence already earned from public APIs and crate tests
- remaining blockers before any section can be promoted beyond manual proof
- the current cross-runtime delta register and non-goals

## Section Rule

The section proof in `packages/gpui/native-accessibility-proof.json` must stay
aligned to the same shared review sections used by the GPUI priority matrix and
the browser accessibility audit surface:

- `form-suite`
- `table-suite`
- `browse-suite`
- `detail-suite`
- `picker-suite`
- `media-suite`
- `notification-suite`
- `command-suite`
- `workspace-suite`

Preview-only docs utilities such as `catalog-hub`, `token-summary-section`, and
`token-inspector` remain outside this shared native proof list.

## Evidence Rule

The current proof posture recognizes three evidence depths:

- explicit API posture
- crate-test evidence
- mounted native manual review

`explicit API posture` means accessible naming, focus ownership intent,
validation exposure, check state, overlay dismissal posture, layout-region
identity, or announcement mode are part of the public GPUI contract surface.

`crate-test evidence` means those semantics are asserted in Rust tests instead
of being left as comments or downstream assumptions.

`mounted native manual review` means the repo has runnable GPUI review surfaces
or downstream shells that can be inspected for actual focus movement,
announcement timing, or assistive-technology traces.

The current baseline still leaves many section-level claims at the third stage
as manual work rather than completed proof.

## Focus And Keyboard Rule

The GPUI accessibility proof must explicitly cover:

- focus entry into overlays, shells, and major review surfaces
- focus recovery after close, dismiss, execution, or state transitions
- keyboard traversal for controls, tables, pickers, command lists, tabs, and
  workstation shells
- state exposure for validation, selection, activity, urgency, and active
  surface identity

If a surface only freezes intent in a spec and test but does not yet have a
mounted native review route, that limitation must stay explicit in the proof
artifact.

## Announcement Rule

The proof baseline must treat announcements as distinct from focus and keyboard
coverage.

Validation summaries, remediation banners, notifications, status bars, picker
state changes, and command execution feedback may only be treated as explicit
native announcement proof when the repo has more than browser analogy or
crate-level role names. Until then, their GPUI status should remain `manual`
or `hybrid` instead of overclaimed.

## Assistive-Technology Honesty Rule

Flint may say:

- accessible naming and state exposure are explicit in the current GPUI API
- crate tests cover part of the current native accessibility posture
- mounted native assistive-technology proof remains manual for many sections

Flint may not say:

- the presence of GPUI crates alone proves final assistive-technology
  conformance
- browser preview evidence closes native focus or announcement debt
- direct-parity visual targets automatically have complete native accessibility
  proof

## Seed Evidence

- `packages/gpui/native-accessibility-proof.json`
- `packages/svelte/preview/src/accessibility.ts`
- `packages/svelte/preview/scripts/build-accessibility-report.ts`
- `packages/svelte/preview/artifacts/accessibility-report.json`
- `packages/gpui/primitives/README.md`
- `packages/gpui/composites/README.md`
- `packages/gpui/workstation/README.md`
- `docs/roadmaps/g04/010-gpui-native-accessibility-focus-keyboard-and-assistive-technology-proof.md`

## Next Task

Carry this explicit native accessibility proof posture into `g04.011` so the
cross-runtime parity report, intentional delta register, and acceptance-harness
expansion build on named accessibility debt rather than implied readiness.
