# 049 GPUI Theme Runtime And Native Preview App Baseline

Status: active
Updated: 2026-03-13
Depends on: `048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`

## Purpose

Freeze the first real GPUI runtime and review-surface baseline for `g04`.

This milestone exists so GPUI work starts from a native review app with emitted
token application, explicit review controls, and side-by-side comparison
targets, instead of abstract language about a future harness.

## Core Rule

The GPUI preview app must be a native review surface that mirrors the most
important Svelte preview review jobs closely enough for side-by-side parity
inspection.

It should not be treated as a browser-route clone. It should be treated as the
native review surface that proves GPUI theme application and exposes the first
side-by-side comparison targets honestly.

## Required Preview Artifact

The current generation must maintain a machine-readable GPUI preview baseline
that records:

- the emitted token source used by the native runtime
- supported theme IDs
- supported density and control-size review dimensions
- the section set required for the first native preview wave
- required review controls and shell areas
- evidence-capture expectations
- explicit non-goals

The current artifact is:

- `packages/gpui/preview-app-baseline.json`

## Theme Runtime Rule

The GPUI review app must consume the same emitted theme vocabulary as the
Svelte surface wherever the contracts and tokens already own that meaning.

At minimum, the first native review wave must carry:

- `light`
- `dark`
- `loophole-studio`

It must also expose density and control-size review dimensions as first-class
state, because those are already part of the shared review model in Svelte.

## Wave-0 Section Rule

The first required GPUI preview sections are:

- `catalog-hub`
- `token-summary-section`
- `token-inspector`

These are the right first targets because they prove:

- the review shell itself exists
- emitted tokens apply natively
- parity debugging can start before larger GPUI component work lands

## Side-By-Side Review Rule

The native preview app should support side-by-side review against the Svelte
preview for the wave-0 sections.

This means:

- section ownership and metadata are inspectable in both runtimes
- token values can be compared under the same theme and density states
- searchable token inspection exists in both runtimes even if the presentation differs

It does not mean the shell chrome must be pixel-identical.

## Evidence Rule

The current preview baseline should name what evidence is worth capturing now.

At minimum, it should allow:

- stable screenshots for theme and section states
- recorded inspection notes for token application defects
- section plus theme snapshots that can be compared against the Svelte review surface

## Honesty Rule

Poodle may say:

- GPUI has a native preview-app baseline
- emitted tokens and themes are being applied in a native review surface
- side-by-side comparison begins with wave-0 sections

Poodle may not say:

- the GPUI preview app proves full GPUI component parity
- native preview shell existence implies all later component layers are solved
- browser preview routes are the actual native review surface

## Current `g04.002` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/gpui/preview-app-baseline.json`
- `packages/gpui/parity-priority-matrix.json`
- `docs/roadmaps/g04/002-gpui-theme-runtime-token-application-and-native-preview-app-baseline.md`

## Next Task

Carry this baseline into `g04.003` so GPUI structural primitives land inside a
real native review app and theme runtime rather than as isolated native widgets.
