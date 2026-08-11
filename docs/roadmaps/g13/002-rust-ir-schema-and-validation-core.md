# g13.002 Rust IR Schema And Validation Core

Status: in progress (first card `011` dispatched 2026-08-11)
Owner: Poodle core
Depends on: `g13.001`

## Objective

Create the versioned Rust model for component and scene definitions without
adding renderer or framework behavior.

## Fixed Inputs (from `g13-b003`)

- Crate: `packages/contracts/ir/`, name `poodle-ir`, **lib only**.
- Dependencies: `poodle-tokens` only. Nothing may depend on `poodle-ir` yet.
- Requirements source: `docs/roadmaps/g13/pilot-expressiveness-corpus.md`
  (129 requirements).
- Before-state: `docs/roadmaps/g13/pilot-baseline-manifest.md`.

Two requirements discovered after spec 063 was written (`g13-b003` R6):

1. **Shared types are first-class.** `ButtonTone` and `OverlayPlacement` each
   fragmented across three contracts, and 8 enumerated shared types have no
   definition anywhere in `docs/`.
2. **Per-component permitted subsets** of a shared type must be expressible and
   must reach the generated artifacts.

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
