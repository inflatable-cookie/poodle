# g01.004 Component Contract Template And Documentation IA

Status: completed
Owner: Flint Core
Updated: 2026-03-11
Depends on: g01.001, g01.002
Primary repos: `flint`

## Research Inputs

- [tm-contract-template](../../research/translation-memos/tm-contract-template.md)
- [tk-cross-framework-contracts](../../research/value-tracks/tk-cross-framework-contracts.md)

## Context

Flint's distinguishing promise is docs-first shared contracts. That means the
documentation system for components is not incidental; it is part of the
product.

## Problem

Without a standard contract template and docs information architecture, each
component will drift in what it documents, what parity means, and how Svelte
and GPUI differences are justified.

## Goals

- [ ] define the canonical per-component contract template
- [ ] define the parity model shared by Svelte and GPUI implementations
- [ ] define how tokens, states, events, layout, and accessibility are
  documented
- [ ] define how Svelte and GPUI implementation notes are attached
- [ ] define how examples, known deltas, and future adoption notes are
  organized

## Non-Goals

- [ ] no full component catalogue yet
- [ ] no docs site implementation yet

## Execution Checklist

- [ ] freeze the minimum required sections every component contract must carry
- [ ] define the anatomy documentation format for component parts and token
  targets
- [ ] define the prop naming, boolean-state, event-handler, and variant naming
  rules
- [ ] define the three-tier parity model for strict semantic parity, visual
  parity, and implementation freedom
- [ ] define the accessibility section requirements including ARIA, keyboard,
  focus, and announcement behavior
- [ ] define when a state table is enough versus when a state machine diagram is
  required
- [ ] define the folder and navigation model for component docs
- [ ] define grouping rules for foundation, composite, and workstation-shell
  components
- [ ] define how intentional framework deltas and known gaps are recorded
- [ ] define how token usage is attached to component anatomy rather than
  hand-wavy visual description
- [ ] define the contract approval workflow before implementation begins
- [ ] sketch one primitive and one composite against the proposed template

## Acceptance Criteria

- [ ] a single contract template exists
- [ ] the parity model is explicit enough to review Svelte and GPUI against the
  same checklist
- [ ] documentation IA is explicit
- [ ] the template is sufficient for primitives and composites alike

## Deliverables

- [ ] component contract template
- [ ] docs IA and navigation model
- [ ] parity and delta-recording policy

## Evidence Requirements

- [ ] one example primitive contract sketched against the template
- [ ] one example composite contract sketched against the template

## Next Task

Open `g01.005` and define the Svelte-side implementation substrate and Bits
integration policy.
