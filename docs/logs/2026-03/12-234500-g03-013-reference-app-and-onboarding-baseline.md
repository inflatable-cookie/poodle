---
title: g03.013 reference-app and onboarding baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, adoption, onboarding]
---

## Summary

Completed `g03.013` by freezing explicit reference-app shapes, onboarding
lanes, and public-example expectations, then wiring lint coverage so those
adoption-facing surfaces stay aligned with real repo evidence.

## What changed

- added the normative spec `docs/specs/046-reference-apps-onboarding-and-public-example-baseline.md`
- completed `docs/roadmaps/g03/013-reference-apps-onboarding-depth-and-public-examples.md`
- added the machine-readable onboarding matrix `packages/reference-apps.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate:
  - required reference shapes
  - required onboarding lanes
  - evidence path existence
  - example section coverage
  - package references against the release manifest
- refreshed the main README surfaces for:
  - `packages/svelte/preview/README.md`
  - `packages/svelte/components/README.md`
  - `packages/svelte/components/README.md`
  - `packages/svelte/workstation/README.md`
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

`g03.013` is now explicit. Pug has named reference-app shapes for the major
adoption lanes, explicit onboarding starting points, and a clearer statement
that preview examples support evaluation and discoverability without becoming
hidden starter templates.

## Next

Move to `g03.014` and close the current program deliberately using the now-explicit
reference-app, ecosystem acceptance, release, accessibility, and adoption
baselines as the summary frame.
