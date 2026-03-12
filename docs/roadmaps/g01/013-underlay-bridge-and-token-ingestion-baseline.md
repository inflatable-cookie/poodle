# g01.013 Underlay Bridge And Token-Ingestion Baseline

Status: completed
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.003, g01.011, g01.012
Primary repos: `pug`, `underlay`

## Context

Underlay is one of the main prospective Svelte-side consumers of Pug, but it
must remain the public surface for Underlay apps.

## Goals

- [x] define how Underlay ingests Pug tokens
- [x] define how Underlay wraps or reuses Pug components without exposing Pug to
  app code
- [x] define namespace, mapping, and ownership rules for token bridging
- [x] define what stays Underlay-owned versus what can be shared directly

## Execution Checklist

- [x] define the token-ingestion model for Underlay
- [x] define wrapper-preservation rules for Underlay components
- [x] define ownership boundaries between Pug internals and Underlay public APIs
- [x] document expected migration pressure points before adoption begins

## Acceptance Criteria

- [x] Underlay token-ingestion posture is explicit
- [x] wrapper-preservation rule is explicit
- [x] zero-leak goal for Underlay apps is explicit

## Deliverables

- [x] Underlay bridge baseline note
- [x] token-ingestion and wrapper rules

## Next Task

`g01.013` is complete. Continue with `g01.014` only when closing the generation,
or move into `g02.001` once `g01` is fully complete.
