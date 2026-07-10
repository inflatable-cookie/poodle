# g11.007 Multi-Framework Adapters And Mitosis Decision

Status: planned
Owner: Poodle core
Depends on: `g11.004` (core sweep substantially complete), `g11.005` (recipe
layer exists, so styled shells have a defined shape)
Updated: 2026-07-10

## Purpose

Prove the "write once" claim: ship a second web framework on the same core,
then make the styled-shell strategy decision (hand-written per framework vs
Mitosis-compiled) from evidence instead of promise.

## Scope

1. **React adapter pilot**: `@poodle/react` with the machine-subscription
   hook + styled shells for a bounded component set (suggest: Button, Tabs,
   Dialog, Select, Checkbox — one from each difficulty class). Recipes and
   tokens shared; only the shell templates are per-framework.
2. **Measure the actual cost**: record adapter LOC, per-component shell
   effort, and defect classes. This number is the input to the Mitosis
   decision — if shells are cheap, a compiler buys little.
3. **Mitosis spike (bounded, one batch)**: author 2–3 styled shells in
   Mitosis, compile to Svelte + React, compare against the hand-written
   versions for output quality, debuggability, recipe/token integration, and
   Svelte 5 runes fidelity. Spike scope is shells only — the core is never a
   Mitosis candidate (decision recorded in spec `062`).
4. **Decision record**: hand-written adapters vs Mitosis shells vs "not now",
   written into spec `062` and promoted to architecture. Include Vue posture
   (do when a real consumer needs it, or proactively).

## Compatibility

- Svelte consumers unaffected; new packages are additive.
- The React package enters under the same contract discipline: contracts are
  the source of truth, machine specs shared, recipes shared, parity evidence
  per component per spec `008` lineage.

## Exit Criteria

- React pilot set passing the same contract state coverage as Svelte
  equivalents (preview or storybook-equivalent surface counts as the demo
  bar)
- cost data recorded
- Mitosis verdict recorded in spec `062` with evidence links
- spec `062` promotion complete: durable outcomes moved to
  `docs/architecture/` and contracts; spec archived or retired per
  shape-with-specs-and-promote rules

## Validation

- React pilot typechecks + unit/interaction tests
- conformance vectors (from `g11.006`) hold for machine behavior under the
  React adapter where applicable
- `effigy docs:lint`

## Next Task

Program closeout review: reconcile spec `062` open questions, promote, then
assess whether g11 rolls over.
