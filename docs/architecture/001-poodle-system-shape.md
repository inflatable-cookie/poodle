# 001 Poodle System Shape

Status: active
Updated: 2026-08-14

## Purpose

Poodle provides one design-system contract across web and native renderers.
Applications receive runtime-native components; component meaning, state,
behavior, accessibility, and token usage remain aligned.

This document defines the current ownership model. Component details live in
[component contracts](../contracts/components/README.md), while delivery status
lives in [roadmaps](../roadmaps/README.md).

## System at a Glance

```text
docs/contracts/components/        packages/tokens/schema/
component semantics               W3C DTCG token source
            \                         /
             shared contracts and artifacts
                    /          \
       web packages              Rust contracts
   core + Svelte + React     specs + headless + node
                                      |
                              poodle-render
                           Spec + Theme -> Node
                              /             \
                    GPUI node backend   Jetstream backend
```

There are two implementation paths:

- Web components use shared framework-free behavior and CSS from
  `@inflatable-cookie/poodle-core`, with thin Svelte or React shells.
- Native components use `poodle-render` as the single Rust component
  implementation. It produces renderer-neutral `poodle-node` trees, which a
  backend interprets for GPUI or Jetstream.

No application is expected to depend on every layer.

## Contract Authority

Each component contract defines its:

- purpose and anatomy
- public inputs and defaults
- states and state transitions
- events and event timing
- keyboard and accessibility behavior
- layout and token use
- allowed runtime differences

Contracts describe observable behavior rather than framework APIs. A Svelte
event handler, React callback, and Rust handler may have idiomatic signatures
while preserving the same semantic event.

Parity is evaluated in this order:

1. semantic inputs, states, and behavior
2. keyboard and accessibility behavior where the runtime supports it
3. token roles, layout intent, and visual treatment
4. runtime-specific rendering details

An implementation difference is acceptable only when the contract permits it
or the runtime cannot express the same mechanism. Known differences must be
documented, not silently embedded in an adapter.

## Implementation Pairs And Conformance

The durable implementation shape is pair-wise:

- **Web pair** — `poodle-core` is the single behavior source for Svelte and
  React; the shells stay idiomatic and thin.
- **Native pair** — `poodle-render` is the single component implementation
  for GPUI and Jetstream; backends interpret, they do not reimplement.

The pairs do not maintain four independent component implementations. They
also do not share executable behaviour across the TypeScript/Rust boundary.

A separate conformance plane binds both pairs to the component contracts. It
owns portable interface declarations, shared cases and specimen structure,
and normalized observations. It may generate types and serialized fixtures;
it must not generate component behaviour or become a renderer. Runtime-owned
mechanisms may differ, but their observable result must satisfy the same case.
See [009 Cross-Runtime Component Conformance](009-cross-runtime-component-conformance.md).

## Web Architecture

### `@inflatable-cookie/poodle-core`

The framework-free web package owns:

- component state machines and prop getters
- generated token data and CSS
- shared component styles
- icon types, the scoped default Lucide set, and the icon build command

It must not import Svelte or React.

### `@inflatable-cookie/poodle-svelte`

The Svelte package owns Svelte 5 component shells and idiomatic bindings. It
uses the core behavior and styles rather than defining a second visual or state
model.

### `@inflatable-cookie/poodle-react`

The React package owns React component shells over the same core behavior and
styles. It is currently experimental, but its contracts are the same contracts
used by Svelte and native renderers.

Bits Svelte may be used inside the Svelte implementation where useful. It is
not part of Poodle's public contract and does not constrain other runtimes.

## Native Architecture

Native rendering is split into contracts, a shared renderer, and backends.

### Contracts

The Rust contracts under `packages/contracts/` include:

- `poodle-specs` for component input data
- `poodle-headless` for reusable interaction rules
- `poodle-tokens` for generated themes and token resolution
- `poodle-layout`, `poodle-style`, and `poodle-events` for shared vocabulary
- `poodle-node` for renderer-neutral output trees and interaction intents

These crates do not own GPUI or Jetstream widgets.

### Shared renderer

`poodle-render` is the single Rust component implementation. Its functions
accept a spec, a theme provider, and handlers where required, then return a
`poodle-node` tree. Component composition, token selection, state treatment,
and interaction intent belong here when they are shared across native targets.

### GPUI

`poodle-gpui` maps common layout and style vocabulary and provides the GPUI
theme provider. `poodle-gpui-node-backend` interprets node trees as GPUI
elements and owns backend-specific input, text editing, and event plumbing.

### Jetstream

`poodle-jetstream` maps common layout and style vocabulary and provides the
Jetstream theme provider. The `jetstream-poodle` crate in the Jetstream
repository converts Poodle nodes into Jetstream `JsEl` trees at the engine
boundary.

A native component should not be reimplemented separately in both backends.
Shared composition belongs in `poodle-render`; only runtime interpretation
belongs in a backend.

## Token Architecture

`packages/tokens/schema/` is the canonical W3C DTCG source. The token build
emits CSS and TypeScript for web packages and Rust for native crates. Themes,
density modes, and control sizes are generated from the same source.

Components consume semantic roles such as background, text, border, spacing,
and control size. They do not hardcode theme values. The complete build and
package flow is defined in
[002 Token System and Package Layout](002-token-system-and-package-layout.md).

## Component Layers

Poodle components fall into three ownership layers:

1. **Foundation** — tokens, layout primitives, controls, feedback, overlays,
   and navigation primitives.
2. **Reusable composites** — fields, forms, cards, lists, tables, pickers,
   media surfaces, and reusable page structures.
3. **Workstation shells** — panels, split regions, inspectors, browsers, and
   other general desktop or pro-tool structures.

The third layer is still reusable system UI. Product-specific editors,
transport controls, domain models, and workflow screens remain outside Poodle.

## Host and Application Boundaries

Applications own routing, data fetching, persistence, authorization, product
language, and service orchestration. Poodle may expose state and events needed
to implement those workflows, but it does not own them.

Underlay-facing integration is isolated in `packages/bridges/underlay/` and in
Underlay-owned adapters. Underlay application code imports Underlay APIs; it
does not acquire a direct dependency on Poodle. This allows either side to
change without leaking design-system implementation details into applications.

## Extension Rules

- Add a component only when its semantics are reusable across products.
- Change the contract before changing observable behavior in one renderer.
- Put cross-native composition in `poodle-render`, not in both backends.
- Put cross-web behavior and styles in `poodle-core`, not in both frameworks.
- Keep runtime escape hatches narrow and document any parity they give up.
- Keep generated artifacts derived from the canonical token source.
