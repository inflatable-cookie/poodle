# 025 Parity Automation And Harness Boundary

Status: active
Updated: 2026-03-12
Depends on: `002-component-contract-template-and-parity-rules.md`, `008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `024-token-evolution-migration-and-compatibility-policy.md`

## Purpose

Freeze what parity evidence Pug should automate now, which harness surfaces are
allowed to stand in for repeat review, and which parity judgments must remain
manual until later generations add deeper runtime coverage.

## Core Rule

Automation should reduce ambiguity and review drift. It should not pretend to
prove runtime parity that the repo cannot yet execute honestly.

`g03.002` therefore freezes a mixed model:

- machine-readable parity registration
- stable preview routes for repeat visual and interaction review
- generated parity reports describing what was covered
- explicit manual-review boundaries where automation is still insufficient

## Required Automated Baseline

The repo must automate these checks now:

- every docs/catalog section participating in parity review is registered in a
  machine-readable parity target list
- every parity target declares contract, visual, and interaction coverage
  posture
- every parity target exposes at least one stable preview route for repeat
  review
- family-to-section coverage remains resolvable from the catalog baseline
- public exports from the Svelte component packages are classified as either
  previewed or contract-only so preview coverage debt stays explicit
- the generated parity report remains reproducible from repo state

The `g03.002` seed implementation for this baseline is:

- `packages/svelte/preview/src/parity.ts`
- `packages/svelte/preview/scripts/build-parity-report.ts`
- `packages/svelte/preview/artifacts/parity-report.json`

## Harness Classes

Three harness classes are recognized in the current baseline:

- `contract`: machine-readable registration, provenance, and expected coverage
- `visual`: stable preview routes used for repeated appearance review
- `interaction`: stable preview routes used for repeated keyboard/focus/state
  review where the Svelte surface can demonstrate them honestly

Every parity target must declare coverage for all three as one of:

- `automated`
- `hybrid`
- `manual`
- `not-applicable`

## Contract Harness Rule

Contract harnesses should be automated first because they are the least
ambiguous.

In the current baseline, contract harness automation is expected to prove:

- section registration exists
- package ownership is explicit
- contract-root provenance is explicit
- review routes exist
- automation and manual-review expectations are declared

Contract harness automation does not prove visual or runtime interaction
quality by itself.

## Visual Harness Rule

Visual harnesses in `g03.002` are route-based and review-oriented, not yet
screenshot-regression gates.

They should provide:

- stable URLs for each review target
- explicit theme, density, and control-size state
- stable section selection
- repeatable surfaces for light, dark, and workstation-oriented review where
  relevant

Visual judgment itself remains hybrid for now because hierarchy, contrast,
spacing, and visual fit still require human review even when the review surface
is stabilized.

## Interaction Harness Rule

Interaction harnesses in `g03.002` are also route-based and review-oriented.

They may support repeat review of:

- keyboard traversal
- focus entry and restoration
- state toggles
- loading, empty, invalid, and remediation posture

They should not yet claim to prove:

- GPUI interaction parity
- assistive technology narration quality in native stacks
- drag or resize nuance across runtimes
- host integration behavior that the Svelte preview does not own

## Manual Boundary Rule

These areas remain manual until later generations add deeper automation:

- final visual acceptance
- GPUI runtime parity review
- assistive-technology narration quality
- drag, resize, and motion nuance
- host-owned behaviors outside the preview surface

Manual boundaries must be named explicitly in the generated parity report. They
must not remain implicit “someone should probably check this” gaps.

## Evidence Artifact Rule

The generated parity report is now a first-class evidence artifact.

It must summarize:

- parity targets
- coverage posture by harness type
- public Svelte package export coverage posture
- review routes
- automated boundaries
- manual boundaries

The current baseline artifact is:

- `packages/svelte/preview/artifacts/parity-report.json`

## Preview Route Rule

The Svelte preview must support stable, URL-addressable review state so parity
evidence can point at concrete surfaces instead of generic instructions.

At minimum the route state must encode:

- section
- theme
- density
- control size

## Honesty Rule

Automation must not overclaim.

Pug may say:

- a parity target is registered
- a review route exists
- a report was generated
- a surface is suitable for repeat visual or interaction review

Pug may not say:

- GPUI parity is proven without GPUI evidence
- accessibility is fully validated from route generation alone
- visual parity is complete because a screenshotable route exists

## Current `g03.002` Baseline

The current baseline now includes:

- preview-state URLs synchronized to section, theme, density, and control size
- a parity target registry derived from the live docs/catalog surface
- a package-export coverage manifest that records which public Svelte exports
  are directly previewed versus only contract-backed today
- a generated parity report artifact
- root-level commands to regenerate the report and run the docs build against it

## Next Task

Use this baseline while executing `g03.003`, so contract linting and docs
completeness checks can build on explicit parity registration instead of
inferred review coverage.
