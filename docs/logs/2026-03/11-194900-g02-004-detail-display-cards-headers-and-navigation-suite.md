# g02.004 Detail Display, Cards, Headers, And Navigation Suite

Status: completed
Date: 2026-03-11
Owner: Poodle Core

## Summary

- completed `g02.004`
- added Svelte detail/display composites at
  `packages/svelte/composites/src/Card.svelte`,
  `packages/svelte/composites/src/PageHeader.svelte`,
  `packages/svelte/composites/src/Breadcrumbs.svelte`,
  `packages/svelte/composites/src/DetailShell.svelte`,
  `packages/svelte/composites/src/DetailSection.svelte`, and
  `packages/svelte/composites/src/DetailItem.svelte`
- extended the preview with a real detail surface that composes breadcrumbs,
  local page identity, summary cards, readonly detail rows, and section
  hierarchy together
- added the normative detail-display baseline at
  `docs/specs/012-detail-display-card-header-and-navigation-rules.md`
- deepened the existing card, header, breadcrumb, detail-shell, detail-section,
  and detail-item contracts so summary/detail composition is explicit rather than
  implied

## Validation

- `bun install`
- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche keeps local page identity separate from workstation shell chrome
- cards and detail rows are now explicitly documented as complementary, not
  interchangeable

## Next Task

Open `docs/roadmaps/g02/005-picker-relation-and-selection-workflow-suite.md`
and build the next meaningful selection and relation batch above the completed
detail baseline.
