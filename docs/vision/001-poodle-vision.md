# 001 Poodle Vision

Status: active
Updated: 2026-09-04

## Purpose

Poodle is a generalized design system for teams that need the same UI language
across Svelte, React, GPUI, and Jetstream applications.

It turns component meaning, design tokens, themes, interaction rules, and
accessibility expectations into explicit contracts. Each renderer stays
idiomatic while operators receive consistent behavior and presentation.

## Core Promise

Every Poodle component should:

- have one renderer-neutral contract
- use one generated semantic token vocabulary
- preserve the same observable inputs, states, events, and behavior across
  supported runtimes
- state intentional runtime differences
- remain reusable outside the application that first needed it

Poodle does not pursue byte-for-byte or implementation-level sameness. Parity
means semantic behavior, accessible operation, layout intent, and token use
before rendering mechanics.

## What Poodle Owns

- W3C DTCG design-token sources and CSS, TypeScript, and Rust artifacts
- portable theme, density, control-size, and appearance-Recipe rules
- framework-neutral component contracts
- foundational controls and layout primitives
- reusable application composites
- general workstation and professional-tool shells
- Svelte and React web implementations
- a shared Rust renderer with GPUI and Jetstream backends
- documentation, parity rules, examples, and migration guidance

## What Poodle Does Not Own

- product-specific workflows, routing, persistence, or service calls
- Loophole-specific DAW surfaces such as transport, timeline, mixer,
  automation, or plugin panels
- Underlay's application-facing APIs and templates
- Bits Svelte APIs as public Poodle contracts
- renderer-only shortcuts that would silently change component semantics

Applications should build domain surfaces from Poodle components rather than
moving domain vocabulary into the design system.

## Ecosystem Boundaries

### Underlay

Underlay and the applications built on it import Poodle's published packages
directly. Underlay may add its own templates or bridges on top, and it owns
them; Poodle does not carry an Underlay-facing adapter layer. Operator decision
2026-09-02, recorded in architecture 001.

### Loophole and other products

Product-specific components live with the product. They may compose Poodle
tokens and components, but Poodle accepts them only when their semantics are
general across products.

### Web renderers

Svelte and React share framework-free behavior, styles, tokens, and contracts.
Bits Svelte may support the Svelte implementation internally, but it does not
define public naming or behavior.

### Native renderers

GPUI and Jetstream share `poodle-render`, which produces renderer-neutral node
trees. Backends interpret those nodes using native runtime capabilities.
Renderer-specific input, lifecycle, and drawing remain in the backend.

## Success Criteria

Poodle succeeds when:

- an operator can choose a runtime and begin without learning repository
  history
- one token change generates aligned web and native artifacts
- one contract can be verified across every supported renderer
- a new renderer can reuse contracts and tokens without redefining them
- applications keep their public APIs and domain logic outside Poodle
- runtime differences are deliberate, bounded, and visible
- packages can evolve through pre-1.0 preview into a documented public release

## Failure Modes

- becoming a component dump for one application
- treating a preview gallery as proof of behavioral parity
- duplicating native component recipes in each backend
- duplicating web state or CSS in each framework
- forcing downstream teams to translate themes by hand
- exposing implementation dependencies as Poodle's public API
- hiding release status or unsupported behavior from operators
