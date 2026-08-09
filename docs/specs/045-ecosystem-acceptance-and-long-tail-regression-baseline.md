# 045 Ecosystem Acceptance And Long-Tail Regression Baseline

Status: active
Updated: 2026-03-12
Depends on: `025-parity-automation-and-harness-boundary.md`, `040-underlay-bridge-zero-leak-adoption-proof-baseline.md`, `041-loophole-foundation-adoption-and-daw-extension-boundary.md`, `042-gpui-multi-app-validation-target-matrix.md`, `043-accessibility-audit-and-cross-runtime-delta-handling-baseline.md`, `044-deprecation-change-control-and-release-channel-operations.md`

## Purpose

Freeze what counts as honest ecosystem acceptance in `g03` and make long-tail
regression coverage explicit before Poodle starts talking more seriously about
reference apps or wider public-facing examples.

The current repo does not yet contain a fleet of runnable downstream apps or
full screenshot and native-runtime regression suites. This milestone exists to
make the acceptance surface explicit anyway, using proof-backed downstream
boundaries and named regression classes rather than wishful language.

## Core Rule

Ecosystem acceptance must be defined as a bounded matrix of representative
consumer suites plus named long-tail regression classes.

Poodle should not claim broad ecosystem readiness from preview build success,
token publication posture, or one downstream proof in isolation.

## Required Acceptance Suites

The current baseline must explicitly track at least:

- one browser-based review harness for direct Svelte package evaluation
- one bridge-mediated downstream adoption suite
- one direct downstream workstation or foundation adoption suite
- one GPUI-native validation target matrix

These suites may be proofs, baselines, or matrix-only targets depending on the
current runtime maturity. They must not all be treated as equally complete.

## Long-Tail Regression Rule

The current baseline must also make the main regression classes explicit.

At minimum, the long-tail matrix must cover:

- tokens and theme emission
- package surface and contract integrity
- preview routes and docs build posture
- accessibility and keyboard posture
- downstream boundaries and ownership
- release metadata and change control
- runtime-specific GPUI assumptions

The goal is to keep “ecosystem regression” from collapsing into a vague bucket
that nobody can reason about.

## Current Acceptance Artifact

The current machine-readable acceptance artifact is:

- `packages/ecosystem-acceptance.json`

It records:

- the representative acceptance suites
- suite status and blockers
- covered packages
- supporting evidence artifacts
- long-tail regression classes
- the current ecosystem-readiness gate

## Suite Status Rule

Acceptance suites must remain honest about their current strength:

- `baseline` for internal harnesses that are real but not downstream proof
- `proof-backed` for downstream adoption boundaries with explicit artifacts
- `matrix-only` for target pressure that is not yet runnable acceptance

If a suite is `matrix-only`, it must record blockers rather than implying
production readiness.

## Readiness Gate Rule

Poodle may only speak in stronger ecosystem-readiness terms when the readiness
gate remains explicit and aligned with the current proofs.

The current gate requires:

- machine-checked preview docs and report surfaces
- explicit Underlay and Loophole adoption proofs
- honest GPUI validation posture
- accessibility, release, and package-surface baselines that still match the
  acceptance suites they are supposed to support

## Honesty Rule

Poodle may say:

- the ecosystem acceptance matrix is explicit
- representative suites and regression classes are named
- some suites are proof-backed while others are still baseline or matrix-only

Poodle may not say:

- broad ecosystem readiness is proven by one downstream proof alone
- GPUI ecosystem acceptance is complete without runnable native component
  evidence
- long-tail regressions are covered if the repo cannot name the regression
  classes and supporting artifacts

## Current `g03.012` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/ecosystem-acceptance.json`
- lint validation that keeps the acceptance matrix present and internally
  coherent
- `docs/roadmaps/g03/012-ecosystem-acceptance-suites-and-long-tail-regression-coverage.md`
