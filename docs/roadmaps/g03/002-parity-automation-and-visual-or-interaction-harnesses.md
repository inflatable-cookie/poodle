# g03.002 Parity Automation And Visual Or Interaction Harnesses

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g02.016
Primary repos: `flint`

## Goals

- [x] define automated parity evidence where useful
- [x] define visual, interaction, and contract-level harness posture
- [x] decide what should remain manual review versus automated proof

## Execution Checklist

- [x] define parity checks suitable for automation
- [x] define visual versus interaction versus contract-level harness posture
- [x] define what remains manual review

## Acceptance Criteria

- [x] parity automation posture is explicit
- [x] manual versus automated boundaries are explicit

## Outcome

- added `docs/specs/025-parity-automation-and-harness-boundary.md` as the
  normative baseline for contract, visual, and interaction harness posture plus
  the manual-review boundary
- added a machine-readable parity target registry at
  `packages/svelte/preview/src/parity.ts` built against the live docs/catalog
  surface rather than a second disconnected checklist
- added a generated evidence artifact at
  `packages/svelte/preview/artifacts/parity-report.json` produced by
  `packages/svelte/preview/scripts/build-parity-report.ts`
- added root-level parity commands so the report can be regenerated and checked
  together with the docs surface
- made the Svelte preview URL-addressable by section, theme, density, and
  control size so review notes can point at stable evidence surfaces

## Validation

- `bun run parity:report`
- `bun run docs:build`
- `git diff --check`

## Next Task

Open `g03.003` and harden contract linting plus docs completeness checks.
