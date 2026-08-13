# 009 Cross-Runtime Component Conformance

Status: active
Updated: 2026-08-14
Owner: Poodle core
Depends on: `001-poodle-system-shape.md`,
`../contracts/001-working-rules.md`

## Decision

Poodle keeps two implementation substrates and adds one conformance plane.

```text
component contract + portable interface
                  |
       shared typed cases
      /                   \
Svelte + React       poodle-render
      |                /       \
 web observers     GPUI       Jetstream
      \                |       /
        normalized observations
                  |
             parity gate
```

- Svelte and React share `poodle-core` behaviour and CSS.
- GPUI and Jetstream share `poodle-render` composition and `poodle-node`.
- The conformance plane shares declarations, inputs, actions, assertions, and
  specimen structure. It does not share executable component behaviour.
- Svelte remains the reference when current implementations disagree. The
  component contract remains the authority for what should ship.

## No Silent Portable Difference

Every portable difference must do one of two things:

1. fail a standing conformance gate; or
2. be classified as runtime-owned mechanism with an equivalent observable
   result.

A declared missing capability is visible debt, not an equivalent result. It
cannot satisfy the component completion gate.

## Conformance Layers

### Portable interface

One constrained declaration records portable props and defaults, controlled
state pairs, semantic events and payloads, named composition regions, public
methods, states, and capability requirements.

The declaration may emit idiomatic TypeScript and Rust types. It does not
model DOM attributes, framework lifecycle, backend objects, arbitrary
callbacks, or behaviour. Generated declarations must replace hand-written
portable declarations; additive mirrors fail the cost gate.

### Shared cases

A case carries:

- fixture data and portable inputs
- specimen group, caption, axes, and capture identity
- semantic actions addressed to stable component parts
- expected state, event order, focus, accessibility, structure, token roles,
  and geometry
- explicit runtime-owned capability requirements and tolerances

The same authored case drives tests and specimen pages in all four runtimes.
Specimens are views over cases, not a second fixture corpus.

### Normalized observations

Each runtime projects real output into a versioned semantic observation:

- component and part identity
- role, accessible name, state, and focusability
- text and icon identity
- semantic token roles and resolved visual channels
- parent/child and layout relationships
- geometry, clipping/layer order, and supported interactions
- emitted semantic events in order

Web observers read DOM, computed style, and accessibility state. Native
observers read `poodle-node` plus backend-observed output. Raw DOM trees and
engine nodes need not match. Shared capture IDs add image evidence where
properties cannot expose rasterization, clipping, or layer composition.

Semantic identity, event order, state, token role, resolved non-raster visual
values, and part relationships are exact. Geometry uses explicit bounded
tolerances. Font rasterization and engine drawing may use named visual or
regional image tolerances; blanket runtime tolerances are forbidden.

### Certified primitive substrate

Component parity depends on a small renderer vocabulary. Poodle certifies:

- layout, sizing, clipping, and scroll
- surface, border, radius, shadow, text, and icon
- control state and semantic token projection
- focus, keyboard, pointer, dismissal, and overlay placement
- accessibility projection
- text editing and IME boundaries

GPUI and Jetstream must prove that they interpret shared `poodle-node`
primitives and intents equivalently. A component-specific workaround cannot
stand in for a missing primitive.

## Component Construction Profiles

Every component uses one build recipe and declares one profile:

| Profile | Adds |
| --- | --- |
| `display` | structure, tokens, layout, accessibility |
| `control` | controlled state, semantic events, focus, keyboard/pointer |
| `collection` | item identity, selection, navigation, repeated anatomy |
| `overlay` | portals/layers, placement, dismissal, focus transfer |
| `input` | editing, validation, selection, clipboard/IME capability |
| `composite` | nested components, host commands, multi-step scenarios |

Profiles accumulate requirements; they do not select different component
architectures. `Button` uses the same pipeline as `HistoryCenter`.
HistoryCenter adds composite fixtures and host-command adapters rather than a
bespoke parity method.

## Completion

A component is complete only when:

1. its contract and portable interface agree;
2. both implementation pairs consume the declared interface;
3. required shared cases execute in Svelte, React, GPUI, and Jetstream;
4. normalized observations pass;
5. the same cases render the four specimen views;
6. required primitive capabilities are certified; and
7. no placeholder, inert handler, missing registration, or declared absence
   remains for required behaviour.

New components enter through this pipeline. A preview registration or passing
unit test alone is not delivery.

## Boundaries

- No cross-language behaviour compiler or expression evaluator.
- No universal render tree for web and native.
- No application state, routing, persistence, or DAW-specific models in
  cases.
- Runtime harnesses may translate actions and observe output; they may not
  contain a second fixture definition.
- Every generated or authored conformance surface must state what existing
  declaration or specimen content it replaces.

The executable format and pilot stop conditions live in
`../specs/066-executable-component-conformance.md`.
