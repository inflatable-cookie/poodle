# g10.012 — Visual Parity Audit: Svelte/GPUI vs Jetstream Comparison

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.011
Primary repos: `pug`

## Goals

- [ ] systematically compare Jetstream specimens against Svelte and GPUI
  references
- [ ] fix style mapping issues and document native adaptation constraints

## Execution Checklist

- [ ] capture reference screenshots from Svelte preview for each component
  with default theme/density
- [ ] capture reference screenshots from GPUI preview for each component
- [ ] capture Jetstream preview screenshots for each component
- [ ] compare structural primitives: background fill, border, padding, gap
- [ ] compare action primitives: button variant colors, disabled opacity,
  sizing
- [ ] compare input primitives: input border, focus state, placeholder color
- [ ] compare selection primitives: indicator sizes, track colors, fill colors
- [ ] compare feedback primitives: progress bar colors, badge backgrounds
- [ ] compare overlay primitives: dialog backdrop, border radius, shadow
  (single shadow limitation)
- [ ] compare composites: layout proportions, spacing, content arrangement
- [ ] for each discrepancy:
  - [ ] classify as fixable (token/style mapping error) or constraint-based
    (Jetstream limitation)
  - [ ] fix style mapping errors in `pug-jetstream` adapter
  - [ ] document constraint-based deltas
- [ ] re-compare after fixes to verify resolution

## Acceptance Criteria

- [ ] every component has been compared across all three runtimes
- [ ] all fixable discrepancies have been resolved
- [ ] constraint-based deltas are documented with Jetstream limitation
  reference
- [ ] token coverage is complete — no unresolved token references

## Next Task

Open `g10.013` and update the delta register.
