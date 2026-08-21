# 007 Underlay Bridge And Wrapper Preservation Rules

Status: superseded
Updated: 2026-08-21
Superseded by: `../roadmaps/g12/022-underlay-bridge-extraction.md`

> **Superseded.** `packages/bridges/underlay` has been removed from Poodle. A
> design system must not carry a package named after one of its consumers, and
> the bridge's only real caller was a single Nightfire import in Underlay.
> Underlay now consumes Poodle's published packages directly and owns any
> translation in its own source. Retained for the reasoning; not current
> guidance.
Depends on: `001-token-source-and-artifact-contract.md`, `002-component-contract-template-and-parity-rules.md`, `003-accessibility-and-assistive-technology-baseline.md`, `005-product-composite-composition-and-information-architecture-rules.md`, `006-workstation-shell-and-panel-system-rules.md`

## Purpose

Freeze the baseline rules for Underlay adoption so Poodle can be consumed
internally by Underlay without turning Poodle into the public API for Underlay
applications.

## Core Rule

Underlay apps should remain Underlay-facing.

The bridge exists to allow:

- token ingestion
- theme translation
- internal component reuse
- and wrapper-backed migration

without requiring app code to import Poodle directly.

## Token Ingestion Rule

The Underlay bridge may ingest:

- emitted CSS token artifacts
- emitted TypeScript token metadata
- emitted theme and mode metadata

The Underlay bridge may not ingest raw schema files as its primary runtime
source.

This keeps:

- token meaning canonical in Poodle
- bridge mapping secondary
- and runtime consumption aligned to emitted artifacts rather than internal
  build-only source files

## Token Mapping Rule

Bridge token maps may rename Poodle-emitted variables into Underlay-owned naming
where needed.

They must:

- point back to canonical Poodle semantic paths
- remain traceable
- and avoid becoming a second semantic taxonomy

Allowed:

- `--underlay-color-surface -> semantic.color.background.surface`
- `--underlay-space-panel-x -> semantic.space.panel.x`

Forbidden:

- inventing new canonical token meaning in the bridge
- forking theme values per bridge consumer
- treating bridge aliases as the canonical semantic source

## Theme Translation Rule

The bridge may translate Poodle theme identities into Underlay-owned runtime theme
registration shapes.

It must preserve:

- semantic theme identity
- mode identity
- and override meaning

Theme translation may adapt:

- selector strategy
- registration API shape
- CSS file loading order
- and runtime naming conventions

It may not create hand-authored duplicate canonical themes.

## Wrapper Preservation Rule

When Underlay wraps Poodle components internally:

- the public Underlay component name remains Underlay-owned
- the public Underlay prop contract remains Underlay-owned
- migration from old Underlay implementation to Poodle-backed internals must be
  bridge-owned, not app-owned

Wrappers may adapt:

- prop names
- slot names
- event names
- class hooks
- and theme application mechanics

Wrappers may not expose raw Poodle component contracts as the new default API for
Underlay apps.

## Zero-Leak Rule

The bridge baseline assumes a zero-leak goal for app consumers.

That means application code should not need:

- Poodle package imports
- Poodle token variable names
- Poodle component names
- Poodle-specific prop names
- or Poodle-specific layout-layer terminology

If a real adoption tranche requires a temporary leak, it must be documented as
an approved delta and treated as migration debt.

## Ownership Rule

### Poodle Owns

- canonical tokens
- canonical component contracts
- Poodle Svelte implementations
- bridge baseline structure

### Underlay Owns

- public APIs
- final app-facing wrappers
- rollout and compatibility policy
- app-facing theme registration experience

### Bridge Owns

- token alias maps
- theme maps
- wrapper-preservation guidance
- migration pressure-point documentation

## Accessibility Rule

Bridge wrappers must preserve Poodle accessibility semantics when adapting APIs.

That includes:

- accessible names
- roles
- state exposure
- keyboard behavior
- focus order and restoration
- announcement behavior where relevant

Accessibility cannot be treated as a web implementation detail that wrappers
may simplify away.

## Pressure-Point Rule

The bridge baseline must explicitly acknowledge likely migration pressure:

- token-name translation and CSS variable ordering
- event and prop translation across wrapper boundaries
- state ownership differences between Underlay components and Poodle contracts
- theme registration shape
- accessibility behavior hidden in old Underlay components

These pressure points should be documented before adoption begins so the later
Underlay adoption tranche does not start from a blank slate.

## Package Baseline Rule

The initial bridge package should contain:

- a CSS variable bridge file
- a TypeScript token map
- a TypeScript theme map
- a TypeScript wrapper-preservation manifest or helper layer

This proves the ownership model without prematurely implementing Underlay
itself inside Poodle.

## Seed Evidence

The first artifacts that explicitly exercise this baseline are:

- `packages/bridges/underlay/css/poodle-to-underlay.css`
- `packages/bridges/underlay/ts/token-map.ts`
- `packages/bridges/underlay/ts/theme-map.ts`
- `packages/bridges/underlay/ts/component-wrappers.ts`
