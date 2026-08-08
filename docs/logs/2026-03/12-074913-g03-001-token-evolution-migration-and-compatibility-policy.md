# g03.001 Token Evolution, Migration, And Compatibility Policy

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- completed `g03.001`
- added `docs/specs/024-token-evolution-migration-and-compatibility-policy.md`
  to freeze token evolution classes, migration evidence requirements, and
  downstream compatibility posture
- made token compatibility meaning-based rather than path-based, so unchanged
  names do not imply unchanged semantics
- elevated alias and deprecation metadata under
  `packages/tokens/schema/metadata/` into canonical migration instruments
- made pre-`1.0` breaking-token posture explicit: permitted when necessary, but
  never silent and never undocumented
- rolled active roadmap and spec surfaces forward to `g03.002`

## Validation

- `git diff --check`

## Notes

- this tranche intentionally froze policy before building parity automation so
  later regression evidence can classify token changes consistently
- downstream adoption remains sequenced after more hardening work, not as an
  immediate follow-on from this policy batch

## Next Task

Open `docs/roadmaps/g03/002-parity-automation-and-visual-or-interaction-harnesses.md`
and harden parity automation plus regression evidence across the shared
surface.
