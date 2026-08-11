# g13.009 Shared Specimen Scene Migration

Status: gated on g13.008 adopt verdict
Owner: Poodle core
Depends on: `g13.008`

## Objective

Move preview composition from four hand-maintained specimen sets to shared
scene definitions without changing component implementations yet.

## Scope

- Migrate shared examples, sizes, densities, orientations, states, and usage
  fixtures by family.
- Preserve runtime-only demonstrations as declared scene extensions.
- Generate registries, navigation metadata, capture IDs, and coverage reports.
- Delete old specimen definitions only after all four runtimes consume the
  replacement.

## Acceptance

- Every registered component has equivalent four-runtime scene coverage.
- Theme and both size/density axes remain interactive.
- Coverage measures rendered scenes, not registry presence alone.

## Next

Component family migrations `g13.010`–`013` use these shared fixtures.
