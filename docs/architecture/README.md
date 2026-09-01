# Architecture

Status: active
Updated: 2026-09-01

Architecture documents define Poodle's stable ownership and runtime boundaries.
They explain the current system; milestone sequencing belongs in roadmaps.

## Start Here

1. [System shape](001-poodle-system-shape.md) — contracts, web and native
   renderer flows, parity, and application boundaries
2. [Token system and package layout](002-token-system-and-package-layout.md) —
   canonical token sources, generated artifacts, and consumer packages
3. [Component docs and implementation substrates](003-component-docs-ia-and-implementation-substrates.md)
   — documentation authority and implementation evidence
4. [Audio control family](008-audio-control-family.md) — audio-domain machines,
   renderer-neutral VisualState, value laws, formatting, and meter feeds

## Additional Decisions

The numbered documents after the active baseline record narrower architectural
decisions. Retired documents remain for traceability and state what superseded
them. Consult the [roadmap index](../roadmaps/README.md) only when you need
current delivery status.

- [Underlay bridge and adapter ownership](004-underlay-bridge-and-adapter-ownership.md)
  — superseded; current consumer ownership lives in the system shape
- [Cross-runtime component conformance](009-cross-runtime-component-conformance.md)
  — rejected g14 pilot record; not standing architecture
- [Native presentation construction context](010-native-presentation-construction-context.md)
  — accepted shared-Rust context, inheritance, composition, and explicit-
  override rules
- [Drag and drop substrate](011-drag-and-drop-substrate.md) — semantic session
  ownership, paired runtime machines, touch, cross-window host boundaries,
  inbound files, and native file drag-out
- [Semantic motion policy](012-semantic-motion-policy.md) — explicit
  full/reduced/frozen host policy, restriction-only inheritance, lifecycle,
  property budget, native approximation, and first-pilot evidence
