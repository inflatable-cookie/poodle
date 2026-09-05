# 042 GPUI Multi-App Validation Target Matrix

Status: active
Updated: 2026-03-12
Depends on: `../008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `021-public-package-api-stability-and-parity-debt-baseline.md`, `040-underlay-bridge-zero-leak-adoption-proof-baseline.md`, `041-loophole-foundation-adoption-and-daw-extension-boundary.md`

## Purpose

Freeze what counts as honest multi-app GPUI validation in `g03` when the repo
does not yet contain multiple runnable downstream GPUI apps.

The goal of this milestone is to make the validation targets and hidden
shared-layer assumptions explicit, not to overclaim rollout evidence the repo
cannot currently prove.

## Core Rule

When runnable downstream GPUI apps are absent, multi-app validation must exist
as an explicit target matrix plus an assumption inventory.

Poodle should not claim production-ready GPUI multi-app validation from token
artifacts or contract notes alone.

## Validation Target Rule

The current baseline must identify at least:

- one workstation-shaped GPUI app target
- one non-workstation productivity or review target
- one smaller utility or tool-window target

These targets exist to expose hidden assumptions in the shared layer, not to
imply all targets are already implemented in-repo.

## Current Validation Artifact

The current target matrix artifact is:

- `packages/gpui/tokens/multi-app-validation.json`

## Required Matrix Contents

The multi-app validation matrix must record:

- target IDs and labels
- target shapes and interaction pressure
- the contract families each target stresses
- the shared-layer assumptions that those targets are expected to expose
- the required follow-up work
- the current blockers that still limit stronger GPUI claims

## Assumption Inventory Rule

The matrix must explicitly challenge these assumption classes:

- workstation shell pressure becoming the default GPUI assumption
- token-level readiness being mistaken for wider component readiness
- app-specific product or DAW semantics leaking into canonical contracts
- host-owned architecture concerns becoming mandatory shared-layer patterns
- accessibility and keyboard obligations being inferred instead of validated

## Honesty Rule

Poodle may say:

- a GPUI multi-app validation target matrix exists
- hidden shared-layer assumptions are explicitly named
- blockers to stronger GPUI claims are recorded

Poodle may not say:

- multiple GPUI apps are already adopted in production
- GPUI component parity is proven by the current repo
- workstation-shaped needs have been proven to generalize automatically to all
  other GPUI app contexts

## Current `g03.009` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/gpui/tokens/multi-app-validation.json`
- `packages/gpui/tokens/README.md`
- `docs/roadmaps/g03/009-additional-gpui-app-adoption-and-multi-app-validation.md`
