# 058 Cross-Runtime Parity Report, Delta Register, And Acceptance-Harness Expansion

Status: active
Updated: 2026-03-12
Depends on: `../025-parity-automation-and-harness-boundary.md`, `045-ecosystem-acceptance-and-long-tail-regression-baseline.md`, `048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`, `057-gpui-native-accessibility-focus-keyboard-and-assistive-technology-proof-baseline.md`

## Purpose

Freeze the first cross-runtime parity evidence surface that treats Svelte and
GPUI as one shared review problem instead of leaving GPUI evidence scattered
across baseline artifacts and milestone logs.

This milestone exists so `g04` can move from “GPUI packages now exist” to an
explicit statement of:

- which shared sections currently have real cross-runtime evidence
- which sections are still direct-parity targets versus native-adaptation
  targets
- which deltas are intentionally allowed, revisited, or still pending
- and how the ecosystem acceptance matrix now reflects those GPUI surfaces

## Core Rule

Cross-runtime parity evidence must be machine-readable, section aligned, and
honest about both intentional deltas and remaining manual review.

Poodle should no longer talk about GPUI parity only through Svelte preview routes
or only through crate-level GPUI baselines. The current generation needs one
artifact that joins those surfaces together.

## Required Report Artifact

The current generation must maintain a machine-readable cross-runtime report at:

- `packages/gpui/cross-runtime-parity-report.json`

That report must record:

- the shared review sections currently participating in GPUI parity evidence
- direct-parity versus native-adaptation posture by section
- side-by-side review targets and the Svelte routes used for comparison
- GPUI evidence artifacts attached to each section
- a first-class intentional delta register
- acceptance-harness alignment for the current GPUI suite

## Section Scope Rule

The current cross-runtime report must stay aligned to the shared GPUI section
set already used by the priority matrix and native accessibility proof:

- `form-suite`
- `table-suite`
- `browse-suite`
- `detail-suite`
- `picker-suite`
- `media-suite`
- `notification-suite`
- `command-suite`
- `workspace-suite`

Preview-only docs utilities remain outside this report because they are not the
main Loophole-facing shared UI target.

## Delta Register Rule

Intentional deltas must no longer be hidden inside baseline JSON files or
milestone summaries.

The cross-runtime report must contain a dedicated `deltaRegister` where each
entry records:

- a stable id
- a title
- one of the existing approval states: `pending`, `allowed`, `revisit`, or
  `rejected`
- the affected section ids
- a concrete runtime-specific reason
- named follow-up work
- linked evidence artifacts

Undocumented cross-runtime deltas remain defects even when the runtime
difference is understandable.

## Side-By-Side Review Rule

For sections marked as side-by-side review targets by the GPUI priority matrix,
the current report must record:

- the Svelte route ids used for repeat comparison
- the GPUI evidence artifacts currently standing in for native review
- a short comparison posture describing what should stay materially the same

This does not require a mounted GPUI route for every section today. It does
require an explicit bridge between the current Svelte review surface and the
current GPUI evidence.

## Acceptance-Harness Expansion Rule

The ecosystem acceptance matrix must reflect the widened GPUI evidence surface.

That means the current GPUI suite in `packages/ecosystem-acceptance.json`
should no longer look token-only. It must carry:

- the current GPUI package set
- the cross-runtime parity report
- the native accessibility proof
- the current blockers that still keep the suite below downstream adoption
  proof

The suite may remain `matrix-only` if mounted GPUI harness depth is still
missing, but it must not ignore the current component and parity evidence.

## Generated Parity Artifact Rule

The existing generated preview parity report may remain Svelte-route based, but
it should expose the current cross-runtime summary so the repo no longer has
two disconnected parity stories.

The generated artifact in:

- `packages/svelte/preview/artifacts/parity-report.json`

should therefore carry a summary view of:

- the current GPUI cross-runtime section counts
- the current side-by-side section set
- the current intentional deltas
- the current GPUI acceptance-harness alignment

## Honesty Rule

Poodle may say:

- cross-runtime parity posture is explicit
- some sections are direct-parity targets while others are native-adaptation
  targets
- some deltas are intentionally allowed or scheduled for revisit
- the GPUI acceptance suite now includes component and parity evidence

Poodle may not say:

- GPUI parity is complete because the Svelte preview has stable routes
- GPUI parity is complete because spec-level crates exist
- intentional deltas are acceptable without being registered explicitly
- the GPUI acceptance suite is runnable if it is still only artifact-backed

## Seed Evidence

- `packages/gpui/cross-runtime-parity-report.json`
- `packages/gpui/native-accessibility-proof.json`
- `packages/svelte/preview/artifacts/parity-report.json`
- `packages/svelte/preview/artifacts/accessibility-report.json`
- `packages/ecosystem-acceptance.json`
- `docs/roadmaps/g04/011-cross-runtime-parity-report-intentional-delta-register-and-acceptance-harness-expansion.md`
