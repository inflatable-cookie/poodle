# 022 Packaging Versioning And Release Channel Rules

Status: active
Updated: 2026-03-12
Depends on: `021-public-package-api-stability-and-parity-debt-baseline.md`

## Purpose

Freeze the packaging and release posture for the current Poodle package set so the
repo can distinguish public-intent packages from internal-only packages before
downstream adoption starts.

## Package Classification Rule

Every package must fall into one of these release classes:

- `source-of-truth`: internal packages that define or generate canonical data
- `runtime-package`: packages intended to be consumed by downstream runtimes
- `bridge`: internal adaptation layers for downstream-owned systems
- `tooling`: internal docs, preview, or validation tooling

The canonical classification record is `packages/release-manifest.json`.
Operational change-control rules now live in `packages/release-operations.json`.

## Channel Rule

Only two release channels exist in the current baseline:

- `preview`: public-intent packages that may eventually be published but are
  still explicitly pre-release
- `internal`: packages that must not be treated as downstream dependencies

No package may imply a stable channel until a later generation explicitly adds
one.

## Versioning Rule

The current baseline remains pre-1.0:

- all packages remain on `0.x`
- breaking changes may happen in minor releases while the system is still
  pre-1.0
- breaking changes must still be called out in release notes, roadmap/log
  summaries, and any downstream handoff material

## Release Metadata Rule

Each release-bearing package manifest should declare:

- package name
- version
- description
- explicit release intent metadata
- explicit package exports or crate boundaries

This metadata may live in:

- `package.json` for JS/TS packages
- `Cargo.toml` metadata for Rust crates
- `packages/release-manifest.json` for repo-wide classification

## Current Package Baseline

The current release posture is:

### Preview Channel Public-Intent Packages

- `@inflatable-cookie/poodle-core/tokens`
- `@inflatable-cookie/poodle-svelte`
- `@inflatable-cookie/poodle-svelte-workstation`
- `poodle-gpui-tokens`
- `poodle-gpui-primitives`
- `poodle-gpui-composites`
- `poodle-gpui-workstation`

### Internal Packages

- `@inflatable-cookie/poodle-tokens`
- `@inflatable-cookie/poodle-bridge-underlay`
- `@inflatable-cookie/poodle-svelte-preview`

## Consumption Rule

Downstream repos should only plan around preview-channel packages.

They should not depend on:

- internal source-of-truth token build packages
- bridge packages
- preview/docs tooling packages

## Release Note Rule

Each release-capable tranche should document:

- which packages changed
- whether the change affects public-intent entry points
- whether the change is additive, behavioral, or breaking
- what downstream evaluators should re-check

## Seed Evidence

- `packages/release-manifest.json`
- `packages/release-operations.json`
- `packages/svelte/tokens/package.json`
- `packages/svelte/components/package.json`
- `packages/svelte/components/package.json`
- `packages/svelte/workstation/package.json`
- `packages/svelte/preview/package.json`
- `packages/bridges/underlay/package.json`
- `packages/gpui/tokens/Cargo.toml`
- `packages/gpui/primitives/Cargo.toml`
- `packages/gpui/composites/Cargo.toml`
- `packages/gpui/workstation/Cargo.toml`

## Next Task

Carry this baseline into `g03.011` and later operations work so preview,
internal, and future stable channel language remain explicit instead of
drifting into implied release promises.
