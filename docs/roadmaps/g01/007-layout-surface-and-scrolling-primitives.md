# g01.007 Layout, Surface, And Scrolling Primitives

Status: completed
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.003, g01.004, g01.005, g01.006
Primary repos: `pug`

## Context

The primitive layer should start with the structural components every other
component depends on.

## Goals

- [ ] define layout primitives such as box, stack, inline, grid, and spacer
- [ ] define surface and separator primitives
- [ ] define scrolling-shell expectations
- [ ] define token usage and layout invariants for these primitives

## Non-Goals

- [ ] no product composites yet
- [ ] no workstation-specific panels yet

## Execution Checklist

- [ ] list the layout primitives included in the first tranche
- [ ] define their required props, layout rules, and composition boundaries
- [ ] define surface, separator, and scroll-shell behavior
- [ ] tie each primitive back to the token system
- [ ] record any expected Svelte versus GPUI deltas

## Acceptance Criteria

- [ ] the layout primitive family is explicitly bounded
- [ ] surface semantics are explicit
- [ ] scrolling behavior expectations are explicit in both frameworks

## Deliverables

- [ ] first layout primitive catalogue
- [ ] surface and scroll-shell contract notes

## Next Task

Open `g01.008` and define the action and text-entry primitives.
