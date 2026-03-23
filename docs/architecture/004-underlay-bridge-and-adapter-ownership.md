# 004 Underlay Bridge And Adapter Ownership

Status: active
Updated: 2026-03-11
Depends on: `001-poodle-system-shape.md`, `002-token-system-and-package-layout.md`, `003-component-docs-ia-and-implementation-substrates.md`

## Purpose

Freeze the ownership model for Underlay integration now that Poodle has explicit
token, primitive, product-composite, and workstation-shell surfaces.

## Core Rule

Underlay remains the public framework surface for Underlay apps.

Poodle may supply:

- canonical tokens
- shared component contracts
- internal implementation surfaces
- and bridge helpers

Poodle may not require Underlay apps to:

- import Poodle directly
- adopt Poodle prop names directly
- think in Poodle component-layer taxonomy
- or treat Poodle as the app-facing source of truth

## Bridge Package Rule

The Poodle-owned Underlay bridge lives at:

```text
packages/bridges/underlay/
```

That package exists to define:

- token-name mapping
- theme and mode mapping
- wrapper-preservation rules
- and bridge-local helpers that Underlay can consume internally

It does not become a second canonical token source or a shadow component
library.

## Token Ingestion Rule

The Underlay bridge may ingest only emitted token artifacts, not raw token
schema files, as its normal runtime dependency.

That means the bridge should read from:

- `packages/tokens/artifacts/css/`
- `packages/tokens/artifacts/ts/`

and map those artifacts into Underlay-owned names and application mechanisms.

The bridge must not:

- redefine canonical token meaning
- fork theme values
- or become the place where semantic roles are renamed canonically

## Wrapper Preservation Rule

Underlay wrappers may adapt Poodle implementations internally, but they must
preserve Underlay's public API surface.

That includes:

- Underlay-owned component names
- Underlay-owned prop names
- Underlay-owned composition patterns
- Underlay-owned migration cadence

Poodle components may sit underneath those wrappers where it helps.
They may not become the new required import path for Underlay apps.

## Ownership Split

### Poodle Owns

- canonical token schema
- emitted token artifacts
- canonical component contracts
- Poodle Svelte and GPUI implementations
- bridge package structure and baseline mapping rules

### Underlay Owns

- public app-facing APIs
- app-facing theme registration and runtime hooks
- rollout sequencing
- compatibility wrappers
- deprecation policy for existing Underlay surfaces

### Shared Boundary

- bridge-local token maps
- wrapper-preservation rules
- migration pressure-point documentation

## Migration Pressure Rule

The first bridge baseline must document where adoption pressure is likely to
appear before implementation begins.

Likely pressure points include:

- token naming mismatches between Poodle semantic roles and Underlay runtime names
- theme registration shape and CSS variable application order
- wrapper prop translation for existing Underlay components
- accessibility parity where Underlay wrappers compose Poodle primitives or
  composites
- shell versus product-surface layering in Underlay apps

These are bridge concerns, not reasons to weaken the canonical Poodle contract.

## Accessibility Rule

Underlay bridge code must preserve accessibility semantics while adapting names
or wrappers.

The bridge may translate:

- prop names
- CSS variable names
- wrapper composition

It may not silently drop:

- names/labels
- roles
- state exposure
- keyboard behavior
- focus restoration
- or announcement behavior

Any accessibility gap introduced by a wrapper is a bridge defect.

## Package Shape

The first bridge package shape is:

```text
packages/bridges/underlay/
  README.md
  package.json
  css/
    poodle-to-underlay.css
  ts/
    index.ts
    token-map.ts
    theme-map.ts
    component-wrappers.ts
```

This is intentionally small.
The baseline should prove ownership and mapping posture before real adoption.

## Next Task

Use this ownership model during later `g02` and `g03` adoption work, especially
where zero-leak wrapper preservation needs to stay intact.
