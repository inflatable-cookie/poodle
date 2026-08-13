# 066 Executable Component Conformance

Status: provisional — g14 pilot contract
Updated: 2026-08-14
Owner: Poodle core
Depends on: `../architecture/009-cross-runtime-component-conformance.md`,
`../contracts/001-working-rules.md`

## Purpose

Define the smallest code-centred system that can make portable drift fail
across Svelte, React, GPUI, and Jetstream. This spec is provisional until the
g14 profile pilots pass. Architecture 009 fixes the invariants; g14 may revise
the storage format and tooling.

## Authored Surfaces

### Portable interface module

Pilot direction: a constrained TypeScript data module is the portable
interface authority because Svelte is the reference and both web runtimes can
consume it directly. It exports a serializable `defineComponentInterface({...})`
value and inferred portable types. It declares data, not behaviour:

- prop names, portable types, defaults, and controlled-state pairs
- semantic events, payloads, and ordering requirements
- named composition regions, multiplicity, and payload shape
- observable states, public methods, and capability requirements
- component profile and stable part identifiers

Codegen emits the Rust-facing declaration and a neutral schema artifact.
Svelte and React import the inferred portable prop/event types. Their Svelte
`Snippet` and React node/function carriers adapt the declared composition
regions locally; those carrier types are not portable declarations.
`poodle-specs` consumes generated Rust declarations. Framework-only props and
methods remain in a small named extension beside the adapter.

This direction fails the pilot if it needs arbitrary TypeScript execution, if
Rust output loses idiomatic/exhaustive types, or if total authored plus
generated interface code is not smaller than the portable declarations it
replaces. A neutral data IDL is the fallback, not an automatic second model.

### Component case module

Cases are authored with a typed TypeScript builder validated against the
portable interface. The build emits deterministic, versioned JSON consumed by
all runners. A case is serializable data:

```ts
componentCase({
  id: "button/default",
  fixture: { props: { tone: "neutral" }, regions: { default: "Run" } },
  specimen: { group: "Tone", caption: "Neutral", axes: ["theme", "size"] },
  steps: [
    action.press("root"),
    expect.event("press"),
    expect.part("root", { role: "button", name: "Run" }),
  ],
});
```

The builder supports literals, structured fixture values, named regions,
linear actions, and assertions. It has no condition evaluator, loops,
arbitrary callback bodies, product state, or runtime branches.

Allowed semantic actions:

- press, pointer enter/leave, pointer down/up
- key press and text entry
- focus, blur, dismiss, select, and scroll
- named host command for composite harnesses

Allowed assertions:

- state and controlled value
- semantic event payload and order
- focused part and focusability
- role, accessible name, state, and relationships
- part tree, text/icon identity, and token roles
- geometry with a named tolerance
- capability exercised

### Runtime harness

Each runtime implements the same harness boundary:

1. mount the component from the fixture;
2. map stable part IDs to runtime objects;
3. perform semantic actions through the real runtime path;
4. collect events and normalized observations;
5. render the case's specimen metadata in its preview.

Harnesses may translate an action to native APIs. They may not restate fixture
content, expected results, or specimen grouping.

## Observation Format

`component-observation.v1` is deterministic JSON. Nodes contain stable part
ID, parent part ID, semantic role/name/state, text or icon ID, token roles,
resolved color/border/radius/typography channels, focus state, interaction
capabilities, logical bounds, clipping, and layer order. The trace contains
ordered semantic events and capability evidence. Shared capture IDs bind
image evidence when those properties cannot describe the visible result.

Runtime-private fields are excluded before comparison. Normalization cannot
rename events, drop nodes, invent roles, change state, or mask missing
interactions. Tolerances are assertion-local and named.

## Specimen Projection

Every authored case is eligible for a specimen view. Specimen metadata defines
groups, captions, axes, ordering, and capture IDs once. Four thin preview
adapters render that structure with their runtime component.

Interactive cases use the same fixture and action vocabulary as tests. A
preview may expose controls for manual exploration, but those controls do not
become a second canonical fixture.

## Evidence And Gates

The pilot must deliver these selectors:

- `conformance:build` — deterministic artifacts; orphan-aware
- `conformance:check` — read-only drift check
- `conformance:test` — run selected cases in available runners
- `conformance:complete` — fail incomplete required runtime coverage
- `conformance:cost` — report authored, generated, replaced, and adapter LOC

Every mechanism needs a planted-failure proof. The standing docs/CI surface
must call the read-only checks. A selector excluded from `docs:check`,
`ci:web`, and `ci:native` is not enforcement.

## Pilot Profiles

The g14 pilot proceeds in increasing complexity:

1. `Button` — display/control kernel
2. `RangeSlider` — controlled multi-part control
3. `Tabs` — identified collection and keyboard navigation
4. `Popover` — overlay, dismissal, placement, focus
5. `TextInput` — text editing and runtime capability boundary
6. `HistoryCenter` — host-coordinated composite

Each profile reuses the same authority, case, observation, harness, specimen,
and completion shapes. A profile may extend typed action/assertion vocabulary
only when the prior vocabulary cannot express a contract requirement.

## Cost And Replacement Rules

Count all schema, generator, generated artifact, runner, adapter, test, and
wiring code. Do not compare only authored model lines against deleted
specimens. For each pilot report:

- new authored and generated lines
- deleted/replaced declarations and specimen lines
- runtime adapter delta
- defects caught that existing gates missed
- ongoing per-component authoring cost

Stop and reassess when the mechanism grows faster than the duplication it
removes, requires behaviour in the schema, or needs component-specific code in
generic runners.

## Completion Semantics

Runtime states are `passing`, `failing`, `missing`, or `not-applicable`.
`not-applicable` is valid only for a contract-classified platform extension.
`missing` may carry a reason but remains incomplete. There is no
`known-divergence` pass state.

## Cleanup Rule

The g13/g14 experiments are evidence, not standing architecture. Before broad
rollout, classify each generated surface, gate, fixture model, baseline, and
test method as `keep`, `adapt`, `replace`, or `retire`, with one owner and one
canonical gate per claim. Retired machinery and stale docs leave the active
path before generation closeout.
