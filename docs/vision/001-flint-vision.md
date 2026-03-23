# 001 Flint Vision

Status: active
Updated: 2026-03-11

## Purpose

Flint is a generalized UI system for teams that need the same documented
component contract implemented in both Svelte and GPUI first, while keeping the
door open for additional implementations later.

Its job is to make tokens, primitives, and reusable composites explicit enough
that:

- GPUI desktop apps can adopt them,
- Svelte apps can adopt them,
- future targets such as React or other desktop UI kits can adopt them without
  forcing a redesign of the core contract,
- Underlay can consume them internally without leaking Flint into app code,
- app-specific systems such as Loophole's DAW widgets can build above them
  instead of re-inventing lower-level UI contracts.

## Core Promise

Every Flint component should be:

- documented before or alongside implementation,
- defined by one contract,
- implemented in Svelte and GPUI against the same semantic rules,
- backed by one shared token vocabulary,
- backed by theme definitions that can be translated into both Svelte and GPUI
  consumers from the same source,
- designed so additional implementation targets can be added later without
  changing the semantic contract model,
- explicit about intentional platform deltas.

## What Flint Must Own

- semantic design tokens
- semantic theme definitions that are portable across framework runtimes
- token artifact emission for CSS, TypeScript, and Rust consumers
- an implementation-neutral contract model that is not tied only to Svelte or
  GPUI internals
- foundational UI primitives
- reusable application/productivity composites
- reusable workstation-shell composites
- documentation, parity rules, and implementation guidance for both stacks

## What Flint Must Not Own

- Loophole-specific DAW widgets such as transport, timeline, mixer, automation,
  or plugin panels as first-class core components
- Underlay product-app APIs
- Bits Svelte APIs as the public contract
- browser-only shortcuts that cannot translate to GPUI
- GPUI-only shortcuts that cannot translate back to Svelte

## Ecosystem Posture

### Underlay

Underlay should be able to consume Flint tokens and components through
Underlay-owned bridges and wrappers. Underlay apps should not need to import or
reason about Flint directly.

### Loophole

Loophole-specific components should be built from Flint tokens and Flint
sub-components, but they should live in Loophole-owned packages or repos rather
than inside Flint core.

### Svelte

Bits Svelte can remain a useful implementation substrate where it improves
quality or speed, but Flint must define its own contract, docs, and token model.

### GPUI

GPUI implementations should be native GPUI components, not a lowest-common-
denominator translation of the web implementation.

## Success Criteria

Flint is successful when:

- the same tokens exist in CSS, TypeScript, and Rust-friendly forms
- one semantic theme definition can be emitted for both browser/Svelte and
  GPUI/Rust consumers without downstream repos redefining the theme separately
- new components are introduced through docs-first contracts
- Svelte and GPUI implementations can be compared against one parity checklist
- future implementations can be added later without rewriting token or contract
  meaning
- Underlay can adopt Flint internally without breaking existing app APIs
- Loophole can build higher-order DAW widgets without needing to redefine
  buttons, panels, tabs, menus, forms, or layout semantics

## Failure Modes To Avoid

- turning Flint into a Loophole-only repo
- turning Flint into a web-only component library
- turning Flint into a docs-only catalogue without implementation discipline
- forcing downstream teams to hand-translate the same theme into CSS and Rust
  separately
- baking Svelte-specific or GPUI-specific assumptions into the canonical token
  and contract layers
- treating Bits or any other dependency as the contract source of truth
- requiring downstream apps to learn Flint-specific abstractions they should not
  need

## Next Task

Translate this vision into package boundaries, token rules, and component-layer
ownership in architecture.
