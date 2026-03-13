# g04.013 Cross-Runtime Demo-App Contract, Section Model, And Parity Checklist

Status: completed
Owner: Pug Core
Updated: 2026-03-13
Depends on: g04.011, g04.012
Primary repos: `pug`

## Goals

- [x] define the demo app as a contract-owned shared target rather than a loose
  implementation detail
- [x] write the demo contracts on both sides so Svelte and GPUI have the same
  screen model, state model, and parity checklist

## Execution Checklist

- [x] define the demo-app shell, screen list, region model, and section
  inventory explicitly
- [x] define the state matrix, interaction checkpoints, and side-by-side review
  checklist for each major demo screen
- [x] identify which parts of the current preview remain docs-shell only and
  must not leak into the demo contract
- [x] define how demo-app contracts attach to Svelte and GPUI without turning
  Bits or GPUI mechanics into the contract itself
- [x] avoid leaving demo parity criteria implicit in milestone prose or review
  screenshots

## Acceptance Criteria

- [x] cross-runtime demo-app contract is explicit
- [x] demo screen or section model is explicit
- [x] parity checklist for the demo target is explicit

## Completed Work

- added the normative baseline `docs/specs/060-shared-demo-app-contract-section-model-and-parity-checklist.md`
- added the machine-readable contract artifact `packages/shared-demo-app-contract.json`
- froze the docs-shell boundary so `catalog-hub`, `token-summary-section`, and
  `token-inspector` remain docs-only and do not leak into the shared demo app
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
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new demo
  contract is machine-checked against the demo audit and current docs-section
  inventory
- rolled the status surfaces forward so the repo now points at `g04.014`

## Next Task

Open `g04.014` and rebuild the Svelte demo app against the explicit demo
contracts and coverage target.
