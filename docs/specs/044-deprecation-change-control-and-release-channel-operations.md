# 044 Deprecation, Change Control, And Release-Channel Operations

Status: active
Updated: 2026-03-12
Depends on: `021-public-package-api-stability-and-parity-debt-baseline.md`, `022-packaging-versioning-and-release-channel-rules.md`, `024-token-evolution-migration-and-compatibility-policy.md`, `039-extension-sdk-composition-guidance-and-starter-package-baseline.md`, `043-accessibility-audit-and-cross-runtime-delta-handling-baseline.md`

## Purpose

Freeze the minimum operational posture Flint should enforce for proposing,
classifying, deprecating, and removing changes while all public-intent packages
remain on the preview channel.

The goal of this milestone is not to introduce a stable channel early. It is
to stop package and contract changes from looking ad hoc now that the docs,
parity, performance, accessibility, and adoption-boundary baselines are all
explicit.

## Core Rule

Every material repo change must be classifiable, documentable, and traceable to
an explicit release-channel posture.

Public-intent packages may still evolve quickly during `g03`, but that speed
must be bounded by named change classes, named deprecation workflow, and an
explicit statement that no stable channel exists yet.

## Change-Class Rule

The current baseline recognizes exactly four release-facing change classes:

- `docs-only`
- `additive`
- `behavioral`
- `breaking`

These classes are recorded in `packages/release-operations.json`.

At minimum:

- additive, behavioral, and breaking changes to public-intent surfaces must
  point back to contract, artifact, or proof-backed evidence
- behavioral and breaking changes must be called out in roadmap, log, or
  release-note style summaries
- breaking changes must name the downstream reevaluation surface explicitly

## Channel Rule

The current generation still has only two active channels:

- `preview`
- `internal`

`stable` exists only as a reserved future channel and is explicitly disabled in
the current operations baseline. Flint may not imply stable release operations
until a later milestone promotes that channel deliberately.

## Deprecation Rule

Public-intent surfaces must follow a documented deprecation sequence before
removal:

1. proposed
2. documented
3. deprecated
4. removal-ready
5. removed

During `g03`, public-intent removals must not happen in the same documented
generation tranche that first announces deprecation unless the surface never
had public intent.

Deprecations must also name one of:

- a successor
- a migration path
- an explicit non-replacement decision

## Package-Operations Rule

Each package in the repo must remain classified across both:

- `packages/release-manifest.json`
- `packages/release-operations.json`

These records must agree on:

- package name
- release channel
- public-intent posture

They should also make clear whether a package is artifact-backed,
contract-backed, proof-backed, or tooling-owned for change-control purposes.

## Current Operations Artifacts

The current operations baseline lives in:

- `packages/release-manifest.json`
- `packages/release-operations.json`
- package-level release metadata in `package.json` or `Cargo.toml`
- `docs/roadmaps/g03/011-deprecation-change-control-and-release-channel-operations.md`

## Honesty Rule

Flint may say:

- preview packages are release-facing but still pre-release
- breaking changes may still occur in `0.x`
- internal packages do not carry downstream compatibility guarantees
- a stable channel is intentionally unavailable

Flint may not say:

- preview implies stable compatibility guarantees
- public-intent packages may remove surfaces without deprecation records
- package release metadata is optional once a package becomes widely used

## Current `g03.011` Seed Baseline

The current seed baseline is:

- this normative spec
- `packages/release-manifest.json`
- `packages/release-operations.json`
- release metadata in package manifests and `Cargo.toml`
- lint validation that keeps those records aligned

## Next Task

Carry this baseline into `g03.012` so ecosystem acceptance and long-tail
regression coverage can classify failures by change class and channel posture
instead of treating all churn as equal.
