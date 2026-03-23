# 046 Reference Apps, Onboarding, And Public Example Baseline

Status: active
Updated: 2026-03-12
Depends on: `020-docs-site-example-and-component-discoverability-rules.md`, `039-extension-sdk-composition-guidance-and-starter-package-baseline.md`, `040-underlay-bridge-zero-leak-adoption-proof-baseline.md`, `041-loophole-foundation-adoption-and-daw-extension-boundary.md`, `045-ecosystem-acceptance-and-long-tail-regression-baseline.md`

## Purpose

Freeze what Flint means by reference apps, adopter onboarding depth, and
public-facing examples during `g03`, without pretending the repo already ships
full runnable starter repos or polished marketing documentation.

This baseline exists so the adoption-facing surface can grow honestly from the
current proofs, packages, and preview examples rather than drifting into vague
"starter" language or template-like example code with no explicit ownership.

## Core Rule

Reference apps in `g03` are reference shapes and onboarding lanes first, not a
fleet of generated app templates.

Flint should define how direct consumers, bridge-mediated consumers,
workstation-oriented consumers, and evaluators should approach the system. It
should not imply that every adoption lane already has a polished runnable
example repository.

## Required Reference Shapes

The current baseline must make at least these adoption shapes explicit:

- a direct Svelte consumer reference shape
- a bridge-mediated reference shape
- a workstation-oriented reference shape
- a public example surface for evaluation and discoverability

These shapes may be `docs-backed`, `proof-backed`, or `preview-backed`
depending on the maturity of the evidence. They should not all be described as
complete starter apps.

## Onboarding Rule

Onboarding depth must identify:

- where an evaluator starts
- where a direct adopter starts
- where a bridge-mediated adopter starts
- where a workstation-oriented adopter starts

Each lane must point at real repo paths and define at least one non-goal so the
adopter can tell what not to infer from the current examples.

## Public Example Rule

Public-facing examples must remain discoverability and inspection aids.

They must:

- point back to the owning package and contract layer
- show meaningful state or shell posture rather than only default specimens
- remain explicit about preview-only glue versus public package API

They must not:

- silently act as hidden product templates
- imply GPUI parity from browser examples alone
- erase the wrapper, bridge, or domain-ownership rules already frozen

## Current Reference Artifact

The current machine-readable reference and onboarding artifact is:

- `packages/reference-apps.json`

It records:

- reference shapes
- onboarding lanes
- starting paths
- supporting evidence paths
- example section coverage
- current blockers and non-goals

## Honesty Rule

Flint may say:

- the repo has explicit reference shapes for major adoption lanes
- onboarding paths are documented
- the preview acts as a public example surface for evaluation

Flint may not say:

- every adoption lane has a finished runnable starter repo
- preview examples are a substitute for contracts and package READMEs
- public examples prove native-runtime parity or downstream production rollout

## Current `g03.013` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/reference-apps.json`
- package and preview README guidance aligned to the reference shapes
- lint validation that keeps the reference matrix present and internally
  coherent
- `docs/roadmaps/g03/013-reference-apps-onboarding-depth-and-public-examples.md`

## Next Task

Carry this baseline into `g03.014` so generation closeout can summarize real
adoption-facing assets and explicit remaining gaps instead of gesturing at
unfinished starter or example work.
