# g13.002 Rust IR Schema And Validation Core

Status: planned
Owner: Poodle core
Depends on: `g13.001`

## Objective

Create the versioned Rust model for component and scene definitions without
adding renderer or framework behavior.

## Deliverables

- Serializable IDs, types, props, events, slots, parts, semantic nodes,
  expressions, axes, recipes, accessibility intent, capabilities, and scenes.
- Graph validation for duplicate IDs, invalid references, impossible prop
  bindings, unsupported cycles, missing accessibility data, and undeclared
  capabilities.
- Stable ordering, schema versioning, round-trip fixtures, and useful source
  diagnostics.
- Ordinary Rust constructors first; macros only after measured authoring pain.

## Acceptance

- Invalid definitions fail at their authored source with focused diagnostics.
- JSON round trips preserve meaning and ordering.
- The schema contains no framework, DOM, GPUI, Jetstream, or `poodle-node`
  implementation types.

## Next

`g13.003` turns validated definitions into deterministic target artifacts.
