# g02.001 Forms, Validation, And Svelte Primitives Baseline

Status: completed
Date: 2026-03-11
Owner: Pug Core

## Summary

- completed `g02.001`
- added the first form-system contracts at `docs/contracts/foundation/field.md`
  and `docs/contracts/foundation/form-actions.md`
- added the normative form baseline at
  `docs/specs/009-form-shell-validation-and-action-row-rules.md`
- created the first implementation-bearing Svelte primitives package at
  `packages/svelte/primitives`
- replaced the preview's raw native form probes with contract-backed `Field`,
  `TextInput`, `SearchField`, and `FormActions` components

## Validation

- `bun install`
- `bun run tokens:build`
- `bun run preview:build`
- `git diff --check`

## Notes

- the new Svelte package is intentionally narrow and only proves the `g02.001`
  baseline rather than pretending the full primitive catalogue is implemented
- pending validation semantics and action-row structure are now explicit in both
  docs and preview code

## Next Task

Open `docs/roadmaps/g02/002-data-table-and-bulk-action-suite.md` and build the
next meaningful batch above the completed form baseline.
