# g11.007 Multi-Framework Adapters And Mitosis Decision

Status: complete (2026-07-10)
Owner: Poodle core
Depends on: `g11.004` (core sweep substantially complete), `g11.005` (recipe
layer exists, so styled shells have a defined shape)
Updated: 2026-07-10

## Purpose

Prove the "write once" claim: ship a second web framework on the same core,
then make the styled-shell strategy decision (hand-written per framework vs
Mitosis-compiled) from evidence instead of promise.

## Scope

1. **React adapter pilot**: `@inflatable-cookie/poodle-react` with the machine-subscription
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

## Completion Notes (2026-07-10)

**React pilot shipped** (`packages/react` → `@inflatable-cookie/poodle-react` +
`@inflatable-cookie/poodle-react-preview`): Button (styled-only class), Checkbox (simple
machine), Tabs (moderate machine) — one per difficulty class. Tokens ship
as plain CSS so they imported unchanged; component CSS was extracted
mechanically from the Svelte shells by script; machines imported directly
from `@inflatable-cookie/poodle-headless`.

Measured cost:

- TSX shells: 268 lines total (Button 41, Checkbox 90, Tabs 134) — the
  adapter layer really is glue
- CSS: mechanical extraction (Button 347 / Checkbox 124 / Tabs 553 lines,
  scripted, zero hand-editing)
- Zero changes to core, tokens, or the Svelte layer
- Runtime-verified in the React preview: token-styled variants, controlled
  checkbox + readOnly revert through the machine, tabs arrow navigation
  skipping a disabled item with correct roving tabindex and Home behavior —
  the same `tabsTransition` that drives Svelte and the Rust mirror

**Mitosis verdict: rejected for styled shells** (core rejection was already
recorded in spec 062). Spike evidence (two shells authored in `.lite.tsx`,
compiled to both targets):

- compilation works and `@inflatable-cookie/poodle-headless` imports pass through cleanly —
  the machine layer composes with Mitosis in principle
- disqualifying: the Svelte target emits legacy Svelte 4 (`export let`,
  `on:` events, no runes) — it would reintroduce the exact public seams
  g11.001 retired, so Svelte shells must stay hand-written regardless,
  which removes the write-once benefit Mitosis was meant to buy
- the React target emits untyped `.jsx` (interfaces dropped)
- the `.lite` dialect fought the controlled/uncontrolled pattern (a
  `defaultChecked` prop silently dropped out of the compiled output)
- against a measured ~90 LOC/component hand-shell cost, a compiler layer
  plus its debugging surface buys nothing

Vue posture: hand-written adapter when a real consumer needs it; the React
pilot is the template (hook glue + scripted CSS extraction).

## Next Task

Runway complete (g11.002–007). Program closeout: spec 062 marked promoted;
g11 rollover is a separate explicit decision.
