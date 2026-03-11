# g01.003 Token Artifact Emission, Themes, And Density Modes

Status: planned
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.002
Primary repos: `pug`

## Research Inputs

- [tm-token-system](../../research/translation-memos/tm-token-system.md)
- [hub-gpui](../../research/source-hubs/hub-gpui.md)
- [tk-design-token-systems](../../research/value-tracks/tk-design-token-systems.md)

## Context

The schema from `g01.002` is only useful if it can emit artifacts consumed by
Svelte, GPUI, Underlay bridges, and future downstream packages.

## Problem

Without explicit emitted shapes, tokens will remain conceptual and each
consumer will invent its own format and runtime semantics.

## Goals

- [ ] choose the first token-emission toolchain
- [ ] define emitted artifact shapes for CSS custom properties
- [ ] define emitted artifact shapes for TypeScript consumers
- [ ] define emitted artifact shapes for Rust/GPUI consumers
- [ ] define theme variants and density/control-size modes
- [ ] define how tokens are versioned and traced back to the source of truth
- [ ] define how one semantic theme emits to both Svelte and GPUI targets

## Non-Goals

- [ ] no full package implementation yet
- [ ] no downstream migration tooling yet

## Execution Checklist

- [ ] freeze Style Dictionary 4.x plus a Rust-facing format/transform as the
  first emission baseline
- [ ] define the CSS artifact surface and variable-namespace strategy
- [ ] define the TypeScript artifact surface and consumer access patterns
- [ ] define the Rust artifact surface and token-binding expectations
- [ ] define how emitted artifacts preserve traceability back to DTCG source
  paths and metadata
- [ ] define how theme variants map into each emitted format
- [ ] define how a single named theme is translated into CSS-facing and
  Rust-facing outputs without semantic divergence
- [ ] define how density and control-size modes are represented
- [ ] define how metadata such as deprecation, aliases, and semantic grouping
  travels with emitted artifacts
- [ ] define whether CSS themes land as separate files, layered selectors, or
  both
- [ ] define the first Rust theme-module shape that GPUI can consume without
  hand-maintained duplicate values
- [ ] name the first generated files under `packages/tokens/artifacts/css/`
- [ ] name the first generated files under `packages/tokens/artifacts/ts/`
- [ ] name the first generated files under `packages/tokens/artifacts/rust/`
- [ ] name the first Svelte consumer stubs under `packages/svelte/tokens/`
- [ ] name the first GPUI consumer stubs under `packages/gpui/tokens/`
- [ ] align artifact planning to
  `docs/architecture/002-token-system-and-package-layout.md`
- [ ] align artifact planning to
  `docs/specs/001-token-source-and-artifact-contract.md`

## Acceptance Criteria

- [ ] the emission-tooling baseline is explicit
- [ ] artifact shapes are documented for CSS, TypeScript, and Rust
- [ ] theme and density behavior are explicit
- [ ] the same semantic theme can be expressed across CSS and Rust targets from
  one source
- [ ] consumer expectations are clear enough to begin package implementation
  later

## Deliverables

- [ ] artifact-shape definitions
- [ ] theme and density-mode rules
- [ ] first generated-file inventory for CSS, TypeScript, and Rust
- [ ] first consumer-stub inventory for Svelte and GPUI

## Evidence Requirements

- [ ] one end-to-end example of a token family through all emitted forms
- [ ] one example of theme-mode selection and one of density variation
- [ ] one concrete mapping from schema files to generated artifact files
- [ ] one concrete mapping from a named semantic theme to both CSS and GPUI
  artifact outputs
- [ ] one documented example of emitted metadata or source-path traceability

## Next Task

Open `g01.004` and define the component contract template now that the token
source and artifact flow are concrete enough to reference directly.
