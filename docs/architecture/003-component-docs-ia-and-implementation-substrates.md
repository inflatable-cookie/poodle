# 003 Component Documentation and Implementation Substrates

Status: active
Updated: 2026-08-09
Depends on: [001 Poodle System Shape](001-poodle-system-shape.md),
[002 Token System and Package Layout](002-token-system-and-package-layout.md)

## Purpose

Component contracts define observable semantics independently of a renderer.
Implementations may use different substrates as long as they preserve those
semantics and use the shared token system.

## Documentation Authority

All reusable component contracts live together under
`docs/contracts/components/`. A contract covers purpose, anatomy, inputs,
states, events, accessibility, layout, token use, runtime notes, parity, and
known differences.

The [component contract template](../contracts/template/component-contract-template.md)
is the starting shape for a new component. The
[component index](../contracts/components/README.md) lists the current surface.

Contracts are normative. Preview examples, parity reports, accessibility
artifacts, and implementation tests are evidence against them.

## Web Substrates

`@inflatable-cookie/poodle-core` owns framework-free state machines, prop
getters, tokens, styles, and icon infrastructure. Svelte and React components
wrap that shared surface with framework-native composition and events.

Bits Svelte may provide internal accessibility, focus, overlay, or compound
state machinery. Poodle still owns public names, behavior, tokens, and docs;
applications must not need Bits imports or types.

Shared CSS uses Poodle classes, semantic token variables, and data attributes.
Do not duplicate component styling between Svelte and React.

## Native Substrates

Rust component specs are renderer-neutral data. `poodle-render` turns a spec,
theme provider, slots, and handlers into a `poodle-node` tree. GPUI and
Jetstream backends interpret that tree using runtime-native layout, drawing,
input, text, focus, and lifecycle facilities.

Shared composition and appearance belong in `poodle-render`. A backend-specific
branch is justified only by a runtime capability or limitation, and any
observable difference must be recorded in the contract or parity register.

## Composition

Contracts model semantic content and named composition points. Framework
snippets, React nodes, and Rust node slots may differ in syntax while exposing
the same role. Application-specific data loading, commands, persistence, and
domain rendering stay outside the contract.

## Review Rule

Review a component in this order:

1. contract completeness
2. shared state and rendering behavior
3. runtime-native accessibility and interaction
4. token and layout parity
5. preview and generated evidence

A polished specimen cannot compensate for a missing contract behavior.
