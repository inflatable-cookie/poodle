# 009 Cross-Runtime Component Conformance

Status: rejected as standing architecture — `g14.008`
Updated: 2026-08-15
Owner: Poodle core
Depends on: `001-poodle-system-shape.md`,
`../contracts/001-working-rules.md`

## Pilot Verdict

`g14.008` rejected this conformance-plane design. The pilot measured 22,746
LOC against 472 LOC replaced, omitted HistoryCenter from its comparator, and
exposed 1,205 cross-runtime differences when that omission was corrected.
Manual component registries, incomplete interface consumption, and
backend-named shared probes also violated this decision's boundaries.

Poodle still requires semantic parity and dependable component construction.
This document records the rejected approach; it is not authority for new
components or rollout. `g14.021` removes the pilot machinery while retaining
component fixes and focused regressions. A replacement architecture needs a
fresh decision after cleanup.

## Decision

Poodle keeps two implementation substrates and adds one conformance plane.

```text
component contract + portable interface
                  |
       shared typed cases
      /                   \
Svelte + React       poodle-render
      |                    |
 web observers          GPUI
      \                    /
        normalized observations
                  |
             parity gate
```

- Svelte and React share `poodle-core` behaviour and CSS.
- GPUI consumes shared `poodle-render` composition and `poodle-node` now.
- Jetstream remains a deferred consumer of the same Rust substrate and case
  corpus; its later backend-admission gate must add execution, not a second
  component implementation.
- The conformance plane shares declarations, inputs, actions, assertions, and
  exhaustive diagnostic cases. It does not share executable component
  behaviour or own the catalogue's teaching structure.
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
- stable case identity, diagnostic labels, axes, and capture identity
- semantic actions addressed to stable component parts
- expected state, event order, focus, accessibility, structure, token roles,
  and geometry
- explicit runtime-owned capability requirements and tolerances

The same authored case drives tests in every active runtime. A dedicated
conformance view may project the complete corpus for manual diagnosis. Cases
remain backend-neutral so the deferred Jetstream runner can consume them
unchanged.

### Catalogue specimens

Catalogue specimens are a documentation layer beside conformance, not a view
that must enumerate every case. Their `Examples` tab is a curated route through
the component: primary use, meaningful variants, important states, and useful
composition. Size and density matrices stay in their named tabs. Exhaustive
cases may appear in a separate `Conformance` tab.

One renderer-neutral specimen plan may own tab order, section order, captions,
and references to shared fixture data. It must remain smaller and simpler than
the runtime specimens it coordinates. It cannot become a universal scene tree
or encode component behaviour. Svelte, React, GPUI, and later Jetstream render
real runtime components through thin adapters; bounded runtime-only examples
remain explicit.

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

The active pilot certifies the node vocabulary and GPUI interpretation without
leaking GPUI objects into `poodle-node`. Jetstream admission must later prove
equivalent interpretation of those primitives and intents. A
component-specific workaround cannot stand in for a missing primitive in
either backend.

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

A component is active-cohort complete only when:

1. its contract and portable interface agree;
2. both web implementations and shared Rust composition consume the declared
   interface;
3. required shared cases execute in Svelte, React, and GPUI;
4. normalized observations pass;
5. each active runtime has a useful curated specimen and the complete case
   corpus remains available to its executable harness;
6. required primitive capabilities are certified; and
7. no placeholder, inert handler, missing registration, or declared absence
   remains for required behaviour.

New components enter through this pipeline. A preview registration or passing
unit test alone is not delivery.

Jetstream has a separate admission state. It is neither required for current
active-cohort completion nor allowed to appear as passing until its backend
executes the shared cases, observations, and interactions. Its curated
specimen joins through the separate catalogue workflow.

## Boundaries

- No cross-language behaviour compiler or expression evaluator.
- No universal render tree for web and native.
- No application state, routing, persistence, or DAW-specific models in
  cases.
- Runtime harnesses may translate actions and observe output; they may not
  contain a second fixture definition.
- Every generated or authored conformance surface must state what existing
  declaration or executable fixture content it replaces. Curated catalogue
  specimens are not deletion credit merely because a case corpus exists.

The executable format and pilot stop conditions live in
`../specs/066-executable-component-conformance.md`.
