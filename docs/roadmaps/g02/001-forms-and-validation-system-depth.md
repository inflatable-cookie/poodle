# g02.001 Forms And Validation System Depth

Status: planned
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.014
Primary repos: `pug`

## Context

With the primitive layer in place, Pug needs a cohesive form system rather than
isolated inputs.

## Goals

- [ ] define field wrappers, labels, help text, error text, validation states,
  and action rows
- [ ] define consistent form composition rules across Svelte and GPUI
- [ ] define async and pending validation posture where relevant

## Execution Checklist

- [ ] define the first form-shell components in scope
- [ ] define validation-state and error-presentation rules
- [ ] define form composition and action-row patterns
- [ ] define async or pending validation behavior where needed

## Acceptance Criteria

- [ ] form system surface is explicit
- [ ] validation semantics are explicit

## Deliverables

- [ ] form-system baseline
- [ ] validation contract notes

## Next Task

Open `g02.002` and define the data-table and bulk-action suite.
