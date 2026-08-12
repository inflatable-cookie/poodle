# g13.003 Deterministic Codegen And Drift Gate

Status: complete (`b022` machinery + `b025` targets; merges `9dab52ac`, `945f55d2`)
Owner: Poodle core
Depends on: `g13.002`

## Objective

Build the compiler boundary before any component migration.

## Deliverables

- Deterministic TypeScript, JSON/schema, registry, conformance-vector, and docs
  fragment emitters.
- Generated headers with source path, IR version, and generator version.
- Effigy `ir:build` and read-only `ir:check` selectors.
- Isolated regeneration that cannot dirty unrelated artifacts.
- Snapshot and malformed-input tests for every emitter.

## Acceptance

- Two clean generations are byte-identical.
- `ir:check` fails on drift and leaves the worktree unchanged.
- Generated TypeScript type-checks without framework dependencies.
- One Rust fixture change updates every declared artifact in one command.

## Next

`g13.004` proves scene composition on the shared preview shell.
