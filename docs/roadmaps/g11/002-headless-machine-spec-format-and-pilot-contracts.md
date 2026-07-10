# g11.002 Headless Machine-Spec Format And Pilot Contracts

Status: complete (2026-07-10)
Owner: Poodle core
Depends on: `docs/specs/062-headless-core-and-dual-layer-strategy.md`
Updated: 2026-07-10

## Purpose

Design the machine-spec format that will describe every component's headless
behavior, extend the component-contract template to carry it, and write the
first three machine specs as proof — all before any implementation code.

This milestone is docs-only. It de-risks everything after it: if the format
cannot cleanly express a hard component, better to learn that here than after
a package exists.

## Deliverables

1. Machine-spec section added to `docs/contracts/template/` covering:
   - states and initial state
   - events (user, programmatic, internal/timer)
   - transitions with guards
   - effects (focus moves, scroll, timers) named, not implied
   - anatomy parts with per-part attribute output: role, ARIA,
     `data-scope`/`data-part`/`data-state`
   - controlled/uncontrolled seams (which values can be externally owned)
2. A recommendation in spec `062` (Open Questions) for the authoring format:
   TS-first vs declarative data. Include a sketch of how the same spec drives
   a Rust port or codegen.
3. Three pilot machine specs written into the existing component contracts:
   - `tabs` — selection + roving tabindex, moderate difficulty
   - `popover` — overlay stack, positioning, dismissal, focus return; the
     hard case that exercises shared machinery
   - `checkbox` — trivial case proving the format has no minimum-complexity
     floor
4. Shared-machinery inventory: enumerate the cross-component services the
   pilots revealed (focus trap, roving tabindex, dismissable-layer stack,
   anchor positioning, presence, typeahead, id wiring) as the build list for
   `g11.003`.

## Method

- Start from the existing contracts' States/Accessibility/Keyboard sections —
  they are most of a machine spec already; formalize, do not reinvent.
- Cross-check each pilot spec against the current Svelte implementation
  (parity authority). Where the implementation and old contract disagree, the
  implementation wins and the contract is corrected in the same change.
- Sanity-check part/attribute naming against Zag.js conventions so we do not
  invent a gratuitously different vocabulary.

## Exit Criteria

- template extension merged; three pilot contracts carry complete machine
  specs
- shared-machinery inventory recorded (here or in spec `062`)
- authoring-format recommendation recorded in spec `062`
- no code changes in this milestone

## Validation

- `effigy docs:lint`
- contract review against the live Svelte components for behavioral fidelity

## Completion Notes (2026-07-10)

- `Behavior Machine` section added to
  `docs/contracts/template/component-contract-template.md` (under section 4):
  classification line, Context, States, Events, Transitions, Effects, Part
  Attribute Output, Machinery Dependencies.
- Pilot machine specs written against the live Svelte implementations:
  - `checkbox.md` — trivial case; single state, value in context; readOnly
    revert and mixed-resolves-to-checked captured as guards/effects
  - `popover.md` — hard case; closed/open, dismissable-layer + focus-restore
    effects; CSS-only anchoring recorded as documented delta vs the planned
    Floating UI service
  - `tabs.md` — moderate case; roving tabindex + selection, with drag/tooltip
    as sub-machines and URL-history/overflow as environment effects
- Format learnings + authoring-format recommendation (contract-markdown is
  the spec, TS is the implementation, codegen deferred to `g11.006`) recorded
  in spec `062`, along with the shared-machinery inventory for `g11.003`.
- Convention adopted: `data-scope`/`data-part`/`data-state` emission is
  additive during the core swap; existing attributes unchanged.

## Next Task

`g11.003` — build the core package, shared machinery, and pilot machines, and
swap the three pilot Svelte components onto them interface-invariantly.
