---
title: g04.014 full primitive demo coverage
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, svelte, demo, parity, primitives]
---

## Summary

Extended the rebuilt Svelte shared demo so it now directly previews the full
public primitive surface instead of stopping at a stronger-but-still-partial
coverage posture.

## What changed

- added `packages/svelte/preview/src/components/PrimitiveCoverageDeck.svelte`
  as a dedicated long-tail primitive review surface inside the shared demo shell
- mounted that deck inside
  `packages/svelte/preview/src/components/SharedDemoApp.svelte` so the shared
  demo now remains one coherent app while still exposing every primitive export
- upgraded `packages/svelte/preview/src/parity.ts` so the parity registry now
  treats all `@pug/svelte-primitives` exports as directly previewed
- rolled `packages/shared-demo-app-audit.json` forward to `63/63` primitive
  coverage and removed remaining priority-missing primitive exports
- tightened `packages/shared-demo-app-contract.json` so the parity checklist now
  explicitly requires every public primitive export to be directly reviewable
  somewhere inside the shared demo shell
- updated the normative docs in:
  - `docs/specs/059-shared-demo-app-audit-and-target-freeze-baseline.md`
  - `docs/specs/060-shared-demo-app-contract-section-model-and-parity-checklist.md`
  - `docs/specs/061-svelte-demo-app-rebuild-and-coverage-upgrade-baseline.md`
- updated `docs/roadmaps/g04/014-svelte-demo-app-rebuild-component-adoption-and-coverage-upgrade.md`
  so the completed milestone now explicitly includes full primitive coverage

## Validation

- `bun run --cwd packages/svelte/preview parity:report`
- `bun run --cwd packages/svelte/preview build`
- `effigy docs:check`
- `git diff --check`

## Outcome

The rebuilt Svelte shared demo is now a full primitive comparison target rather
than a mostly coherent workflow shell with contract-only long-tail controls.
The parity artifact now records:

- `@pug/svelte-primitives`: `63/63`
- `@pug/svelte-composites`: `20/20`
- `@pug/svelte-workstation`: `14/14`

## Next

Open `g04.015` and implement the same shared demo app in GPUI against the now
complete Svelte primitive surface, using side-by-side review to catch real
runtime deltas rather than simple coverage gaps.
