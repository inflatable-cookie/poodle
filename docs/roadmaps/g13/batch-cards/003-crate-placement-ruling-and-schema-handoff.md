# 003 Crate Placement Ruling And Schema Handoff

Status: planned — orchestrator only
Milestone: `g13.001`
Owner: Poodle maintainer
Depends on: `g13-b001-authority-inventory` and
`g13-b002-pilot-fixture-and-metrics-freeze` merged and reviewed
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-01`–`IR-12`), `docs/architecture/001-poodle-system-shape.md`,
`docs/architecture/002-token-system-and-package-layout.md`

## Goal

Make the judgment calls workers must not make, close `g13.001`, and leave
`g13.002` executable without discovering authority or package boundaries in
code.

## Rulings

- Select the owning crate locations and publication posture for `poodle-ir`
  and `poodle-codegen` from batch `001` evidence.
- Freeze dependency direction between IR, specs/headless, render, node,
  generated TypeScript, and preview/report consumers.
- Rule on each direct Jetstream path and compatibility layer: active runtime
  capability, migration debt, or dead code.
- Approve the fixture manifest and quantitative baseline from batch `002`.
- Amend spec 063 planning notes where evidence narrows or contradicts an
  assumption; do not promote provisional architecture early.

## Exit

- `g13.001` acceptance is evidenced and its status can move to complete.
- `g13.002` has exact package paths, dependency constraints, inputs, outputs,
  validation rules, stop conditions, and its first worker-ready batch card.
- Any unresolved design question pauses `g13.002`; it is not delegated.
