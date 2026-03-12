# g02.001 Forms And Validation System Depth

Status: completed
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.014
Primary repos: `pug`

## Context

With the primitive layer in place, Pug needs a cohesive form system rather than
isolated inputs.

## Goals

- [x] define field wrappers, labels, help text, error text, validation states,
  and action rows
- [x] define consistent form composition rules across Svelte and GPUI
- [x] define async and pending validation posture where relevant

## Execution Checklist

- [x] define the first form-shell components in scope
- [x] define validation-state and error-presentation rules
- [x] define form composition and action-row patterns
- [x] define async or pending validation behavior where needed

## Acceptance Criteria

- [x] form system surface is explicit
- [x] validation semantics are explicit

## Deliverables

- [x] form-system baseline
- [x] validation contract notes

## Next Task

Open `g02.002` and define the data-table and bulk-action suite.
