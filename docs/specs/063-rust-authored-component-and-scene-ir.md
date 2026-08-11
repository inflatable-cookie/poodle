# 063 Rust-authored Component And Scene IR

Status: provisional — governed by the g13 pilot
Updated: 2026-08-11
Owner: Poodle core
Depends on: `062-headless-core-and-dual-layer-strategy.md`,
`../architecture/001-poodle-system-shape.md`,
`../architecture/006-headless-core-and-machine-model.md`,
`../architecture/007-appearance-recipe-contract.md`

## Purpose

Make Rust the source of truth for Poodle's renderer-independent component and
composition definitions. Generate neutral TypeScript artifacts for the web;
interpret the same Rust definitions through the native renderer stack.

This spec governs the g13 pilot. It does not change stable architecture until
the Button, RangeSlider, and TextInput proofs pass and `g13.008` records the
verdict.

## Decision

Poodle will author a constrained declarative IR in Rust and generate other
runtime inputs from it.

```text
Rust source of truth
  component IR + scene IR + schemas + conformance vectors
                         |
                    poodle-codegen
              /            |             \
     TypeScript IR     JSON evidence      docs/registries
        /      \                              |
    Svelte    React                     parity checks

Rust component IR -> poodle-render -> poodle-node -> GPUI / Jetstream
```

Rust authority is chosen for exhaustive enums, explicit ownership, stable
serialization, compile-time validation, and direct native consumption. The
web receives generated typed data, not a second hand-maintained model.

## Hard Boundary: Data, Not Rust Transpilation

The shared source may contain only typed, serializable declarations and a
bounded expression vocabulary. Arbitrary Rust functions, closures, trait
objects, runtime borrowing, and backend calls are not cross-compiled to
TypeScript.

Cross-runtime behavior must be represented as one of:

- a declarative transition, guard, or effect-intent expression in the IR
- shared conformance vectors implemented by each runtime machine
- a named adapter capability such as focus, measurement, pointer capture,
  text editing, portal placement, timers, or announcements
- an explicit runtime extension with a documented parity consequence

This narrows the compiler problem enough to keep generated output dependable.

## Component IR

Each component definition carries stable identifiers for:

- public props, defaults, types, controlled state, events, slots, and parts
- semantic anatomy and parent/child constraints
- states and state-derived attributes
- accessibility intent, keyboard commands, and adapter capabilities
- semantic token and appearance-recipe hook references
- size, density, orientation, direction, and contrast axes
- renderer-neutral render nodes and conditional/repeated composition
- contract and specimen references

The component IR is above `poodle-node`. `poodle-node` remains resolved native
output. It is not the universal authoring model: web lowering must retain DOM
semantics, CSS cascade and recipes, framework lifecycle, slots, portals, and
native form behavior.

Default drawing remains a pure projection of serializable state. Input,
hit-testing, focus, accessibility, and environment effects stay in machines
and adapters.

## Scene IR

Scene IR defines how components compose into preview shells, specimen pages,
examples, size/density matrices, and later reusable interface fixtures. It
contains:

- component references and typed prop bindings
- layout nodes, text, groups, loops, conditions, and named slots
- local fixture state and semantic event wiring
- theme, size, density, orientation, and contrast axes
- interaction scenarios and stable capture identifiers
- declared runtime capability requirements

Scene IR is not an application framework. Routing, persistence, data fetching,
authorization, product state, arbitrary host callbacks, and DAW-specific
models remain outside it.

## Lowering And Runtime Ownership

### Rust

- `poodle-ir` owns versioned serializable definitions and validation.
- `poodle-codegen` validates the graph and emits deterministic artifacts.
- `poodle-render` lowers component IR and resolved VisualState to
  `poodle-node`.
- GPUI and Jetstream interpret nodes and own runtime capabilities only.

Crate locations are frozen by `g13.001` after checking current workspace and
publication boundaries. Names above describe responsibilities, not permission
to create packages early.

### TypeScript And Web

- Generated TypeScript contains discriminated unions, readonly definitions,
  schemas where required, stable registries, and conformance fixtures.
- A small shared web interpreter lowers semantic nodes to framework adapter
  operations.
- Svelte and React own idiomatic lifecycle, refs, context, snippets/children,
  DOM events, form integration, focus, portals, measurement, and text editing.
- Generated files are never hand-edited.

The compiler may generate thin registries and static shells. It must not emit
large framework source trees whose generated lifecycle code becomes a new
debugging surface.

## Authoring Form

Start with ordinary Rust types and constructor helpers. Add macros only where
the pilot proves they materially improve authoring without hiding validation
or diagnostics. A macro must expand to the same `poodle-ir` structs accepted
by serialized fixtures.

Schema evolution is explicit. Every emitted artifact carries an IR version;
breaking changes require a migration or a deliberate pre-1.0 regeneration.
Output ordering and formatting are deterministic.

## Generated Artifact Contract

- Commit generated TypeScript, JSON evidence, registries, and documentation
  fragments needed by source consumers and CI.
- Provide Effigy `ir:build` and `ir:check` selectors.
- `ir:check` regenerates in isolation and fails on drift without rewriting the
  worktree.
- Generated headers name the source definition and generator version.
- One Rust definition change must update every expected target in one build.
- Hand-written runtime extensions are inventoried and checked for an owning IR
  declaration.

## Capability And Escape-hatch Rules

Capabilities are named, typed, and visible in the definition. Backends report
support explicitly. A missing capability may produce a documented degradation
or make the component unavailable; it may not silently drop behavior.

An escape hatch is acceptable only when it cannot be expressed without
damaging runtime-native semantics. It must include:

- owning runtime and reason
- semantic effect on parity
- test or evidence surface
- removal condition, or a statement that the difference is intentional

## Pilot

The proof proceeds in increasing difficulty:

1. Shared preview shell and Button specimen: composition, props, slots,
   recipes, theme selection, axes, events, and four-runtime rendering.
2. RangeSlider: controlled state, pointer/keyboard behavior, multiple thumbs,
   orientation, embedded treatment, and unipolar/bipolar geometry.
3. TextInput: browser-native input, IME, selection, focus, validation, and
   native text-system capability boundaries.

The pilot passes only when:

- changing one Rust definition deterministically updates all four previews
- Svelte and React retain idiomatic runtime semantics and public APIs
- GPUI and Jetstream consume the shared Rust path without component forks
- interaction, accessibility, recipe, size, density, and visual evidence pass
- generated code is smaller and easier to inspect than the duplicated source
  it replaces
- diagnostics point to the authored definition, not only generated output

## Stop Conditions

Stop and return to the spec when:

- web semantics require lowering from `poodle-node`
- the IR needs arbitrary executable Rust to describe cross-runtime behavior
- generated framework lifecycle code becomes the primary implementation
- focus, IME, portals, measurement, or accessibility are hidden in drawing
- a runtime needs an untyped side channel to render or interact
- a pilot passes only by weakening an existing component contract

Failure of one expression shape is not permission to fork four definitions.
Record the missing capability or revise the IR.

## Promotion

`g13.008` records the evidence and one verdict:

- **adopt** — promote the durable authority and lowering model into
  architecture 001/006 and working rules, then unlock the rollout cards
- **revise** — amend this spec and run another bounded pilot
- **reject** — retain the evidence, close the compiler runway, and recompile
  the remaining parity plan without IR codegen

Spec 062 remains correct for arbitrary framework behavior and for its Mitosis
rejection. This spec supersedes only its assumption that declarative codegen
should remain deferred.
