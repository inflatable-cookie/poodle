# 010 Native Presentation Construction Context

Status: active
Accepted: 2026-08-23
Owner: Poodle core
Depends on: `001-poodle-system-shape.md`,
`../contracts/001-working-rules.md`,
`../contracts/components/ui-presentation-provider.md`,
`012-semantic-motion-policy.md`
Decision authority: operator approval after the `g15.043` impact audit

## Decision

Shared Rust composition receives one explicit `RenderContext`. It carries:

- a borrowed `ThemeProvider` for token resolution; and
- copyable effective UI-presentation defaults: semantic control-size scale and
  density; and
- the copyable effective `MotionPolicy` from architecture 012.

Every public component renderer in `poodle-render` receives that context.
Component specs retain omission with `Option<ControlSize>` and
`Option<ControlDensity>`. A renderer resolves an omitted size from the context,
then applies the component's semantic size role to that inherited scale. An
explicit size is already the final component size and bypasses role mapping;
an explicit value always wins, including an explicit `md` or `default` inside
a non-default scope.

`UiPresentationProvider` is a construction boundary, not a painted node. It
creates a nested context, invokes a child-building closure with that context,
and returns the resulting child unchanged. Nested providers replace the two
presentation defaults only for construction performed inside their closure;
they preserve motion unchanged. `MotionPolicyProvider` uses the same
construction boundary but combines its request with the inherited policy by
restriction and never re-enables motion.

This is a clean pre-v1 Rust API break. There is no old-signature wrapper,
alias, default-value heuristic, or ambient fallback.

## Why Construction Time

The shared native flow is:

```text
Spec + RenderContext -> poodle-render -> Node -> backend
```

Size and density alter component recipes: padding, gaps, typography, control
geometry, and supporting visuals. Those decisions have already happened once
a `Node` exists. A backend or post-build provider cannot recover them.

The context must also cross composition boundaries. A component that creates a
new presentation scope for host content receives a child builder, not an
already-built child `Node`. The builder is invoked immediately with the scoped
context. Prebuilt children remain valid only where the parent does not create a
new scope for them.

## Resolution Rules

At the root:

- size scale is `md`;
- density is `default`; and
- motion policy is `full` unless the host supplies an explicit preference or
  capture policy.

For a component:

1. use an explicit `size` unchanged when present;
2. otherwise map the context size scale through the component's `sizeRole`;
3. use an explicit `density` when present, otherwise the context density.

For a nested provider:

- its `sizeScale` and `density` replace the parent defaults;
- a motion provider resolves the more restrictive parent or child policy;
- a presentation provider leaves the inherited motion policy unchanged;
- its wrapper adds no layout, paint, accessibility node, focus target, or
  interaction state;
- exiting the child closure restores the parent context by ordinary borrowing,
  not mutation.

An explicit child reset is therefore unambiguous:

```text
root md/default
  provider xl/comfortable
    omitted child          -> xl/comfortable
    explicit md/default    -> md/default
    provider sm/compact
      omitted child        -> sm/compact
```

## Ownership

- `poodle-adapter::ThemeProvider` remains token-only.
- `poodle-render::RenderContext` owns native construction context and resolver
  helpers.
- `poodle-render` owns provider scoping and every composite's context flow.
- `poodle-node` remains resolved renderer-neutral output and gains no provider
  metadata.
- GPUI interprets the resulting nodes. It does not implement presentation
  inheritance or motion-preference discovery.
- Jetstream follows its current admission status. Required compile adaptation
  may consume the shared context API, but this decision does not claim
  Jetstream parity.
- Svelte and React retain their framework-native context implementations and
  shared CSS. No cross-language context representation is introduced.

## Migration Boundary

The accepted audit found:

- 125 native component spec files with semantic `ControlSize` or
  `ControlDensity` inputs;
- 103 shared-render modules reading those inputs;
- 168 shared-render modules accepting `ThemeProvider` directly;
- 113 shared-render modules constructing descendant specs; and
- 14 paired-web components that create an internal
  `UiPresentationProvider` boundary.

The migration is atomic on `main`. It changes the specs, renderer entrypoints,
composite calls, native preview facades, tests, and current in-repository
consumers together. A standing source audit rejects a future concrete semantic
presentation default or a public component renderer that bypasses
`RenderContext`.

The 14 internal-provider owners require explicit review:

- ActionDiscoveryPanel
- AppHeader
- BlockEditor
- CommandPalette
- EditableList
- Field
- FilterToolbar
- LogList
- MarkdownEditor
- MediaBrowsePanel
- MediaPicker
- MediaPreview
- PageHeader
- RelationPicker

Where their native counterpart supplies scoped host content, its slot changes
to immediate context-aware construction. This is not a general scene or
component representation.

## Rejected Alternatives

### Seed concrete specs before builder calls

Rejected. Correctness depends on call order. Seeding after an explicit
`md` / `default` assignment silently destroys the override.

### Carry presentation through ThemeProvider

Rejected. Theme identity and subtree presentation state have different
lifetimes and ownership. The token provider stays token-only.

### Store ambient state globally or per thread

Rejected. It is unsafe across nested construction, parallel tests, and future
multi-window hosts. Context is an explicit borrowed value.

### Mutate Node output or teach GPUI the provider

Rejected. Component-specific recipe decisions are already resolved, and a
backend implementation would duplicate shared composition.

### Build a universal native scene/component tree

Rejected. A short-lived child-building closure exists only to carry native
construction context. Contracts, specs, and real runtime components remain the
authorities.

## Completion Evidence

The implementation must prove root defaults, outer scope, nested scope,
explicit reset, a primitive, a form control, a composite with internal child
construction, and a host-provided scoped slot. Mounted GPUI evidence must show
the resolved geometry without adding a provider node to layout or the
accessibility tree.
