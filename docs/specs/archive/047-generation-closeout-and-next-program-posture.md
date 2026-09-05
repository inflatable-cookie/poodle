# 047 Generation Closeout And Next-Program Posture

Status: active
Updated: 2026-03-12
Depends on: `039-extension-sdk-composition-guidance-and-starter-package-baseline.md`, `040-underlay-bridge-zero-leak-adoption-proof-baseline.md`, `041-loophole-foundation-adoption-and-daw-extension-boundary.md`, `042-gpui-multi-app-validation-target-matrix.md`, `043-accessibility-audit-and-cross-runtime-delta-handling-baseline.md`, `../044-deprecation-change-control-and-release-channel-operations.md`, `045-ecosystem-acceptance-and-long-tail-regression-baseline.md`, `046-reference-apps-onboarding-and-public-example-baseline.md`

## Purpose

Freeze what `g03` actually accomplished, what remains explicitly outside its
scope, and how the next program should be opened without treating unresolved
work as implied completion.

This baseline exists so generation closeout is a real artifact-backed decision
instead of a vague statement that the repo "feels mature enough" to move on.

## Core Rule

Generation closeout must name both the stable surfaces earned during the
generation and the carry-forward gaps that remain outside that generation's
claims.

Poodle should not close a generation by collapsing all remaining work into a
generic "next time" bucket.

## Required Closeout Artifact

The current generation closeout must exist as a machine-readable artifact that
records:

- the completed milestone set
- the stable surfaces now frozen as explicit outcomes
- the carry-forward gaps that remain for a later program
- the next-program posture

The current artifact is:

- `packages/g03-closeout.json`

## Stable Surface Rule

At minimum, `g03` closeout must make explicit that the generation now owns:

- canonical tokens and generated artifacts
- contract-backed Svelte package surfaces
- docs, preview, parity, accessibility, and publish evidence baselines
- Underlay and Loophole adoption-boundary proofs
- release operations, ecosystem acceptance, and onboarding guidance

These are the real `g03` outcomes and should not be written back down as vague
future aspirations.

## Carry-Forward Rule

Closeout must also record the gaps that remain outside `g03` claims.

At minimum, the current generation should still carry forward:

- GPUI component and runtime parity beyond token bindings and matrix-only evidence
- a published public docs platform rather than an internal preview harness
- richer runnable reference apps beyond reference shapes and onboarding lanes
- deeper automation such as screenshot regression or native-runtime acceptance proof

These gaps are not failures if they are explicit. They are failures only when
the repo implies they are already solved.

## Next-Program Rule

The next program must remain unopened until its goals are narrower than “more
hardening” or “more adoption” in the abstract.

The next program should start from the explicit `g03` artifacts rather than
reopening:

- release posture
- accessibility posture
- ecosystem acceptance taxonomy
- adoption boundary proofs
- reference-app and onboarding guidance

unless a new generation has concrete evidence for changing them.

## Honesty Rule

Poodle may say:

- `g03` is complete
- `g03` produced explicit hardening and adoption baselines
- some important runtime and publishing gaps still remain for a later program

Poodle may not say:

- GPUI parity is complete
- public docs publishing is complete
- reference-app depth is complete
- all future work is just generic maintenance

## Current `g03.014` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/g03-closeout.json`
- `docs/roadmaps/g03/014-generation-closeout-and-next-program-cutover.md`
- updated roadmap and generation-index surfaces marking `g03` complete
