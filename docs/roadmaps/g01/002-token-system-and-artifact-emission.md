# g01.002 Canonical Token Schema And Naming System

Status: completed
Owner: Flint Core
Updated: 2026-03-11
Depends on: g01.001
Primary repos: `flint`

## Research Inputs

- [tm-token-system](../../research/translation-memos/tm-token-system.md)
- [hub-gpui](../../research/source-hubs/hub-gpui.md)
- [tk-design-token-systems](../../research/value-tracks/tk-design-token-systems.md)

## Context

Flint cannot promise parity across Svelte and GPUI unless both implementations
consume one semantic token model.

## Problem

If tokens are defined ad hoc per framework, Flint will immediately accumulate
drift in naming, theme logic, component states, density modes, and downstream
integration posture.

## Goals

- [ ] adopt one canonical DTCG token source format for Flint
- [ ] define the canonical token taxonomy
- [ ] separate raw scales, semantic aliases, and mode/theme overlays
- [ ] define naming rules that work across CSS, TypeScript, and Rust consumers
- [ ] define stateful token roles and density/control-size variants
- [ ] define which token categories are core versus optional
- [ ] define the semantic theme model so one named theme can translate across
  browser/Svelte and GPUI consumers

## Non-Goals

- [ ] no artifact emission implementation yet
- [ ] no component-specific token mapping beyond illustrative examples

## Execution Checklist

- [ ] freeze W3C DTCG as the canonical schema format for token source files
- [ ] define token families for color, typography, spacing, sizing, radius,
  border, elevation, motion, density, icon, overlay, and state
- [ ] document the split between `primitives/`, `semantic/`, and `modes/`
  schema layers
- [ ] define token naming rules that survive CSS, TypeScript, and Rust emission
- [ ] define mode dimensions such as theme, density, and control size
- [ ] define the metadata layer for aliases, deprecations, and manifest data
- [ ] define how primitives and composites consume semantic roles instead of raw
  scales directly
- [ ] freeze the first package layout for `packages/tokens/schema/`,
  `packages/tokens/scripts/`, and `packages/tokens/artifacts/`
- [ ] name the first required schema files under `primitives/`, `semantic/`,
  `modes/`, `metadata/`, and `manifest.json`
- [ ] define how named themes, light/dark modes, and density/control-size
  overlays coexist without duplicating semantic meaning
- [ ] define the canonical theme-schema fields needed so one theme can emit to
  both CSS and Rust targets
- [ ] define at least one example named theme, such as a Loophole-oriented
  theme, as a semantic theme rather than a framework-local implementation
- [ ] define the semantic naming transforms expected for CSS custom properties,
  TypeScript exports, and Rust constants/modules
- [ ] align this milestone to
  `docs/architecture/002-token-system-and-package-layout.md`
- [ ] align this milestone to
  `docs/specs/001-token-source-and-artifact-contract.md`

## Acceptance Criteria

- [ ] token taxonomy is documented
- [ ] the DTCG source-format decision is explicit
- [ ] naming rules are documented
- [ ] theme and mode dimensions are documented
- [ ] theme translation requirements across Svelte and GPUI are documented
- [ ] downstream bridge constraints are represented in the schema design

## Deliverables

- [ ] canonical token schema note
- [ ] naming and state-role rules
- [ ] first package and file layout for token source-of-truth data
- [ ] first normative token-source contract

## Evidence Requirements

- [ ] one documented mapping from raw scale to semantic role
- [ ] one documented example showing how the same token category reaches CSS and
  Rust consumers conceptually
- [ ] one concrete package/file map for the first token tranche
- [ ] one documented example showing how the same named theme will emit to both
  browser and GPUI consumers
- [ ] one documented example of a DTCG token path surviving emission-name
  changes without semantic drift

## Next Task

Open `g01.003` and convert the schema plan into concrete CSS, TypeScript, and
Rust artifact outputs plus consumer stubs.
