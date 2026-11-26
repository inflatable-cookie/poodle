# Contracts

Status: active

Contracts are the docs-first source of truth for Poodle components.

Each component contract describes one semantic surface that Svelte, React,
GPUI, and Jetstream implementations can satisfy against the same review
checklist.

## Structure

```text
docs/contracts/
  README.md
  template/
    component-contract-template.md
  components/
    ...
```

## Grouping Rules

- `components/` for all reusable component contracts, from low-level primitives
  through higher-order application composites
- workstation semantics are currently expressed through the shared contract
  crates plus the component docs above, rather than a separate
  `docs/contracts/workstation/` subtree

App-specific DAW widgets do not belong in this contract surface. They build
above it in downstream repos such as Loophole.

## Cross-Cutting Rules

Rules that bind more than one contract live beside them, and a component
contract references the rule rather than restating it:

- [working rules](../knowledge/contracts/working-rules.md) — authority, validation and parity rules
- `002-anchored-overlays.md` — every anchored surface portals to the theme root
  and is positioned in viewport coordinates
- `003-native-accessibility.md` — both native runtimes project roles, names
  and states into AccessKit: Jetstream live, GPUI with a bounded live proof
- `004-shared-control-types.md` — types shared by more than one component
  contract are defined once and referenced, never restated with fewer members

## Current Contracts

- [Component index](components/README.md) — every component contract, grouped by family
- [Supported core behaviour](core-behaviour.md) — exports for building custom web controls
- [Component contract template](template/component-contract-template.md)
