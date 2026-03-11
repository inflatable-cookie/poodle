# g01.013 Underlay Bridge And Token-Ingestion Baseline

Status: planned
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.003, g01.011, g01.012
Primary repos: `pug`, `underlay`

## Context

Underlay is one of the main prospective Svelte-side consumers of Pug, but it
must remain the public surface for Underlay apps.

## Goals

- [ ] define how Underlay ingests Pug tokens
- [ ] define how Underlay wraps or reuses Pug components without exposing Pug to
  app code
- [ ] define namespace, mapping, and ownership rules for token bridging
- [ ] define what stays Underlay-owned versus what can be shared directly

## Execution Checklist

- [ ] define the token-ingestion model for Underlay
- [ ] define wrapper-preservation rules for Underlay components
- [ ] define ownership boundaries between Pug internals and Underlay public APIs
- [ ] document expected migration pressure points before adoption begins

## Acceptance Criteria

- [ ] Underlay token-ingestion posture is explicit
- [ ] wrapper-preservation rule is explicit
- [ ] zero-leak goal for Underlay apps is explicit

## Deliverables

- [ ] Underlay bridge baseline note
- [ ] token-ingestion and wrapper rules

## Next Task

Open `g01.014` and close the generation with parity evidence rules plus the
downstream extension contract.
