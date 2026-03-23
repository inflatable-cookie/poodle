# g01.005 Svelte Substrate And Bits Integration Policy

Status: completed
Owner: Poodle Core
Updated: 2026-03-11
Depends on: g01.002, g01.004
Primary repos: `poodle`

## Research Inputs

- [tm-svelte-substrate](../../research/translation-memos/tm-svelte-substrate.md)
- [hub-bits](../../research/source-hubs/hub-bits.md)

## Context

Poodle's Svelte implementation needs an explicit relationship to Bits and to any
other lower-level browser/UI dependencies.

## Problem

If the Svelte side treats Bits as the public contract, Poodle loses ownership of
its own semantics. If it rejects Bits entirely, it may waste effort rebuilding
stable primitives with no benefit.

## Goals

- [ ] define Bits as an internal substrate rather than a public contract source
- [ ] define what Bits may be used for
- [ ] define what Poodle must still own regardless of Bits usage
- [ ] define the expected shape of the Svelte implementation packages
- [ ] define how token artifacts are consumed on the Svelte side
- [ ] ensure the Svelte substrate does not become the canonical shape for all
  future web implementations

## Non-Goals

- [ ] no final package layout implementation yet
- [ ] no Underlay bridge work yet

## Execution Checklist

- [ ] freeze the wrapper-component posture for Bits-backed primitives
- [ ] define allowed and disallowed dependency patterns around Bits
- [ ] define how Bits-backed components remain contract-owned by Poodle
- [ ] define which concerns Bits is allowed to own: accessibility, focus,
  keyboard, state, overlay plumbing
- [ ] define which concerns Poodle must own: public props, variants, tokens, docs,
  parity rules
- [ ] define how Svelte packages expose primitives and composites
- [ ] define how theme, density, and state tokens reach the Svelte runtime
- [ ] define how `class` and `data-*` driven token styling should work in the
  wrapper layer
- [ ] define when Poodle should expose compound components versus simplified
  facades
- [ ] define which browser/runtime concerns are implementation details versus
  public contract
- [ ] define the no-leak rule so app code should not need Bits imports or Bits
  types to consume Poodle
- [ ] document which Svelte-specific conveniences must remain outside the
  canonical contract so React or other web targets stay possible

## Acceptance Criteria

- [ ] Bits policy is explicit
- [ ] Svelte package boundary model is explicit
- [ ] public contract ownership remains with Poodle
- [ ] no Bits surface leaks into the intended public API
- [ ] Svelte-specific implementation choices are prevented from becoming
  canonical web-contract requirements

## Deliverables

- [ ] Svelte substrate policy
- [ ] Bits integration rules

## Evidence Requirements

- [ ] one example showing a Bits-assisted implementation that still presents a
  Poodle-defined contract
- [ ] one example showing token/state styling applied through the wrapper layer

## Next Task

Open `g01.006` and define the GPUI-side substrate and Rust token-binding
baseline.
