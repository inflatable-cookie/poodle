# g04.010 Loading, Skeleton Presets, And Progress Patterns

Status: planned
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `poodle`

## Goals

- [ ] extend Skeleton with data-shape presets for common layouts (table rows,
  card grids, list items, detail views)
- [ ] implement PageLoading as a composite for full-page loading states with
  progress and messaging

## Execution Checklist

- [ ] amend Skeleton contract: add preset prop with values like `table-row`,
  `card`, `list-item`, `detail-section`, `avatar-line`
- [ ] implement Skeleton presets in `@poodle/svelte`
- [ ] write contract for PageLoading: full-viewport overlay, spinner or progress
  bar, status message, cancel option
- [ ] implement PageLoading composite in `@poodle/svelte`
- [ ] update Skeleton specimen with preset examples
- [ ] create PageLoading specimen
- [ ] register PageLoading in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] Skeleton presets render realistic placeholder shapes matching their target
  layout
- [ ] Skeleton presets animate with the standard shimmer effect
- [ ] PageLoading renders a centered loading state over the full viewport
- [ ] PageLoading supports progress bar and status message updates
- [ ] both components pass build and render in the preview catalogue

## Next Task

Open `g04.011` and implement DataTable and Select depth enhancements.
