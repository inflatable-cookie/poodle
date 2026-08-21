# 004 Underlay Bridge and Adapter Ownership

Status: superseded
Updated: 2026-08-21
Superseded by: `../roadmaps/g12/022-underlay-bridge-extraction.md`

> **Superseded.** `packages/bridges/underlay` has been removed from Poodle. A
> design system must not carry a package named after one of its consumers, and
> the bridge's only real caller was a single Nightfire import in Underlay.
> Underlay now consumes Poodle's published packages directly and owns any
> translation in its own source. Retained for the reasoning; not current
> guidance.
Depends on: [001 Poodle System Shape](001-poodle-system-shape.md),
[002 Token System and Package Layout](002-token-system-and-package-layout.md)

## Core Rule

Underlay remains the public framework surface for Underlay applications.
Poodle may implement Underlay internals, but an application must not need to
import Poodle, adopt Poodle prop names, or understand Poodle's component layers.

## Bridge Ownership

The Poodle-owned bridge lives at `packages/bridges/underlay/`. It contains:

- CSS variable and semantic token maps
- theme and mode translation
- compatibility wrapper helpers
- zero-leak proof utilities
- the Nightfire block-editor adapter

The bridge consumes emitted token and package surfaces. It does not redefine
canonical token meaning or become a second component library.

Underlay owns:

- application-facing component and token APIs
- theme registration and runtime hooks
- compatibility guarantees and deprecation policy
- adoption and rollout sequencing

Poodle owns:

- canonical token schema and generated artifacts
- component contracts
- Svelte, React, and native implementations
- bridge-local mappings needed to preserve Underlay APIs

## Wrapper Preservation

An adapter may translate names, prop shapes, CSS variables, and composition
syntax. It must preserve observable behavior, including:

- labels, roles, and state exposure
- keyboard operation and focus restoration
- dismissal and event timing
- validation and announcement behavior

An accessibility or behavioral gap introduced by a wrapper is a bridge defect,
not an acceptable downstream difference.

## Token Ingestion

The bridge consumes generated artifacts or public core exports, never raw token
schema as a runtime dependency. Poodle remains the token authority; Underlay
maps those values into Underlay-owned names and application mechanisms.

Application overrides use Underlay's supported surface. They must not fork
Poodle theme files inside individual applications.

## Migration Pressure

Bridge work should make translation pressure explicit before replacing an
existing wrapper:

- token-name and theme-registration differences
- prop and composition translation
- accessibility preservation
- ownership of shell versus product behavior
- release and compatibility expectations

If a mismatch is generic across consumers, improve Poodle or the bridge. If it
is application vocabulary or workflow policy, keep it in the application.

## Package Shape

```text
packages/bridges/underlay/
  css/poodle-to-underlay.css
  ts/index.ts
  ts/token-map.ts
  ts/theme-map.ts
  ts/component-wrappers.ts
  ts/zero-leak-proof.ts
  ts/nightfire-block-editor.ts
```

The package is internal infrastructure. It is not part of Poodle's public
operator installation path.
