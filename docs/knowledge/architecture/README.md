# Architecture

Status: active

Architecture documents define Poodle's stable ownership and runtime boundaries.
They explain the current system; what comes next belongs in
the Nucleus plan.

## Start Here

1. [System shape](001-poodle-system-shape.md) — contracts, web and native
   renderer flows, parity, and application boundaries
2. [Token system and package layout](002-token-system-and-package-layout.md) —
   canonical token sources, generated artifacts, and consumer packages
3. [Component docs and implementation substrates](003-component-docs-ia-and-implementation-substrates.md)
   — documentation authority and implementation evidence
4. [Headless core and machine model](006-headless-core-and-machine-model.md)
   — the renderer-neutral behaviour layer
5. [Appearance recipe contract](007-appearance-recipe-contract.md)
   — appearance recipes for components
6. [Audio control family](008-audio-control-family.md) — audio-domain machines,
   renderer-neutral VisualState, value laws, formatting, and meter feeds

## Additional Decisions

The numbered documents after the active baseline record narrower architectural
decisions. Retired documents remain for traceability and state what superseded
them.

- [Underlay bridge and adapter ownership](004-underlay-bridge-and-adapter-ownership.md)
  — superseded; current consumer ownership lives in the system shape
- 005 (retired) — superseded by the appearance recipe contract (007)
- [Node backend conformance](009-cross-runtime-component-conformance.md)
  — typed fixtures, headless observations and versioned backend receipts
- [Native presentation construction context](010-native-presentation-construction-context.md)
  — accepted shared-Rust context, inheritance, composition, and explicit-
  override rules
- [Drag and drop substrate](011-drag-and-drop-substrate.md) — semantic session
  ownership, paired runtime machines, touch, cross-window host boundaries,
  inbound files, and native file drag-out
- [Semantic motion policy](012-semantic-motion-policy.md) — explicit
  full/reduced/frozen host policy, restriction-only inheritance, lifecycle,
  property budget, native approximation, and first-pilot evidence
- [Icon geometry substrate](013-icon-geometry-substrate.md) — internal
  normalized geometry, deterministic pair planning, and candidate/rejection
  lineage; no runtime-eligible pair
- [Compiled web package distribution](014-compiled-web-package-distribution.md)
  — source-free `dist` boundary, dual Svelte client/server lanes, deterministic
  receipts, and installed-tarball certification ownership
- [Theme resolution](015-theme-resolution.md) — settings, colour roles, fit
  targets, presets and cross-runtime conformance
- [Shared native specimens](016-shared-native-specimens.md) — shared page
  bodies, typed sessions and host effects
