# 063 Rust-authored Component And Scene IR

Status: amended — narrowed to vocabulary by the `g13.008` **revise** verdict
Updated: 2026-08-13
Owner: Poodle core
Depends on: `062-headless-core-and-dual-layer-strategy.md`,
`../architecture/001-poodle-system-shape.md`,
`../architecture/006-headless-core-and-machine-model.md`,
`../architecture/007-appearance-recipe-contract.md`

## Scope, after the g13.008 revise verdict

The pilot ran to completion and the verdict is **revise**
(`../roadmaps/g13/pilot-verdict-evidence.md`). This spec is narrowed
accordingly, and the narrowing is the point — the original scope was measured
and did not hold.

**In scope — cross-runtime vocabulary, with drift gating.** Part names,
`data-*` attribute names, value domains, axes, recipe hooks, and capability
declarations: authored once in Rust, emitted to all four runtimes, and pinned
by `ir:check`. This is what the pilot demonstrably delivered.

**Out of scope — behaviour.** The IR does not describe transitions, effects,
derived values, per-attribute derivation, or framework lifecycle, and **will
not grow an evaluator**. `expr.rs` and the unused expression vocabulary are
removed by `g13.017`. Across three components of rising difficulty the IR
executed nothing: every prop was declared, zero were executed.

**Out of scope — replacing implementations.** Consuming a definition is
additive. Measured: the nine pilot files grew 3,672 → 4,637 lines while
removing zero duplication. Any future claim that generation will shrink a
component needs new evidence, not this spec.

**Amendments the pilot named**, both vocabulary rather than behaviour, owned by
`g13.018`:

1. Per-runtime capability expression, **including absence**.
   `CapabilityRequirement` currently carries only `capability` and a prose
   `purpose`, so ownership is untyped and "this runtime does not have it"
   cannot be said at all — Jetstream renders a text field nobody can type into,
   declared identically to GPUI, which implements the whole editing model.
2. Repeated anatomy with per-item identity. `PartKind::Repeated` requires a
   `List` prop and yields identical instances; its own doc comment names the
   two RangeSlider thumbs as the example it cannot serve, and both web and
   native renderers hard-code "two".

The pass conditions and stop conditions below are retained **as the pilot's
record**. Condition 5 failed; condition 6 was never tested. They are not a
live checklist.

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

### The bounded expression vocabulary (normative)

Added 2026-08-11. The spec asked for "a bounded expression vocabulary" without
bounding it; `g13-b011` hit the gap on `CROSS-20`
(`isUnavailable = disabled || loading`). The bound below is derived from every
`$derived` in the three pilot components, not chosen in the abstract.

**Operands.** A reference to one of: a declared prop, a declared state field, a
VisualState projection field, a slot's presence, a resolved axis value; or a
literal boolean, integer, string, or shared-type member.

**Operators.** Exactly these, and no others:

| Group | Operators |
|---|---|
| Logical | `and`, `or`, `not` |
| Equality | `eq`, `ne` — against a literal or shared-type member |
| Nullability | `is_null`, `is_present`, `coalesce` |
| Ordering | `gt`, `gte`, `lt`, `lte` — integers only |
| Emptiness | `is_empty` — strings and collections |
| Selection | `if / then / else` |

**Excluded, deliberately:** arithmetic (`+ - * /`), string manipulation,
interpolation and formatting, function calls of any kind, iteration, recursion,
variable binding, and indexing or field access beyond the declared references
above.

**Expressions are total, pure, and typed.** They always evaluate, have no side
effects, and are type-checked against declared prop types during IR validation
— a malformed expression fails at its authored source, not at generation time.

**Where expressions may appear.** Only in: state-derived attribute emission
conditions and values, part render conditions, prop default and axis fallback
resolution, and guard conditions on transitions and effect-intents. They may
not compute values that feed a behaviour machine.

**If you need something excluded, it is not an expression.** The three escapes
already in this spec absorb every case found in the pilots:

- Needs arithmetic, string building, or normalization → it is a **VisualState
  projection field** or a **conformance vector**. `visualState.lowerNorm * 100`,
  the adornment-count padding of `TXT-16`, and `${charCount}/${maxLength}` are
  projection and formatting concerns; `safeSliderMax`, `normalizeRangeValue`,
  `slugify`, and `rangeSliderVisualState` are machines.
- Needs the environment → it is a **named adapter capability**.
- Needs to differ per runtime → it is an **explicit runtime extension**.

This keeps arithmetic out of the shared expression language entirely, which is
what makes the vocabulary total and every target able to evaluate it without a
runtime.

## Pilot Rules

- **IR-01 — Rust authority:** renderer-independent component and scene
  declarations are authored once in Rust.
- **IR-02 — Serializable boundary:** shared definitions contain typed,
  serializable data and bounded expressions only.
- **IR-03 — No function transpilation:** arbitrary Rust execution is never
  translated into TypeScript.
- **IR-04 — Semantic authoring layer:** the authoring IR sits above resolved
  `poodle-node` output and preserves web-native semantics.
- **IR-05 — Adapter ownership:** focus, IME, portals, measurement, pointer
  capture, text systems, accessibility projection, and other environment work
  remain runtime capabilities.
- **IR-06 — VisualState purity:** drawing consumes serializable state and does
  not read machine state or own hit-testing/input.
- **IR-07 — Deterministic generation:** emitted artifacts are reproducible,
  versioned, source-linked, and drift-checked without worktree mutation.
- **IR-08 — Typed capability gaps:** missing runtime capabilities are declared
  and evidenced, never silently ignored.
- **IR-09 — Stable migration:** pilot and rollout preserve existing public
  component contracts unless a separate contract change is approved first.
- **IR-10 — Four-runtime proof:** registration alone is not parity; executed
  semantic, interaction, accessibility, recipe, axis, and visual evidence is
  required.
- **IR-11 — Pilot gate:** broad migration cannot start before Button,
  RangeSlider, and TextInput pass and `g13.008` records **adopt**.
- **IR-12 — No early packages:** crate placement is decided from the g13.001
  authority inventory before implementation packages are created.

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
- **references to shared types, and the subset of a shared type this component
  permits** (added 2026-08-11, see below)

### Shared types and permitted subsets

Added from `g13.001` evidence, after this spec was first written.

A named enumerated type used by more than one component is defined **once** and
referenced. Per-component prop lists are not sufficient: `ButtonTone` fragmented
across three contracts into three disagreeing unions, `OverlayPlacement`
fragmented the same way across three more, and `g13-b007` found 8 further
enumerated shared types with no definition anywhere in `docs/`.

A component may permit a **subset** of a shared type's members, and that
constraint is first-class in the IR and must survive into every generated
artifact. The motivating case is exact: `ButtonSpec` accepted
`ButtonTone::Success` while `button.md` permitted only three tones, and the
inverse held for IconButton — a value that type-checked, resolved correctly in
the Rust renderer, and silently rendered as default on the web.

This is the most direct evidence the pilot has that one source prevents real
drift, and the schema is required to express it. The docs-side counterpart is
`docs/contracts/004-shared-control-types.md`.

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

`g13.008` recorded **revise** on 2026-08-13. Evidence:
`../roadmaps/g13/pilot-verdict-evidence.md`.

- ~~**adopt**~~ — not available: pass condition 5 failed.
- **revise** — *recorded.* This spec is narrowed to vocabulary (see "Scope,
  after the g13.008 revise verdict" above). The runway is `g13.017`–`g13.020`.
- ~~**reject**~~ — defensible on the same evidence, declined because the
  propagation and drift machinery works and is cheap at the vocabulary scope.

`g13.009`–`g13.016` are closed, not deferred. Architecture 001/006 were **not**
amended: nothing here is promoted to stable while the model stays provisional
at vocabulary scope.

Spec 062 remains correct for arbitrary framework behavior and for its Mitosis
rejection. This spec supersedes only its assumption that declarative codegen
should remain deferred.
