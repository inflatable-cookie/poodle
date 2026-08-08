---
title: g04.013 shared demo app contract
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, planning, svelte, gpui, demo]
---

## Summary

Completed `g04.013` by turning the frozen shared demo target into one explicit
cross-runtime demo-app contract for both Svelte and GPUI.

## What changed

- added the normative baseline `docs/specs/060-shared-demo-app-contract-section-model-and-parity-checklist.md`
- completed `docs/roadmaps/g04/013-cross-runtime-demo-app-contract-section-model-and-parity-checklist.md`
- added the machine-readable contract artifact `packages/shared-demo-app-contract.json`
- froze the docs-shell boundary so `catalog-hub`, `token-summary-section`, and
  `token-inspector` remain docs-only and do not leak into the shared demo-app
  contract
- defined the shared demo shell region model explicitly:
  - `app-header`
  - `screen-tabs`
  - `context-toolbar`
  - `primary-content`
  - `companion-panel`
  - `status-bar`
  - `modal-layer`
- turned the frozen target shape into six explicit screen contracts with source
  section mapping, region ownership, component expectations, state matrices,
  interaction checkpoints, and comparison modes
- attached the demo contract to both runtimes without turning runtime mechanics
  into the contract itself:
  - Svelte rebuild bound to `g04.014`
  - GPUI demo implementation bound to `g04.015`
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the demo contract
  is machine-checked against the demo audit and current docs-section inventory
- rolled the status surfaces forward so the repo now points at `g04.014`

## Validation

- `bun run --cwd packages/svelte/preview docs:lint`
- `git diff --check`

## Outcome

`g04.013` is now explicit. The repo no longer has only a target-shape sketch;
it now has one machine-readable shared demo-app contract that both the rebuilt
Svelte demo and later GPUI demo must implement.

## Next

Open `g04.014` and rebuild the Svelte demo app against the explicit screen
model, shell regions, state matrix, and parity checklist.
