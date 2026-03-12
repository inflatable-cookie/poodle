# g03.003 Contract Linting, Docs Completeness, And Publish Pipeline

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g02.016
Primary repos: `pug`

## Goals

- [x] define how docs completeness is enforced
- [x] define contract linting or validation posture
- [x] define documentation publishing pipeline expectations

## Execution Checklist

- [x] define docs completeness rules
- [x] define contract linting or validation posture
- [x] define documentation publishing pipeline expectations

## Acceptance Criteria

- [x] docs completeness posture is explicit
- [x] contract linting posture is explicit
- [x] publishing pipeline posture is explicit

## Outcome

- added `docs/specs/027-docs-completeness-contract-linting-and-publish-pipeline.md`
  as the normative baseline for docs completeness, contract linting, and the
  internal publish-candidate posture
- added `packages/svelte/preview/scripts/lint-docs.ts` plus root and preview
  Bun scripts so docs coverage, contract structure, and parity/catalog wiring
  can be validated from repo state instead of by convention alone
- promoted `bun run docs:lint` and `bun run docs:check` to the expected docs
  hardening entry points alongside the existing parity report and preview build
- froze the current publish posture as an internal static preview build rooted
  in `packages/svelte/preview/dist`, without pretending a broader public docs
  site already exists

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Next Task

Open `g03.004` and harden performance and cost characteristics of the shared
surface.
