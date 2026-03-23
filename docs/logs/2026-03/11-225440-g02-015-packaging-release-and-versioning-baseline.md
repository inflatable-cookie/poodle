# g02.015 Packaging Release And Versioning Baseline

Status: completed
Date: 2026-03-11
Owner: Poodle Core

## Summary

- completed `g02.015`
- added release-intent metadata to the package manifests so public-intent
  packages, internal packages, and tooling packages stop looking equivalent
- added repo-level classification in `packages/release-manifest.json`
  so package kind, release channel, and public-intent posture are explicit in
  one place
- added the normative packaging and release baseline at
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`
- kept the whole system pre-1.0 and explicitly split `preview` versus
  `internal` channels rather than pretending stable release semantics already
  exist

## Validation

- `git diff --check`

## Notes

- this tranche intentionally freezes policy and package metadata, not a publish
  pipeline
- downstream adoption remains deferred even with clearer release posture; the
  next step is generation closeout and a deliberate handoff into `g03`

## Next Task

Open `docs/roadmaps/g02/016-generation-closeout-and-g03-cutover-plan.md` and
summarize what is stable enough to carry into the first downstream-adoption
generation.
