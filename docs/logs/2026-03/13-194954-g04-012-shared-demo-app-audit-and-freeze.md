---
title: g04.012 shared demo app audit and freeze
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, planning, svelte, gpui, demo]
---

## Summary

Completed `g04.012` by auditing the current Svelte demo or preview surface and
freezing the target shape for the rebuilt shared demo app before GPUI demo
parity and downstream proof continue.

## What changed

- added the normative baseline `docs/specs/059-shared-demo-app-audit-and-target-freeze-baseline.md`
- completed `docs/roadmaps/g04/012-shared-demo-app-audit-gap-register-and-target-shape-freeze.md`
- added the machine-readable audit artifact `packages/shared-demo-app-audit.json`
- froze the current source-surface posture explicitly:
  - `packages/svelte/preview/src/App.svelte` is still a 3328-line monolith
  - `packages/svelte/preview/src/app.css` is still a 2085-line global stylesheet
  - docs-only sections such as `catalog-hub`, `token-summary-section`, and
    `token-inspector` are useful preview infrastructure but not the main shared
    demo target GPUI should reproduce
- made the current coverage debt explicit from the generated parity report:
  - `@pug/svelte-primitives`: `14/63` previewed
  - `@pug/svelte-composites`: `18/20` previewed
  - `@pug/svelte-workstation`: `12/14` previewed
- froze the main audit findings: docs-shell leakage, monolithic entry shape,
  broad but shallow coverage, missing coherent app story, and excessive
  preview-glue ownership
- froze the first target shape for the rebuilt shared demo as six coherent
  screen families: shell or overview, form or validation, browse or table,
  detail and related data, picker or media, and command or workspace
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new shared
  demo-app audit is machine-checked against the parity artifact and current
  docs-section inventory
- rolled the status surfaces forward so the repo now points at `g04.013`

## Validation

- `bun run --cwd packages/svelte/preview docs:lint`
- `git diff --check`

## Outcome

`g04.012` is now explicit. The repo no longer treats the current preview page
as the GPUI target by default. It now records why that surface is too mixed and
shallow, and it freezes the target shape the rebuilt Svelte demo and later GPUI
demo must both satisfy.

## Next

Open `g04.013` and write the explicit cross-runtime demo-app contract, section
model, and parity checklist for both Svelte and GPUI.
