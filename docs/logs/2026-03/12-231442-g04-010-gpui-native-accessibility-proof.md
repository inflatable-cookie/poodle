---
title: g04.010 gpui native accessibility proof
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, accessibility, docs]
---

## Summary

Completed `g04.010` by freezing the first honest GPUI native accessibility,
focus, keyboard, and assistive-technology proof posture across the widened
primitive, composite, and workstation surface.

## What changed

- added the normative baseline `docs/specs/057-gpui-native-accessibility-focus-keyboard-and-assistive-technology-proof-baseline.md`
- completed `docs/roadmaps/g04/010-gpui-native-accessibility-focus-keyboard-and-assistive-technology-proof.md`
- added the machine-readable artifact `packages/gpui/native-accessibility-proof.json`
- updated `packages/svelte/preview/src/accessibility.ts` so shared GPUI review
  sections now carry explicit `manual` GPUI proof posture instead of defaulting
  to `blocked`
- updated `packages/svelte/preview/scripts/build-accessibility-report.ts` and
  regenerated `packages/svelte/preview/artifacts/accessibility-report.json`
  so the browser accessibility audit reflects the new GPUI proof posture
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the GPUI
  accessibility artifact is checked against the widened primitive, composite,
  and workstation baselines plus the section-level accessibility audit source
- updated `packages/gpui/primitives/README.md`,
  `packages/gpui/composites/README.md`,
  `packages/gpui/workstation/README.md`, and
  `packages/gpui/tokens/README.md` so the repo no longer implies GPUI
  accessibility proof is still entirely future work
- rolled the roadmap and status surfaces forward so the repo now points at
  `g04.011`

## Validation

- `bun run --cwd packages/svelte/preview accessibility:report`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.010` is now explicit. Pug no longer treats the widened GPUI surface as
either inaccessible by default or fully proven by browser analogy. The repo now
states, section by section and layer by layer, where GPUI naming, focus,
keyboard, state exposure, and announcement posture are explicit, and where
mounted native assistive-technology proof is still manual work.

## Next

Open `g04.011` and harden the cross-runtime parity report, intentional delta
register, and acceptance-harness expansion on top of the explicit GPUI
primitive, composite, workstation, and accessibility-proof baselines.
