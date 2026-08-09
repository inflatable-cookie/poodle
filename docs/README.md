# Poodle Documentation

Use this page to find the shortest path from your question to the authoritative
answer. You do not need to read the roadmap or project logs to use Poodle.

## Adopt Poodle

Choose the runtime used by your application:

- [Svelte developer guide](guides/svelte-developer-guide.md) — package setup,
  themes, icons, components, and application integration
- [React package guide](../packages/react/components/README.md) — current React
  surface and experimental-package constraints
- [GPUI developer guide](guides/gpui-developer-guide.md) — Rust contracts,
  themes, node rendering, and the GPUI backend
- [Jetstream developer guide](guides/jetstream-developer-guide.md) — Rust
  contracts, themes, node rendering, and Jetstream conversion
- [Application pattern recipes](guides/README.md) — forms, lists, dialogs,
  media workflows, and admin shells

Poodle is currently a pre-1.0 source preview. Public-intent packages are still
private and are consumed through workspace or file dependencies rather than a
public registry.

## Understand the System

- [System shape](architecture/001-poodle-system-shape.md) explains contracts,
  renderer boundaries, parity, and application ownership.
- [Tokens and package layout](architecture/002-token-system-and-package-layout.md)
  explains the token build, published surfaces, themes, and runtime packages.
- [Component documentation structure](architecture/003-component-docs-ia-and-implementation-substrates.md)
  explains how contracts and implementation evidence relate.
- [Underlay bridge ownership](architecture/004-underlay-bridge-and-adapter-ownership.md)
  defines the host-adapter boundary.

## Look Up Component Behavior

[Component contracts](contracts/components/README.md) are the source of truth
for public inputs, states, events, accessibility, layout, and token usage. Use
them when integrating a component, comparing runtimes, or proposing an API
change.

Contracts describe observable behavior. Framework and engine implementation
details may differ when the contract permits it.

## Contribute

- [Working rules](contracts/001-working-rules.md) define cross-runtime and
  contract-first expectations.
- [Architecture index](architecture/README.md) identifies structural authority.
- [Specs index](specs/README.md) collects repository-wide normative rules and
  generated baselines.
- [Roadmaps](roadmaps/README.md) contain current milestone planning.

Run documentation locally with:

```sh
bun install
effigy docs:dev
```

Validate documentation changes with:

```sh
effigy docs:check
```

## Project Record

The remaining sections preserve decision and delivery context:

- `vision/` — long-range intent and scope
- `roadmaps/` — milestone sequencing and active work
- `logs/` — completed work and validation evidence
- `research/` — external comparisons and early investigation
- `handoffs/` — point-in-time continuation briefs

These records are useful to contributors, but they are not part of the operator
learning path and may describe superseded implementation states.
