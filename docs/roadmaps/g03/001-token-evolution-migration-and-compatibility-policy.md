# g03.001 Token Evolution, Migration, And Compatibility Policy

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g02.016
Primary repos: `pug`

## Goals

- [x] define how tokens evolve over time
- [x] define migration posture for renamed, split, merged, or deprecated tokens
- [x] define compatibility guarantees for downstream consumers

## Execution Checklist

- [x] define token evolution rules
- [x] define migration posture for breaking or soft-breaking token changes
- [x] define compatibility guarantees for downstream consumers

## Acceptance Criteria

- [x] token evolution policy is explicit
- [x] migration posture is explicit
- [x] compatibility guarantees are explicit

## Outcome

- added `docs/specs/024-token-evolution-migration-and-compatibility-policy.md`
  as the normative baseline for token evolution classes, migration evidence,
  alias/deprecation usage, and downstream compatibility posture
- elevated `packages/tokens/schema/metadata/aliases.json` and
  `packages/tokens/schema/metadata/deprecations.json` from bootstrap wiring to
  canonical migration instruments
- froze the rule that token compatibility is determined by meaning and emitted
  behavior, not by path stability alone
- made pre-`1.0` breaking-change posture explicit: allowed when necessary, but
  never silent and never undocumented
- rolled the active roadmap/spec surfaces forward to `g03.002`

## Next Task

Open `g03.002` and harden parity automation plus regression evidence across the
shared surface.
