# g03.005 Theming, Branding, And Downstream Override Strategy

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g03.001
Primary repos: `flint`

## Goals

- [x] define brand overrides and theming extension posture
- [x] define how downstream apps can customize without redefining token meaning
- [x] define safe override boundaries

## Execution Checklist

- [x] define downstream theming and branding override patterns
- [x] define safe override boundaries
- [x] define what remains canonical token meaning versus downstream theming

## Acceptance Criteria

- [x] downstream override strategy is explicit
- [x] token-meaning preservation is explicit

## Outcome

- added `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md`
  to freeze the extension model: pure semantic tokens, recipe variables and
  treatment roles, then app-owned wrappers or composites
- made gradients and similar browser-only styling effects explicit web-layer
  treatments rather than canonical token values
- established seeded treatment roles for `interactive`, `interactive-primary`,
  and `interactive-subtle`
- implemented a scoped Svelte proof across input chrome, tabs, and preview
  action/button surfaces without redefining token meaning
- kept the next active roadmap task on `g03.003`, since docs linting and
  completeness are still the next uncompleted hardening tranche

## Validation

- `bun run tokens:build`
- `bun run docs:build`
- `git diff --check`

## Next Task

Return to `g03.003` and harden contract linting plus docs completeness checks.
