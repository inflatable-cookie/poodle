# 048 GPUI Contract Audit, Priority, And Side-By-Side Review Baseline

Status: active
Updated: 2026-03-13
Depends on: `008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `042-gpui-multi-app-validation-target-matrix.md`, `047-generation-closeout-and-next-program-posture.md`

## Purpose

Freeze the implementation order and parity posture for `g04` before GPUI work
turns into a pile of ad hoc native widgets.

This baseline exists so Pug can move from token-only GPUI posture toward the
same contract-owned UI in Svelte and GPUI, without confusing “native
adaptation” with “anything goes” and without pretending that every browser
preview section deserves identical native treatment.

## Core Rule

GPUI should imitate the Svelte surface as closely as possible where the
contract owns the UI, and document native-runtime deltas explicitly where it
does not.

The goal for shared Loophole-facing surfaces is the same UI across runtimes,
not merely abstract conceptual equivalence. Native adaptation is allowed only
when it preserves the contract more honestly than blind Svelte imitation.

## Required Priority Artifact

The current generation must maintain a machine-readable GPUI priority matrix
that records:

- implementation waves
- section-level priority
- parity mode
- owning GPUI layer
- whether side-by-side review should exist
- explicit reasons for that classification

The current artifact is:

- `packages/gpui/parity-priority-matrix.json`

## Parity Mode Rule

The current baseline recognizes exactly three parity modes:

- `direct-parity`
- `native-adaptation`
- `deferred`

`direct-parity` means the GPUI surface should stay visually and structurally
close enough to the Svelte surface for side-by-side comparison to expose real
defects.

`native-adaptation` means the same contract still holds, but the runtime is
expected to differ in mechanics or presentation enough that the delta should be
made explicit.

`deferred` is allowed only when the repo is honest that a surface is not yet in
the active implementation wave.

## Side-By-Side Review Rule

The GPUI review surface should mirror the Svelte preview closely enough that
the most important shared sections can be inspected side by side.

This does not require pixel-identical review chrome. It does require explicit
coverage for the sections where side-by-side comparison materially improves
defect detection.

The current highest-value side-by-side surfaces are:

- tokens and live theme values
- forms and validation posture
- detail displays
- notifications and remediation
- command discovery
- workstation shell surfaces

## Initial Wave Rule

The implementation order should begin with:

1. theme runtime and native preview foundation
2. primitives that later composites depend on
3. product composites
4. workstation shell surfaces

This prevents GPUI workstation work from being built on one-off native controls
that never aligned with the shared foundation layer.

## Honesty Rule

Pug may say:

- GPUI implementation order is explicit
- some sections require direct parity while others require native adaptation
- side-by-side review is a first-class parity tool for the shared UI

Pug may not say:

- browser preview parity is enough to imply GPUI parity
- native adaptation means Loophole-facing UI can drift freely from Svelte
- every browser preview surface deserves a one-to-one native clone regardless
  of contract ownership

## Current `g04.001` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/gpui/parity-priority-matrix.json`
- `docs/roadmaps/g04/001-gpui-contract-audit-parity-priority-matrix-and-implementation-order.md`

## Next Task

Carry this baseline into `g04.002` so the GPUI preview app and theme runtime
start from explicit side-by-side review targets instead of abstract native
harness language.
