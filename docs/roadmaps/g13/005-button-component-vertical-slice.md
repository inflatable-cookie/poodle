# g13.005 Button Component Vertical Slice

Status: in progress
Owner: Poodle core
Depends on: `g13.004`

## Objective

Prove the low-complexity end-to-end path with Button.

## Deliverables

- Rust-authored Button prop, event, slot, anatomy, accessibility, axis,
  recipe, render, and specimen definitions.
- Generated TypeScript consumed by thin Svelte and React lowering.
- Direct Rust consumption through `poodle-render` for GPUI and Jetstream.
- Contract, registry, interaction, accessibility, recipe, and visual evidence.

## Acceptance

- One definition change is visible in all four previews.
- Public APIs and current pixels remain contract-equivalent.
- No backend reads component machine state from drawing code.
- Hand-written exceptions are zero or explicitly justified in the pilot log.

## Next

`g13.006` tests whether the model survives a multi-thumb stateful control.
