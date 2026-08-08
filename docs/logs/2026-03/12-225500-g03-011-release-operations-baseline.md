---
title: g03.011 release operations baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, release, operations]
---

## Summary

Completed `g03.011` by freezing explicit change classes, deprecation workflow,
and release-channel operations for the current Pug package set, then wiring the
repo lint surface to keep those records aligned with package metadata.

## What changed

- added the normative spec `docs/specs/044-deprecation-change-control-and-release-channel-operations.md`
- completed `docs/roadmaps/g03/011-deprecation-change-control-and-release-channel-operations.md`
- added the machine-readable operations record `packages/release-operations.json`
- updated `packages/release-manifest.json` so release classification and operations records stay linked
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate:
  - required change classes
  - active versus disabled release channels
  - package inventory parity between release records
  - `pugRelease` or Cargo metadata alignment with the release manifest
- refreshed related baseline docs in:
  - `docs/specs/021-public-package-api-stability-and-parity-debt-baseline.md`
  - `docs/specs/022-packaging-versioning-and-release-channel-rules.md`
  - `docs/specs/README.md`
  - `docs/roadmaps/g03/README.md`
  - `docs/roadmaps/README.md`
  - `docs/README.md`
  - `README.md`

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Outcome

`g03.011` is now explicit. Pug has a named release-operations baseline, an
honest statement that no stable channel exists yet, and machine-checked
alignment between repo-wide release classification and per-package release
metadata.

## Next

Move to `g03.012` and define ecosystem acceptance plus long-tail regression
coverage using the now-explicit release operations baseline as part of the
shared hardening posture.
